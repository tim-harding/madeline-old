mod graphics;
use crate::graphics::GraphGeo;
use graphics::{
    quad,
    utils::{self, Globals, PassFrag, PassVert, Vec2, Vec3},
    Info, Mesh,
};
use std::mem::size_of;

#[repr(C)]
struct Component {
    pub bind_group: wgpu::BindGroup,
    pub mesh: Mesh,
    pub pass_vert: wgpu::Buffer,
    pub pass_frag: wgpu::Buffer,
}

impl Component {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        mesh: Mesh,
        offset: Vec2,
        color: Vec3,
    ) -> Self {
        let pass_vert = device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM)
            .fill_from_slice(&[offset]);
        let pass_frag = device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM)
            .fill_from_slice(&[color]);
        Self {
            mesh,
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &pass_vert,
                            range: 0..size_of::<PassVert>() as u64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &pass_frag,
                            range: 0..size_of::<PassFrag>() as u64,
                        },
                    },
                ],
            }),
            pass_vert,
            pass_frag,
        }
    }
}

fn main() -> Result<(), &'static str> {
    use winit::{
        event::{self, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    };

    {
        let colors = fern::colors::ColoredLevelConfig::default();
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Error)
            .chain(std::io::stdout())
            .apply()
            .map_err(|_| "Failed to start logger")?;
    }

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Madeline")
        .build(&event_loop)
        .map_err(|_| "Could not create window")?;
    let surface = wgpu::Surface::create(&window);

    let (device, mut queue) = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        // Vulkan, metal, or DX12, rather than OpenGL or DX11
        backends: wgpu::BackendBit::PRIMARY,
    })
    .ok_or("Could not get adapter")?
    .request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let (mut swapchain, info) = {
        let desc = swapchain_desc(window.inner_size());
        let swapchain = device.create_swap_chain(&surface, &desc);
        let info = Info::new(&device, desc)?;
        (swapchain, info)
    };

    let components = {
        let comp = |mesh, offset, color| {
            Component::new(&device, &info.pass_bind_group_layout, mesh, offset, color)
        };

        let geo = GraphGeo::new(&device)?;
        let GraphGeo {
            rect,
            rect_outline,
            slot,
            trapezoid,
        } = geo;

        [
            // comp(rect, Vec2::new(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            // comp(rect_outline, Vec2::new(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            comp(slot, Vec2::new(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            // comp(trapezoid, Vec2::new(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        ]
    };

    let node_texture_intermediate = device
        .create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 182,
                height: 56,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: swapchain_desc(window.inner_size()).format,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        })
        .create_default_view();

    let quad_vbo = device
        .create_buffer_mapped(quad::VERTICES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(quad::VERTICES);

    let quad_ibo = device
        .create_buffer_mapped(quad::INDICES.len(), wgpu::BufferUsage::INDEX)
        .fill_from_slice(quad::INDICES);

    let resolution_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    });

    let resolution_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
            ],
        });

    let globals_uniform = utils::buffer::<Globals>(&device);

    let resolution_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &resolution_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&node_texture_intermediate),
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&resolution_sampler),
            },
            wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &globals_uniform,
                    range: 0..size_of::<Globals>() as u64,
                },
            },
        ],
    });

    let resolution_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&resolution_bind_group_layout],
        }),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(
                    std::fs::File::open("shaders/node_resolution/vert.spv")
                        .map_err(|_| "Could not read shader")?,
                )
                .map_err(|_| "Could not read SPIR-V")?,
            ),
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(
                    std::fs::File::open("shaders/node_resolution/frag.spv")
                        .map_err(|_| "Could not read shader")?,
                )
                .map_err(|_| "Could not read SPIR-V")?,
            ),
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: swapchain_desc(window.inner_size()).format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: (size_of::<f32>() * 2) as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float2,
                offset: 0,
                shader_location: 0,
            }],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut globals = Globals {
        screen_size: Vec2::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        ),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),

            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    globals.screen_size = Vec2::new(size.width as f32, size.height as f32);
                    swapchain = device.create_swap_chain(&surface, &swapchain_desc(size));
                    window.request_redraw();
                }

                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },

            event::Event::RedrawRequested(_) => {
                let frame = swapchain.get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                {
                    let mut init_encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                    let tmp_buf = device
                        .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                        .fill_from_slice(&[globals]);
                    init_encoder.copy_buffer_to_buffer(
                        &tmp_buf,
                        0,
                        &globals_uniform,
                        0,
                        std::mem::size_of::<Globals>() as u64,
                    );
                    queue.submit(&[init_encoder.finish()]);
                }

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &node_texture_intermediate,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 1.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&info.pipeline);
                    for c in components.iter() {
                        rpass.set_bind_group(0, &c.bind_group, &[]);
                        rpass.set_index_buffer(&c.mesh.ibo, 0);
                        rpass.set_vertex_buffers(0, &[(&c.mesh.vbo, 0)]);
                        rpass.draw_indexed(0..c.mesh.indices as u32, 0, 0..1);
                    }
                }

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.2,
                                g: 0.4,
                                b: 0.6,
                                a: 0.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    rpass.set_pipeline(&resolution_pipeline);
                    rpass.set_bind_group(0, &resolution_bind_group, &[]);
                    rpass.set_index_buffer(&quad_ibo, 0);
                    rpass.set_vertex_buffers(0, &[(&quad_vbo, 0)]);
                    rpass.draw_indexed(0..quad::INDICES.len() as u32, 0, 0..1);
                }

                let command_buf = encoder.finish();
                queue.submit(&[command_buf]);
            }
            _ => {}
        }
    });
}

fn swapchain_desc(size: winit::dpi::PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
    wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Vsync,
    }
}
