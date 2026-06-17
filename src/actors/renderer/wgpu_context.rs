use anyhow::{Context, Result, bail};
use diretto::{ClientCapability, Device as DrmDevice};
use rustix::fd::{AsFd, AsRawFd};
use tracing::debug;
use wgpu::{
    Backends, CurrentSurfaceTexture, ExperimentalFeatures, PresentMode, SurfaceTargetUnsafe,
};

use crate::actors::{output_manager::OutputInfo, session::DeviceBroker};

struct DrmState {
    // Kept alive to ensure the DRM fd remains open for the duration of the WGPU surface.
    _device: DrmDevice,
}

pub struct WgpuContext<'s> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'s>,
    _drm_state: DrmState,
}

impl<'s> WgpuContext<'s> {
    pub async fn new(device_broker: &DeviceBroker, output: &OutputInfo) -> Result<Self> {
        let fd: std::os::fd::OwnedFd = device_broker
            .open(output.device_path.clone())
            .await
            .map_err(|_| anyhow::anyhow!("Session actor is dead"))?;

        let drm_device = unsafe { DrmDevice::new_unchecked(fd) };

        drm_device.set_client_capability(ClientCapability::Atomic, true)?;

        let surface_target = SurfaceTargetUnsafe::Drm {
            fd: drm_device.as_fd().as_raw_fd(),
            plane: output.plane_id,
            connector_id: output.connector_id,
            width: output.width,
            height: output.height,
            refresh_rate: output.refresh_rate,
        };

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::VULKAN,
            flags: wgpu::InstanceFlags::default()
                | wgpu::InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER,
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = unsafe { instance.create_surface_unsafe(surface_target)? };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                ..Default::default()
            })
            .await
            .context("Failed to find an appropriate adapter")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: adapter.limits(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
                experimental_features: ExperimentalFeatures::disabled(),
            })
            .await
            .context("Failed to create device")?;

        let mut config = surface
            .get_default_config(&adapter, output.width, output.height)
            .context("Surface not supported by adapter")?;

        config.present_mode = PresentMode::AutoVsync;
        surface.configure(&device, &config);

        debug!("Created WGPU resources");

        Ok(Self {
            surface,
            device,
            queue,
            _drm_state: DrmState {
                _device: drm_device,
            },
        })
    }

    pub fn present(&self) -> anyhow::Result<()> {
        let frame = self.surface.get_current_texture();

        let (CurrentSurfaceTexture::Success(frame) | CurrentSurfaceTexture::Suboptimal(frame)) =
            frame
        else {
            bail!("Failed to acquire next swapchain texture")
        };

        let texture_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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
            ..Default::default()
        });

        drop(renderpass);
        self.queue.submit([encoder.finish()]);
        frame.present();

        Ok(())
    }
}
