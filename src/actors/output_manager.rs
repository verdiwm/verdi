use std::{ffi::CString, os::fd::AsFd};

use diretto::{ClientCapability, Device as DrmDevice, ModeType, sys::DRM_MODE_OBJECT_PLANE};
use stagecraft::{
    Actor, Bus, Context, HasMailbox, Responder, SuspendRefused, Suspendable, WatchReader,
};
use tracing::{debug, trace};

use crate::actors::session::{DeviceBroker, SessionState};

/// Parameters describing a connected output, discovered by [`OutputManager`].
///
/// Sent via [`OutputEvent::Added`] so that the Renderer can open the device
/// and create a WGPU surface without holding a long-lived DRM fd.
#[derive(Clone)]
pub struct OutputInfo {
    /// Path to the DRM device node.
    pub device_path: CString,
    /// DRM connector ID (passed to `SurfaceTargetUnsafe::Drm`).
    pub connector_id: u32,
    /// DRM primary plane ID (passed to `SurfaceTargetUnsafe::Drm`).
    pub plane_id: u32,
    /// Horizontal resolution in pixels.
    pub width: u32,
    /// Vertical resolution in pixels.
    pub height: u32,
    /// Refresh rate in mHz (as expected by wgpu's `SurfaceTargetUnsafe::Drm`).
    pub refresh_rate: u32,
}

/// Events published by [`OutputManager`] on the output bus.
#[derive(Clone)]
pub enum OutputEvent {
    /// A new output is available. Subscribe and call [`Renderer`] to create a surface.
    Added(OutputInfo),
    /// The output has been removed.
    Removed,
}

#[stagecraft::message(OutputManager)]
pub enum OutputManagerMessage {
    /// Suspend coordination: the OutputManager holds no hardware resources, so it
    /// just acknowledges.
    #[call(Result<(), SuspendRefused>)]
    PrepareSuspend,
    /// Suspend coordination: no-op; rescans are driven off the session-state watch.
    Resume,
    /// The session became active; rescan outputs and notify the Renderer.
    Rescan,
}

impl Suspendable for OutputManager {
    fn prepare_suspend(responder: Responder<Result<(), SuspendRefused>>) -> Self::Message {
        OutputManagerMessage::PrepareSuspend {
            respond_to: responder,
        }
    }

    fn resume() -> Self::Message {
        OutputManagerMessage::Resume
    }
}

pub struct OutputManagerInit {
    pub device_broker: DeviceBroker,
    pub state: WatchReader<SessionState>,
    pub output_bus: Bus<OutputEvent>,
}

pub struct OutputManager {
    device_broker: DeviceBroker,
    output_bus: Bus<OutputEvent>,
}

impl HasMailbox for OutputManager {
    type Message = OutputManagerMessage;
}

impl Actor for OutputManager {
    type Init = OutputManagerInit;

    async fn init(init: OutputManagerInit, ctx: &mut Context<Self>) -> Self {
        // Rescan whenever the session becomes active (startup and every resume).
        // The watch delivers the current state immediately, so a late subscribe
        // still triggers the initial scan.
        ctx.forward_watch(init.state, |state| {
            (state == SessionState::Active).then_some(OutputManagerMessage::Rescan)
        });

        Self {
            device_broker: init.device_broker,
            output_bus: init.output_bus,
        }
    }

    async fn handle_message(&mut self, msg: OutputManagerMessage, _ctx: &mut Context<Self>) {
        match msg {
            OutputManagerMessage::PrepareSuspend { respond_to } => {
                // No persistent hardware resources to release.
                respond_to.respond(Ok(()));
            }
            OutputManagerMessage::Resume => {}
            OutputManagerMessage::Rescan => match Self::scan_outputs(&self.device_broker).await {
                Ok(info) => {
                    self.output_bus.publish(OutputEvent::Added(info));
                }
                Err(e) => {
                    tracing::error!("OutputManager: failed to scan outputs: {e}");
                }
            },
        }
    }
}

impl OutputManager {
    /// Discover the first DRM device with a connected display and return the
    /// parameters needed to create a WGPU surface. Each candidate device is
    /// closed immediately after scanning (the Renderer will reopen the chosen
    /// device to create the surface).
    async fn scan_outputs(device_broker: &DeviceBroker) -> anyhow::Result<OutputInfo> {
        let mut card_nodes = Self::enumerate_card_nodes()?;
        card_nodes.sort();

        for device_path in card_nodes {
            match Self::scan_device(device_broker, &device_path).await {
                Ok(Some(info)) => return Ok(info),
                Ok(None) => continue,
                Err(e) => {
                    tracing::warn!(
                        "OutputManager: skipping {}: {e}",
                        device_path.to_string_lossy()
                    );
                    continue;
                }
            }
        }

        anyhow::bail!("No DRM device with a connected display found")
    }

    /// List the DRM card nodes under `/dev/dri` (e.g. `card0`, `card1`).
    fn enumerate_card_nodes() -> anyhow::Result<Vec<CString>> {
        use std::os::unix::ffi::OsStrExt;

        let mut nodes = Vec::new();
        for entry in std::fs::read_dir("/dev/dri")? {
            let entry = entry?;
            if entry.file_name().as_bytes().starts_with(b"card") {
                nodes.push(CString::new(entry.path().as_os_str().as_bytes())?);
            }
        }
        Ok(nodes)
    }

    /// Open a single DRM device and probe it. Returns `Ok(Some(info))` if it has
    /// a connected display, `Ok(None)` if it does not, and `Err` on a DRM error.
    /// The device fd is always closed before returning.
    async fn scan_device(
        device_broker: &DeviceBroker,
        device_path: &CString,
    ) -> anyhow::Result<Option<OutputInfo>> {
        let fd: std::os::fd::OwnedFd = device_broker
            .open(device_path.clone())
            .await
            .map_err(|_| anyhow::anyhow!("Session actor is dead"))?;

        let fd_for_close = fd.as_fd().try_clone_to_owned()?;
        let device = unsafe { DrmDevice::new_unchecked(fd) };

        let result = Self::probe_device(&device, device_path);

        drop(device); // closes the original fd

        // Notify saddle/logind that we no longer need the device.
        let _ = device_broker.close(fd_for_close).await;

        result
    }

    /// Discover the best connector/mode/plane on an already-open device.
    /// Returns `Ok(None)` when the device has no connected connector.
    fn probe_device(
        device: &DrmDevice,
        device_path: &CString,
    ) -> anyhow::Result<Option<OutputInfo>> {
        device.set_client_capability(ClientCapability::Atomic, true)?;

        let resources = device.get_resources()?;

        // Find a connected connector.
        let connector = {
            let mut found = None;
            for connector_id in &resources.connectors {
                let c = device.get_connector(*connector_id, false)?;
                if c.connection.is_connected() {
                    found = Some(c);
                    break;
                }
            }
            match found {
                Some(c) => c,
                None => return Ok(None),
            }
        };

        // Find the best mode (prefer DEFAULT flag, otherwise largest area).
        let mode = {
            let mut best = None;
            let mut max_area = 0u32;
            for m in connector.modes.iter().copied() {
                if m.ty().contains(ModeType::DEFAULT) {
                    best = Some(m);
                    break;
                }
                let area = m.display_width() as u32 * m.display_height() as u32;
                if area > max_area {
                    best = Some(m);
                    max_area = area;
                }
            }
            best.ok_or_else(|| anyhow::anyhow!("No suitable mode found"))?
        };

        debug!(
            "OutputManager: selected {} mode {}x{}@{}",
            device_path.to_string_lossy(),
            mode.display_width(),
            mode.display_height(),
            mode.vertical_refresh_rate()
        );

        // Find the primary plane.
        let plane_id = {
            let plane_resources = device.get_plane_resources()?;
            let mut primary = None;
            for id in plane_resources {
                let (props, values) = unsafe { device.get_properties(id, DRM_MODE_OBJECT_PLANE)? };
                for (index, prop) in props.into_iter().enumerate() {
                    let (name, _) = unsafe { device.get_property(prop)? };
                    if name.as_c_str() == c"type" && values[index] == 1 {
                        trace!("OutputManager: found primary plane: {id}");
                        primary = Some(id);
                        break;
                    }
                }
                if primary.is_some() {
                    break;
                }
            }
            primary.ok_or_else(|| anyhow::anyhow!("No primary plane found"))?
        };

        Ok(Some(OutputInfo {
            device_path: device_path.clone(),
            connector_id: connector.connector_id.into(),
            plane_id,
            width: mode.display_width() as u32,
            height: mode.display_height() as u32,
            refresh_rate: mode.wsi_refresh_rate(),
        }))
    }
}
