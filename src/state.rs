use anyhow::{Context, Result};
use rand::Rng;
use wgpu::util::DeviceExt;

use crate::context::WgpuContext;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct WindowProps {
    scale: [f32; 2],
    offset: [f32; 2],
}

impl WindowProps {
    fn layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("window_props_layou"),
        }
    }
}

struct Window {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    props_buffer: wgpu::Buffer,
    props_bind_group: wgpu::BindGroup,
    num_indices: u32,
}

pub struct State<'c> {
    render_pipeline: wgpu::RenderPipeline,
    windows: Vec<Window>,
    context: WgpuContext<'c>,
}

impl State<'_> {
    pub async fn new() -> Result<Self> {
        let context = WgpuContext::new().await?;

        let device = &context.device;
        let queue = &context.queue;
        let config = &context.config;

        // Compile the window shader
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let window_props_bind_group_layout =
            device.create_bind_group_layout(&WindowProps::layout());

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&window_props_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create a rendering pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Red triangle render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let num_objects = 1;
        let mut windows = Vec::with_capacity(num_objects);

        let mut rng = rand::thread_rng();

        for _ in 0..num_objects {
            let aspect = config.width / config.height;
            let scale = rng.gen_range(0.2..0.5);

            let props = WindowProps {
                scale: [scale / (aspect as f32), scale],
                offset: [rng.gen_range(-0.9..0.9), rng.gen_range(-0.9..0.9)],
            };

            let props = WindowProps {
                scale: [1.0, 1.0],
                offset: [0.0, 0.0],
            };

            let vertices = &[
                Vertex {
                    position: [-1.0, 1.0],
                    color: [rng.gen(), rng.gen(), rng.gen()],
                },
                Vertex {
                    position: [-1.0, -1.0],
                    color: [rng.gen(), rng.gen(), rng.gen()],
                },
                Vertex {
                    position: [1.0, -1.0],
                    color: [rng.gen(), rng.gen(), rng.gen()],
                },
                Vertex {
                    position: [1.0, 1.0],
                    color: [rng.gen(), rng.gen(), rng.gen()],
                },
            ];

            let indices: &[u16] = &[0, 1, 2, 2, 3, 0];

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            let props_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[props]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let props_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &window_props_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: props_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

            windows.push(Window {
                vertex_buffer,
                index_buffer,
                props_buffer,
                props_bind_group,
                num_indices: indices.len() as u32,
            })
        }

        Ok(Self {
            render_pipeline,
            windows,
            context,
        })
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.context.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Encoder"),
                });

        {
            let mut render_pass: wgpu::RenderPass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });

            render_pass.set_pipeline(&self.render_pipeline);

            for window in &self.windows {
                render_pass.set_bind_group(0, &window.props_bind_group, &[]);
                render_pass.set_vertex_buffer(0, window.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(window.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..window.num_indices, 0, 0..1);
            }
        }

        let command_buffer = encoder.finish();
        self.context.queue.submit([command_buffer]);

        output.present();

        Ok(())
    }
}
