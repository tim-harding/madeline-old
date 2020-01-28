use super::utils::{self, Globals, PassVert, PassFrag};
use super::GraphGeo;
use std::mem::size_of;

pub struct Info {
    pub geo: GraphGeo,
    pub globals_bind_group: wgpu::BindGroup,
    pub pass_bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub uniforms: Uniforms,
    pub msaa_frame: wgpu::TextureView,
}

pub struct Uniforms {
    pub globals: wgpu::Buffer,
    pub pass_vert: wgpu::Buffer,
    pub pass_frag: wgpu::Buffer,
}

impl Uniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            globals: buffer::<Globals>(device),
            pass_vert: buffer::<PassVert>(device),
            pass_frag: buffer::<PassFrag>(device),
        }
    }
}

fn buffer<T>(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        size: size_of::<T>() as u64,
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    })
}

impl Info {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: wgpu::SwapChainDescriptor,
    ) -> Result<Self, &'static str> {
        let geo = GraphGeo::new(device)?;

        let uniforms = Uniforms::new(&device);

        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        // Dynamic uniform buffers are used for
                        // instancing, so each instance could,
                        // for example, get a different transform matrix
                        dynamic: false,
                    },
                },
            ],
        });

        let pass_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
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
            globals_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &globals_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniforms.globals,
                            range: 0..size_of::<Globals>() as u64,
                        },
                    },
                ],
            }),

            pass_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &pass_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniforms.pass_vert,
                            range: 0..size_of::<PassVert>() as u64,
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniforms.pass_frag,
                            range: 0..size_of::<PassFrag>() as u64,
                        },
                    },
                ],
            }),

            pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&globals_bind_group_layout, &pass_bind_group_layout],
                }),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &shader_module(&device, "shaders/vert.spv")?,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &shader_module(&device, "shaders/frag.spv")?,
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
                sample_count: utils::SAMPLES,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }),

            msaa_frame: utils::create_msaa_buffer(&device, &sc_desc),

            geo,
            uniforms,
        })
    }
}

fn shader_module(device: &wgpu::Device, path: &str) -> Result<wgpu::ShaderModule, &'static str> {
    Ok(device.create_shader_module(
        &wgpu::read_spirv(std::fs::File::open(path).map_err(|_| "Could not read shader")?)
            .map_err(|_| "Could not read SPIR-V")?,
    ))
}
