use std::io;

use anyhow::{Context, Result};
use diretto::{Connector, Device as DrmDevice};
use raw_window_handle::{DisplayHandle, DrmDisplayHandle, DrmWindowHandle, WindowHandle};
use rustix::{
    fd::{AsFd, AsRawFd},
    fs::{self, Mode, OFlags},
};
use tracing::{debug, info};
use wgpu::{Backends, InstanceDescriptor, PresentMode, SurfaceTargetUnsafe};

pub struct WgpuContext<'s> {
    pub surface: wgpu::Surface<'s>,
    pub _instance: wgpu::Instance,
    pub _adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub _drm_device: DrmDevice,
}

impl<'s> WgpuContext<'s> {
    pub async fn new() -> Result<Self> {
        // Open the dri device
        // FIXME: should employ a strategy to choose a correct one
        let fd = fs::open(
            "/dev/dri/card0",
            OFlags::RDWR | OFlags::NONBLOCK,
            Mode::empty(),
        )?;

        // SAFETY: we assume the path is not lying and the device is actually drm capable
        let drm_device = unsafe { DrmDevice::new_unchecked(fd) };

        info!("Opened device /dev/dri/card0");

        let version = drm_device.version()?;

        debug!(
            "Driver: {} ({}) version {}.{}.{} ({})",
            version.name.to_string_lossy(),
            version.desc.to_string_lossy(),
            version.major,
            version.minor,
            version.patchlevel,
            version.date.to_string_lossy()
        );

        let res = drm_device.get_resources()?;

        // Collect available connectors so we don't iterate again later
        let connectors = res
            .connectors
            .iter()
            .map(|id| drm_device.get_connector(*id, true))
            .collect::<io::Result<Vec<Connector>>>()?;

        for connector in &connectors {
            debug!("Found connector {}", connector.connector_id);

            for mode in &connector.modes {
                debug!(
                    "Found mode {}@{} for connector {}",
                    mode.name().to_string_lossy(),
                    mode.vertical_refresh_rate(),
                    connector.connector_id
                )
            }
        }

        // Find the first connected monitor
        // FIXME: support more monitors
        let connector = connectors
            .into_iter()
            .find(|connector| connector.connection == 1) // 1 means connected
            .unwrap();

        // FIXME: The first mode is usually the prefered one but we should employ a better strategy
        let mode = connector.modes.first().expect("Connector has no modes");

        // This should somehow be passed to wgpu to choose the correct mode
        dbg!(mode.wsi_refresh_rate());

        let planes = drm_device.get_plane_resources()?;

        // FIXME: use a proper strategy to determine the best plane
        let plane = planes[0];

        let display_handle = unsafe {
            DisplayHandle::borrow_raw({
                let handle = DrmDisplayHandle::new(drm_device.as_fd().as_raw_fd());
                handle.into()
            })
        };

        let window_handle = unsafe {
            WindowHandle::borrow_raw({
                let handle =
                    DrmWindowHandle::new_with_connector_id(plane, connector.connector_id.into());
                handle.into()
            })
        };

        // FIXME: add drm + opengl support to wgpu
        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None, // We don't pass a compatible surface because we need to create it later to associate the mode
            })
            .await
            .context("Failed to find an appropriate adapter")?;

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .context("Failed to create device")?;

        let surface_target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: display_handle.as_raw(),
            raw_window_handle: window_handle.as_raw(),
        };

        // Safety: the drm_device handle is valid until the end
        let surface = unsafe { instance.create_surface_unsafe(surface_target)? };

        // Get a config according to the mode
        // FIXME: allow dynamic config
        let mut config = surface
            .get_default_config(
                &adapter,
                mode.display_width().into(),
                mode.display_height().into(),
            )
            .expect("Surface not supported by adapter");

        config.present_mode = PresentMode::AutoVsync;

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            _instance: instance,
            _adapter: adapter,
            device,
            queue,
            config,
            _drm_device: drm_device,
        })
    }
}
