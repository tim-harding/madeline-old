use std::mem::size_of;

pub struct Info {
    pub pass_bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl Info {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: wgpu::SwapChainDescriptor,
    ) -> Result<Self, &'static str> {
        let pass_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    },
                ],
            });

        Ok(Self {
            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&pass_bind_group_layout],
                }),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &shader_module(&device, "shaders/node_rasterize/vert.spv")?,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &shader_module(&device, "shaders/node_rasterize/frag.spv")?,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::None,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
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
            }),

            pass_bind_group_layout,
        })
    }
}

fn shader_module(device: &wgpu::Device, path: &str) -> Result<wgpu::ShaderModule, &'static str> {
    Ok(device.create_shader_module(
        &wgpu::read_spirv(std::fs::File::open(path).map_err(|_| "Could not read shader")?)
            .map_err(|_| "Could not read SPIR-V")?,
    ))
}
