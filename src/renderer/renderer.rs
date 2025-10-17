use super::camera::{Camera, CameraUniform, Vertex};
use crate::world::{Chunk, World, CHUNK_SIZE, WORLD_HEIGHT};
use glam::Vec3;
use std::collections::HashMap;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    chunk_meshes: HashMap<(i32, i32), ChunkMesh>,
}

struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Renderer {
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find an adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create camera buffer
        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            depth_texture,
            depth_view,
            camera_buffer,
            camera_bind_group,
            chunk_meshes: HashMap::new(),
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // Recreate depth texture
            self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: new_size.width,
                    height: new_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            self.depth_view = self
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn update_camera(&mut self, camera: &Camera) {
        let mut uniform = CameraUniform::new();
        uniform.update(
            camera.build_view_matrix(),
            camera.build_projection_matrix(),
            camera.position,
        );
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn update_chunks(&mut self, world: &World, camera_pos: Vec3) {
        let camera_chunk_x = (camera_pos.x / CHUNK_SIZE as f32).floor() as i32;
        let camera_chunk_z = (camera_pos.z / CHUNK_SIZE as f32).floor() as i32;
        let render_distance = 4;

        // Remove chunks that are too far
        self.chunk_meshes.retain(|(x, z), _| {
            (*x - camera_chunk_x).abs() <= render_distance
                && (*z - camera_chunk_z).abs() <= render_distance
        });

        // Generate meshes for visible chunks
        for loaded_chunk in world.get_loaded_chunks() {
            let (chunk_x, chunk_z) = loaded_chunk;
            if (chunk_x - camera_chunk_x).abs() <= render_distance
                && (chunk_z - camera_chunk_z).abs() <= render_distance
            {
                if !self.chunk_meshes.contains_key(&(chunk_x, chunk_z)) {
                    // This is a simplified approach - in production you'd want to do this asynchronously
                    // For now, we'll skip mesh generation in this method
                }
            }
        }
    }

    pub fn generate_chunk_mesh(&mut self, chunk: &Chunk) {
        let (vertices, indices) = self.build_chunk_mesh(chunk);

        if indices.is_empty() {
            return;
        }

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.chunk_meshes.insert(
            (chunk.x, chunk.z),
            ChunkMesh {
                vertex_buffer,
                index_buffer,
                num_indices: indices.len() as u32,
            },
        );
    }

    fn build_chunk_mesh(&self, chunk: &Chunk) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let chunk_offset = Vec3::new(
            chunk.x as f32 * CHUNK_SIZE as f32,
            0.0,
            chunk.z as f32 * CHUNK_SIZE as f32,
        );

        for x in 0..CHUNK_SIZE {
            for y in 0..WORLD_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z);
                    if !block.is_solid() {
                        continue;
                    }

                    let pos = Vec3::new(x as f32, y as f32, z as f32) + chunk_offset;
                    let color = block.get_color();

                    // Check each face
                    self.add_block_faces(
                        chunk,
                        x,
                        y,
                        z,
                        pos,
                        color,
                        &mut vertices,
                        &mut indices,
                    );
                }
            }
        }

        (vertices, indices)
    }

    fn add_block_faces(
        &self,
        chunk: &Chunk,
        x: usize,
        y: usize,
        z: usize,
        pos: Vec3,
        color: [f32; 3],
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        // Check neighbors and add faces
        let faces = [
            // Top (+Y)
            (
                y + 1 >= WORLD_HEIGHT || !chunk.get_block(x, y + 1, z).is_solid(),
                [
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0, 0.0],
                ],
                [0.0, 1.0, 0.0],
            ),
            // Bottom (-Y)
            (
                y == 0 || !chunk.get_block(x, y - 1, z).is_solid(),
                [
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                ],
                [0.0, -1.0, 0.0],
            ),
            // Front (+Z)
            (
                z + 1 >= CHUNK_SIZE || !chunk.get_block(x, y, z + 1).is_solid(),
                [
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0, 1.0],
                    [1.0, 1.0, 1.0],
                    [0.0, 1.0, 1.0],
                ],
                [0.0, 0.0, 1.0],
            ),
            // Back (-Z)
            (
                z == 0 || !chunk.get_block(x, y, z - 1).is_solid(),
                [
                    [0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [1.0, 1.0, 0.0],
                    [1.0, 0.0, 0.0],
                ],
                [0.0, 0.0, -1.0],
            ),
            // Right (+X)
            (
                x + 1 >= CHUNK_SIZE || !chunk.get_block(x + 1, y, z).is_solid(),
                [
                    [1.0, 0.0, 0.0],
                    [1.0, 1.0, 0.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 0.0, 1.0],
                ],
                [1.0, 0.0, 0.0],
            ),
            // Left (-X)
            (
                x == 0 || !chunk.get_block(x - 1, y, z).is_solid(),
                [
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 1.0, 1.0],
                    [0.0, 1.0, 0.0],
                ],
                [-1.0, 0.0, 0.0],
            ),
        ];

        for (visible, face_vertices, normal) in &faces {
            if *visible {
                let start_index = vertices.len() as u32;

                for vertex_pos in face_vertices {
                    vertices.push(Vertex {
                        position: [
                            pos.x + vertex_pos[0],
                            pos.y + vertex_pos[1],
                            pos.z + vertex_pos[2],
                        ],
                        color,
                        normal: *normal,
                    });
                }

                // Two triangles per face
                indices.extend_from_slice(&[
                    start_index,
                    start_index + 1,
                    start_index + 2,
                    start_index,
                    start_index + 2,
                    start_index + 3,
                ]);
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.53,
                            g: 0.81,
                            b: 0.92,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            for mesh in self.chunk_meshes.values() {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

use wgpu::util::DeviceExt;
