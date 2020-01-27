mod graphics;
use graphics::*;

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
        let (info, init) = Info::new(&device, &window, desc.format)?;
        queue.submit(&[init]);
        (swapchain, info)
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),

            event::Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                swapchain = device.create_swap_chain(&surface, &swapchain_desc(size));
                window.request_redraw();
            }

            event::Event::WindowEvent { event, .. } => match event {
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
                _ => {
                    // Do update
                }
            },

            event::Event::RedrawRequested(_) => {
                let frame = swapchain.get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&info.pipeline);
                    rpass.set_bind_group(0, &info.bind_group, &[]);
                    rpass.set_index_buffer(&info.index_buf, 0);
                    rpass.set_vertex_buffers(0, &[(&info.vertex_buf, 0)]);
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
