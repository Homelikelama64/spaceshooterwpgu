use crate::always_some::AlwaysSome;
use encase::{ShaderSize, ShaderType, StorageBuffer, UniformBuffer};
use slotmap::{SlotMap, SparseSecondaryMap};
use std::sync::Arc;
use texture::{Texture, TextureId};
use winit::{dpi::PhysicalSize, window::Window};

pub mod texture;

#[derive(ShaderType)]
struct Camera {
    position: cgmath::Vector2<f32>,
    view_height: f32,
    aspect: f32,
}

#[derive(ShaderType)]
struct Quad {
    position: cgmath::Vector2<f32>,
    size: cgmath::Vector2<f32>,
    color: cgmath::Vector4<f32>,
    rotation: f32,
}

pub struct Renderer {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    quads_bind_group_layout: wgpu::BindGroupLayout,
    quad_buffers: SparseSecondaryMap<TextureId, (wgpu::Buffer, wgpu::BindGroup)>,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    quad_render_pipeline: wgpu::RenderPipeline,
    background_render_pipeline: wgpu::RenderPipeline,
    textures: SlotMap<TextureId, Texture>,
    default_texture: TextureId,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Renderer Device"),
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: Camera::SHADER_SIZE.get(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(Camera::SHADER_SIZE),
                    },
                    count: None,
                }],
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        let quads_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quads Storage Buffer"),
            size: <&[Quad]>::min_size().get(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let quads_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Quads Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: Some(<&[Quad]>::min_size()),
                    },
                    count: None,
                }],
            });
        let quads_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Quads Bind Group"),
            layout: &quads_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: quads_storage_buffer.as_entire_binding(),
            }],
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let quad_shader = device.create_shader_module(wgpu::include_wgsl!("./quad_shader.wgsl"));

        let quad_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Quad Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &quads_bind_group_layout,
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let quad_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Render Pipeline"),
            layout: Some(&quad_render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &quad_shader,
                entry_point: "vertex",
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &quad_shader,
                entry_point: "fragment",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });

        let background_shader =
            device.create_shader_module(wgpu::include_wgsl!("./background_shader.wgsl"));

        let background_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Background Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        let background_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Background Render Pipeline"),
                layout: Some(&background_render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &background_shader,
                    entry_point: "vertex",
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &background_shader,
                    entry_point: "fragment",
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                }),
                multiview: None,
                cache: None,
            });

        let mut textures = SlotMap::with_key();
        let default_texture = textures.insert(Texture::new(
            "Default Texture",
            1,
            1,
            &[255, 255, 255, 255],
            &device,
            &queue,
            &texture_bind_group_layout,
        ));

        Self {
            window,
            surface,
            surface_config,
            device,
            queue,
            camera_uniform_buffer,
            camera_bind_group,
            quads_bind_group_layout,
            quad_buffers: SparseSecondaryMap::new(),
            texture_bind_group_layout,
            quad_render_pipeline,
            background_render_pipeline,
            textures,
            default_texture,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn create_texture(
        &mut self,
        label: &str,
        width: u32,
        height: u32,
        pixels: &[u8],
    ) -> TextureId {
        self.textures.insert(Texture::new(
            label,
            width,
            height,
            pixels,
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
        ))
    }
}

pub struct FrameRendering<'renderer> {
    renderer: &'renderer mut Renderer,
    output: AlwaysSome<wgpu::SurfaceTexture>,
    render_encoder: AlwaysSome<wgpu::CommandEncoder>,
    render_pass: AlwaysSome<wgpu::RenderPass<'static>>,
}

impl<'renderer> FrameRendering<'renderer> {
    pub fn new(renderer: &'renderer mut Renderer, clear_color: wgpu::Color) -> Option<Self> {
        let output = match renderer.surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::Timeout) => return None,
            Err(wgpu::SurfaceError::Outdated) => {
                let size = renderer.window.inner_size();
                renderer.resize(size);
                return None;
            }
            e => e.unwrap(),
        };
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_encoder =
            renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let render_pass = render_encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            })
            .forget_lifetime();

        Some(FrameRendering {
            renderer,
            output: output.into(),
            render_encoder: render_encoder.into(),
            render_pass: render_pass.into(),
        })
    }
}

impl Drop for FrameRendering<'_> {
    fn drop(&mut self) {
        // make sure to drop render pass before submitting encoder
        drop(self.render_pass.take());
        self.renderer
            .queue
            .submit(std::iter::once(self.render_encoder.take().finish()));

        self.renderer.window.pre_present_notify();
        self.output.take().present();
    }
}

pub struct Rendering2D<'renderer, 'frame> {
    frame: &'frame mut FrameRendering<'renderer>,
    camera_size: cgmath::Vector2<f32>,
    quads: SparseSecondaryMap<TextureId, Vec<Quad>>,
}

impl<'renderer, 'frame> Rendering2D<'renderer, 'frame> {
    pub fn new(
        frame: &'frame mut FrameRendering<'renderer>,
        camera_position: cgmath::Vector2<f32>,
        camera_height: f32,
    ) -> Self {
        let renderer = &mut *frame.renderer;

        let window_size = renderer.window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;

        // Upload camera
        {
            let camera = Camera {
                position: camera_position,
                view_height: camera_height / 2.0,
                aspect,
            };

            let camera_buffer = &mut *renderer
                .queue
                .write_buffer_with(&renderer.camera_uniform_buffer, 0, Camera::SHADER_SIZE)
                .unwrap();

            UniformBuffer::new(camera_buffer).write(&camera).unwrap();
        }
        frame
            .render_pass
            .set_bind_group(0, &renderer.camera_bind_group, &[]);

        Self {
            frame,
            camera_size: cgmath::vec2(camera_height * aspect, camera_height),
            quads: SparseSecondaryMap::new(),
        }
    }

    pub fn get_camera_size(&self) -> cgmath::Vector2<f32> {
        self.camera_size
    }

    pub fn reserve_quads(&mut self, additional: usize) {
        self.quads.reserve(additional);
    }

    pub fn draw_quad(
        &mut self,
        position: cgmath::Vector2<f32>,
        size: cgmath::Vector2<f32>,
        color: cgmath::Vector4<f32>,
        rotation: f32,
        texture: Option<TextureId>,
    ) {
        self.quads
            .entry(texture.unwrap_or(self.frame.renderer.default_texture))
            .unwrap()
            .or_default()
            .push(Quad {
                position,
                size,
                color,
                rotation: rotation.to_radians(),
            });
    }
}

impl Drop for Rendering2D<'_, '_> {
    fn drop(&mut self) {
        let renderer = &mut *self.frame.renderer;

        // Draw background
        {
            self.frame
                .render_pass
                .set_pipeline(&renderer.background_render_pipeline);
            self.frame.render_pass.draw(0..4, 0..1);
        }

        // Draw quads
        {
            self.frame
                .render_pass
                .set_pipeline(&renderer.quad_render_pipeline);

            // Upload quads
            for (texture, quads) in &self.quads {
                let quads_size = quads.size();

                let (quads_storage_buffer, quads_bind_group) =
                    match renderer.quad_buffers.entry(texture).unwrap() {
                        slotmap::sparse_secondary::Entry::Occupied(occupied_entry)
                            if quads_size.get() <= occupied_entry.get().0.size() =>
                        {
                            occupied_entry.into_mut()
                        }
                        _ => {
                            let quads_storage_buffer =
                                renderer.device.create_buffer(&wgpu::BufferDescriptor {
                                    label: Some("Quads Storage Buffer"),
                                    size: quads_size.get(),
                                    usage: wgpu::BufferUsages::STORAGE
                                        | wgpu::BufferUsages::COPY_DST,
                                    mapped_at_creation: false,
                                });
                            let quads_bind_group =
                                renderer
                                    .device
                                    .create_bind_group(&wgpu::BindGroupDescriptor {
                                        label: Some("Quads Bind Group"),
                                        layout: &renderer.quads_bind_group_layout,
                                        entries: &[wgpu::BindGroupEntry {
                                            binding: 0,
                                            resource: quads_storage_buffer.as_entire_binding(),
                                        }],
                                    });
                            renderer
                                .quad_buffers
                                .insert(texture, (quads_storage_buffer, quads_bind_group));
                            renderer.quad_buffers.get_mut(texture).unwrap()
                        }
                    };

                {
                    let buffer = &mut *renderer
                        .queue
                        .write_buffer_with(quads_storage_buffer, 0, quads_size)
                        .unwrap();

                    StorageBuffer::new(buffer).write(quads).unwrap();
                }

                self.frame
                    .render_pass
                    .set_bind_group(1, quads_bind_group, &[]);
                self.frame.render_pass.set_bind_group(
                    2,
                    &renderer.textures[texture].bind_group,
                    &[],
                );

                self.frame
                    .render_pass
                    .draw(0..4, 0..quads.len().try_into().unwrap());
            }
        }
    }
}
