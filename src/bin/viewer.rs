mod graphics;
use crate::graphics::GraphGeo;
use graphics::{
    utils::{self, Globals, PassFrag, PassVert, Vec2, Vec3},
    Info, Mesh,
};
use std::mem::size_of;

#[repr(C)]
struct Component {
    pub uniforms: Uniforms,
    pub bind_group: wgpu::BindGroup,
    pub mesh: Mesh,
}

impl Component {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        mesh: Mesh,
        offset: Vec2,
        color: Vec3,
    ) -> Self {
        let pass_vert_buffer = utils::buffer::<PassVert>(device);
        let pass_frag_buffer = utils::buffer::<PassFrag>(device);
        Self {
            mesh,
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &pass_vert_buffer,
                            range: 0..size_of::<PassVert>() as u64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &pass_frag_buffer,
                            range: 0..size_of::<PassFrag>() as u64,
                        },
                    },
                ],
            }),
            uniforms: Uniforms {
                pass_vert: Uniform {
                    data: PassVert { offset },
                    buffer: pass_vert_buffer,
                },
                pass_frag: Uniform {
                    data: PassFrag { color },
                    buffer: pass_frag_buffer,
                },
            },
        }
    }
}

struct Uniform<T> {
    pub data: T,
    pub buffer: wgpu::Buffer,
}

struct Uniforms {
    pub pass_vert: Uniform<PassVert>,
    pub pass_frag: Uniform<PassFrag>,
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

    let mut globals = {
        let size = window.inner_size();
        Globals {
            screen_size: Vec2::new(size.width as f32, size.height as f32),
        }
    };

    let (mut swapchain, mut info) = {
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
            comp(rect, Vec2::new(0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            comp(
                rect_outline,
                Vec2::new(30.0, 30.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            comp(slot, Vec2::new(60.0, 60.0), Vec3::new(0.0, 0.0, 1.0)),
            comp(trapezoid, Vec2::new(90.0, 90.0), Vec3::new(0.0, 0.0, 0.0)),
        ]
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),

            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    globals.screen_size = Vec2::new(size.width as f32, size.height as f32);
                    let sc_desc = swapchain_desc(size);
                    info.msaa_frame = utils::create_msaa_buffer(&device, &sc_desc);
                    swapchain = device.create_swap_chain(&surface, &sc_desc);
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
                    // Repopulate globals uniform
                    let tmp_buf = device
                        .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                        .fill_from_slice(&[globals]);
                    encoder.copy_buffer_to_buffer(
                        &tmp_buf,
                        0,
                        &info.globals_uniform,
                        0,
                        std::mem::size_of::<Globals>() as u64,
                    );
                }

                {
                    // Repopulate pass uniforms
                    // Component contains some unnecessary bytes,
                    // perhaps there's a more compact way of getting
                    // this data to the buffer.
                    let component_data = unsafe {
                        std::slice::from_raw_parts(
                            components.as_ptr() as *const u8,
                            components.len() * size_of::<Component>(),
                        )
                    };

                    let tmp_buf = device
                        .create_buffer_mapped(
                            components.len() * size_of::<Component>(),
                            wgpu::BufferUsage::COPY_SRC,
                        )
                        .fill_from_slice(&component_data);

                    for (i, c) in components.iter().enumerate() {
                        let c_offset = (i * size_of::<Component>()) as u64;
                        encoder.copy_buffer_to_buffer(
                            &tmp_buf,
                            c_offset,
                            &c.uniforms.pass_vert.buffer,
                            0,
                            std::mem::size_of::<PassVert>() as u64,
                        );
                        encoder.copy_buffer_to_buffer(
                            &tmp_buf,
                            c_offset + size_of::<Uniform<PassVert>>() as u64,
                            &c.uniforms.pass_frag.buffer,
                            0,
                            std::mem::size_of::<PassFrag>() as u64,
                        );
                    }
                }

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &info.msaa_frame,
                            resolve_target: Some(&frame.view),
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
                    rpass.set_pipeline(&info.pipeline);
                    rpass.set_bind_group(0, &info.globals_bind_group, &[]);

                    for c in components.iter() {
                        rpass.set_bind_group(1, &c.bind_group, &[]);
                        rpass.set_index_buffer(&c.mesh.ibo, 0);
                        rpass.set_vertex_buffers(0, &[(&c.mesh.vbo, 0)]);
                        rpass.draw_indexed(0..c.mesh.indices as u32, 0, 0..1);
                    }
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
