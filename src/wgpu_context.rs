use anyhow::{Context, Result};
use diretto::{
    ClientCapability, Connector, Device as DrmDevice, ModeType, sys::DRM_MODE_OBJECT_PLANE,
};
use rustix::{
    fd::{AsFd, AsRawFd},
    fs::{Mode, OFlags, open},
};
use tracing::{debug, trace};
use wgpu::{Backends, PresentMode, SurfaceTargetUnsafe};

#[derive(Debug)]
struct DrmState {
    device: DrmDevice,
    connector: Connector,
    mode: diretto::Mode,
    plane_id: u32,
}

#[derive(Debug)]
struct WgpuState<'s> {
    surface: wgpu::Surface<'s>,
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub struct WgpuContext<'s> {
    wgpu_state: WgpuState<'s>,
    _drm_state: DrmState,
}

fn open_drm_device() -> Result<DrmDevice> {
    let fd = open(
        "/dev/dri/card1",
        OFlags::RDWR | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )?;
    let device = unsafe { DrmDevice::new_unchecked(fd) };

    debug!("Opened DRM device /dev/dri/card1");
    Ok(device)
}

impl<'s> WgpuContext<'s> {
    pub async fn new() -> Result<Self> {
        let drm_state = Self::create_drm_resources()?;
        let wgpu_state = Self::create_wgpu_resources(&drm_state).await?;

        Ok(Self {
            _drm_state: drm_state,
            wgpu_state,
        })
    }

    fn create_drm_resources() -> Result<DrmState> {
        let device = open_drm_device()?;
        device.set_client_capability(ClientCapability::Atomic, true)?;
        device.set_master().context("Failed to become DRM master")?;

        let resources = device.get_resources()?;

        // Find connected connector
        let connector = {
            let mut found_connector = None;
            for connector_id in &resources.connectors {
                let connector = device.get_connector(*connector_id, false)?;
                if connector.connection.is_connected() {
                    found_connector = Some(connector);
                    break;
                }
            }
            found_connector.ok_or_else(|| anyhow::anyhow!("No connected display found"))?
        };

        // Find best mode
        let mode = {
            let mut best_mode = None;
            let mut max_area = 0;

            for current_mode in connector.modes.iter().copied() {
                if current_mode.ty().contains(ModeType::DEFAULT) {
                    best_mode = Some(current_mode);
                    break;
                }

                let area =
                    current_mode.display_width() as u32 * current_mode.display_height() as u32;
                if area > max_area {
                    best_mode = Some(current_mode);
                    max_area = area;
                }
            }
            best_mode.ok_or_else(|| anyhow::anyhow!("No suitable mode found"))?
        };

        debug!(
            "Selected mode {}x{}@{}",
            mode.display_width(),
            mode.display_height(),
            mode.vertical_refresh_rate()
        );

        // Find primary plane
        let plane_id = {
            let plane_resources = device.get_plane_resources()?;
            let mut primary_plane = None;

            for id in plane_resources {
                let (props, values) = unsafe { device.get_properties(id, DRM_MODE_OBJECT_PLANE)? };

                for (index, prop) in props.into_iter().enumerate() {
                    let (name, _) = unsafe { device.get_property(prop)? };
                    let current_value = values[index];

                    if name.as_c_str() == c"type" && current_value == 1 {
                        trace!("Found primary plane: {}", id);
                        primary_plane = Some(id);
                        break;
                    }
                }

                if primary_plane.is_some() {
                    break;
                }
            }
            primary_plane.ok_or_else(|| anyhow::anyhow!("No primary plane found"))?
        };

        Ok(DrmState {
            device,
            connector,
            mode,
            plane_id,
        })
    }

    async fn create_wgpu_resources<'a, 'b>(drm_state: &'b DrmState) -> Result<WgpuState<'a>> {
        let surface_target = SurfaceTargetUnsafe::Drm {
            fd: drm_state.device.as_fd().as_raw_fd(),
            plane: drm_state.plane_id,
            connector_id: drm_state.connector.connector_id.into(),
            width: drm_state.mode.display_width() as u32,
            height: drm_state.mode.display_height() as u32,
            refresh_rate: drm_state.mode.vertical_refresh_rate() * 1000,
        };

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: Backends::VULKAN,
            flags: wgpu::InstanceFlags::default()
                | wgpu::InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                ..Default::default()
            })
            .await
            .context("Failed to find an appropriate adapter")?;

        let surface = unsafe { instance.create_surface_unsafe(surface_target)? };

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: adapter.limits(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await
            .context("Failed to create device")?;

        let mut config = surface
            .get_default_config(
                &adapter,
                drm_state.mode.display_width().into(),
                drm_state.mode.display_height().into(),
            )
            .context("Surface not supported by adapter")?;

        config.present_mode = PresentMode::AutoVsync;
        surface.configure(&device, &config);

        debug!("Created WGPU resources");
        Ok(WgpuState {
            surface,
            _instance: instance,
            _adapter: adapter,
            device,
            queue,
        })
    }

    pub fn present(&self) -> Result<()> {
        let wgpu_state = &self.wgpu_state;

        let frame = wgpu_state
            .surface
            .get_current_texture()
            .context("Failed to acquire next swapchain texture")?;

        let texture_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        drop(renderpass);
        wgpu_state.queue.submit([encoder.finish()]);
        frame.present();

        Ok(())
    }
}
