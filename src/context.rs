use std::io;

use anyhow::{Context, Result, bail};
use diretto::{
    ClientCapability, Connector, Device as DrmDevice, ModeType, sys::DRM_MODE_OBJECT_PLANE,
};
use rustix::{
    fd::{AsFd, AsRawFd},
    fs::{self, Mode, OFlags},
};
use tracing::{debug, info, trace};
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

fn find_drm_device() -> Result<DrmDevice> {
    // TODO: implement an actual strategy
    let fd = fs::open(
        "/dev/dri/card1",
        OFlags::RDWR | OFlags::NONBLOCK,
        Mode::empty(),
    )?;
    let device = unsafe { DrmDevice::new_unchecked(fd) };

    debug!("Opened device /dev/dri/card1");

    Ok(device)
}

fn find_drm_connector(device: &DrmDevice, resources: &diretto::Resources) -> Result<Connector> {
    for connector_id in &resources.connectors {
        let connector = device.get_connector(*connector_id, false)?;
        if connector.connection.is_connected() {
            return Ok(connector);
        }
    }

    bail!("No connected display found")
}

impl WgpuContext<'_> {
    pub async fn new() -> Result<Self> {
        let drm_device = find_drm_device()?;
        let resources = drm_device.get_resources()?;
        let connector = find_drm_connector(&drm_device, &resources)?;

        let mode = {
            let mut mode = None;

            let mut area = 0;

            for current_mode in connector.modes {
                if current_mode.ty().contains(ModeType::PREFERRED) {
                    mode = Some(current_mode);
                    break;
                }

                let current_area = current_mode.display_width() * current_mode.display_height();
                if current_area > area {
                    mode = Some(current_mode);
                    area = current_area;
                }
            }

            mode.expect("Couldn't find a mode")
        };

        debug!(
            "Selected mode {}x{}@{}",
            mode.display_width(),
            mode.display_height(),
            mode.vertical_refresh_rate()
        );

        drm_device.set_client_capability(ClientCapability::Atomic, true)?;

        let plane_resources = drm_device.get_plane_resources()?;

        let mut plane = None;

        for id in plane_resources {
            debug!("Found plane {id}");
            let (props, values) = unsafe { drm_device.get_properties(id, DRM_MODE_OBJECT_PLANE)? };

            trace!("Properties for plane {id}:");
            for (index, prop) in props.into_iter().enumerate() {
                let (name, possible_values) = unsafe { drm_device.get_property(prop)? };
                let current_value = values[index];

                trace!(
                    "  Property '{}' = {} (possible values: {:?})",
                    name.to_string_lossy(),
                    current_value,
                    possible_values
                );

                if name.as_c_str() == c"type" {
                    match current_value {
                        1 => {
                            trace!("    This is a primary plane");
                            plane = Some(id)
                        }
                        2 => trace!("    This is an overlay plane"),
                        3 => trace!("    This is a cursor plane"),
                        _ => trace!("    Unknown plane type"),
                    }
                }
            }
        }

        let plane = plane.expect("Failed to find an appropriate plane");

        let surface_target = SurfaceTargetUnsafe::Drm {
            fd: drm_device.as_fd().as_raw_fd(),
            plane,
            connector_id: connector.connector_id.into(),
            width: mode.display_width() as u32,
            height: mode.display_height() as u32,
            refresh_rate: mode.vertical_refresh_rate() * 1000,
        };

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            flags: wgpu::InstanceFlags::default()
                | wgpu::InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface_unsafe(surface_target)? };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
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

    pub fn present(&self) -> Result<()> {
        let frame = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // If you wanted to call any drawing commands, they would go here.

        // End the renderpass.
        drop(renderpass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);

        frame.present();

        Ok(())
    }
}
