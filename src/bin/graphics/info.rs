use super::quad;
use super::utils;
use super::GraphGeo;

pub struct Info {
    pub vbo: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub indices: u32,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub locals_uniform: wgpu::Buffer,
}

impl Info {
    pub fn new(
        device: &wgpu::Device,
        swapchain_format: wgpu::TextureFormat,
    ) -> Result<(Self, wgpu::CommandBuffer), &'static str> {
        let shader_module = |path| shader_module(&device, path);
        let vs_module = shader_module("shaders/vert.spv")?;
        let fs_module = shader_module("shaders/frag.spv")?;

        let GraphGeo { geometry } = GraphGeo::new()?;

        println!("Vertex count: {}", geometry.vertices.len());
        println!("Index count:  {}", geometry.indices.len());
        
        let vbo = device
            .create_buffer_mapped(geometry.vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&geometry.vertices);

        let ibo = device
            .create_buffer_mapped(geometry.indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&geometry.indices);

        // Create pipeline layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        // Dynamic uniform buffers are used for
                        // instancing, so each instance could,
                        // for example, get a different transform matrix
                        dynamic: false,
                    },
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let (pixels, dims) = {
            let img = image::open("data/kitty.png").map_err(|_| "Could not open texture")?;
            let mut img = match img {
                image::DynamicImage::ImageRgba8(img) => Ok(img),
                _ => Err("Unexpected texture format"),
            }?;
            let img = match (img.width() * 4) % 256 {
                0 => img,
                _ => {
                    log::info!("Resizing image to match stride");
                    let width = img.width() * 4 / 256 * 256 + 1;
                    assert!(width > img.width());
                    let height = img.height();
                    image::imageops::crop(&mut img, 0, 0, width, height).to_image()
                }
            };
            let dims = img.dimensions();
            let raw = img.into_raw();
            (raw, dims)
        };

        let texture_extent = wgpu::Extent3d {
            width: dims.0,
            height: dims.1,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });
        let texture_view = texture.create_default_view();
        let temp_buf = utils::buffer_with_data(&device, &pixels, wgpu::BufferUsage::COPY_SRC);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buf,
                offset: 0,
                row_pitch: 4 * dims.0,
                image_height: dims.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            texture_extent,
        );

        // Create other resources
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        let locals_uniform = utils::buffer_with_data(
            &device,
            &[utils::Locals { brightness: 0.5 }],
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &locals_uniform,
                        range: 0..std::mem::size_of::<utils::Locals>() as u64,
                    },
                },
            ],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
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
                format: swapchain_format,
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
                stride: (std::mem::size_of::<f32>() * 2) as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float2,
                        offset: 0,
                        shader_location: 0,
                    },
                ],
            }],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok((
            Self {
                vbo,
                ibo,
                indices: geometry.indices.len() as u32,
                bind_group,
                pipeline,
                locals_uniform,
            },
            encoder.finish(),
        ))
    }
}

fn shader_module(device: &wgpu::Device, path: &str) -> Result<wgpu::ShaderModule, &'static str> {
    Ok(device.create_shader_module(
        &wgpu::read_spirv(std::fs::File::open(path).map_err(|_| "Could not read shader")?)
            .map_err(|_| "Could not read SPIR-V")?,
    ))
}
