use super::camera::{Camera, CameraUniform, Vertex};
use super::texture::{texture_key_for, AtlasUV, BlockFace, TextureResolver};
use super::advanced::AdvancedRenderer;
use crate::world::{BlockType, Chunk, World, CHUNK_SIZE, WORLD_HEIGHT};
use glam::Vec3;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[allow(dead_code)]
pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    default_vsync_mode: wgpu::PresentMode,
    non_vsync_mode: Option<wgpu::PresentMode>,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group: Option<wgpu::BindGroup>,
    atlas_texture: Option<wgpu::Texture>,
    atlas_texture_view: Option<wgpu::TextureView>,
    atlas_sampler: wgpu::Sampler,
    chunk_meshes: HashMap<(i32, i32), ChunkMesh>,
    pub advanced: AdvancedRenderer,
    // Shadow mapping resources (placeholder)
    shadow_texture: Option<wgpu::Texture>,
    shadow_view: Option<wgpu::TextureView>,
    shadow_sampler: Option<wgpu::Sampler>,
    shadow_bind_group_layout: Option<wgpu::BindGroupLayout>,
    shadow_bind_group: Option<wgpu::BindGroup>,
    // Light uniform and shadow pipeline
    light_buffer: Option<wgpu::Buffer>,
    light_bind_group_layout: Option<wgpu::BindGroupLayout>,
    light_bind_group: Option<wgpu::BindGroup>,
    shadow_pipeline: Option<wgpu::RenderPipeline>,
}

struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

// Light uniform struct used for shadow pass (moved to module scope)
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct LightUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl Renderer {
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        vsync_enabled: bool,
    ) -> anyhow::Result<Self> {
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

        let default_vsync_mode = surface_caps
            .present_modes
            .iter()
            .copied()
            .find(|mode| matches!(mode, wgpu::PresentMode::AutoVsync | wgpu::PresentMode::Fifo))
            .unwrap_or(wgpu::PresentMode::Fifo);
        let non_vsync_mode = surface_caps.present_modes.iter().copied().find(|mode| {
            matches!(
                mode,
                wgpu::PresentMode::AutoNoVsync
                    | wgpu::PresentMode::Immediate
                    | wgpu::PresentMode::Mailbox
            )
        });

        let present_mode = if vsync_enabled {
            default_vsync_mode
        } else {
            non_vsync_mode.unwrap_or(default_vsync_mode)
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
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

        let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Atlas Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 4.0,
            compare: None,
            anisotropy_clamp: 8,
            border_color: None,
        });

        // Shadow sampler and placeholder bind group layout
        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 4.0,
            compare: Some(wgpu::CompareFunction::LessEqual),
            anisotropy_clamp: 1,
            border_color: None,
        });

        let shadow_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });

        // Light uniform layout for shadow pass
        let light_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
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
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create shadow shader and pipeline (depth-only)
        let shadow_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shadow.wgsl").into()),
        });

        let shadow_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shadow Pipeline Layout"),
            bind_group_layouts: &[&light_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: Some(&shadow_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shadow_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: None,
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout, &shadow_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            default_vsync_mode,
            non_vsync_mode,
            size,
            render_pipeline,
            depth_texture,
            depth_view,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_layout,
            texture_bind_group: None,
            atlas_texture: None,
            atlas_texture_view: None,
            atlas_sampler,
            chunk_meshes: HashMap::new(),
            advanced: AdvancedRenderer::new(),
            shadow_texture: None,
            shadow_view: None,
            shadow_sampler: Some(shadow_sampler),
            shadow_bind_group_layout: Some(shadow_bind_group_layout),
            shadow_bind_group: None,
            light_buffer: None,
            light_bind_group_layout: Some(light_bind_group_layout),
            light_bind_group: None,
            shadow_pipeline: Some(shadow_pipeline),
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

    pub fn set_texture_atlas(&mut self, pixels: &[u8], width: u32, height: u32) {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Chunk Texture Atlas"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                },
            ],
        });

        self.atlas_texture = Some(texture);
        self.atlas_texture_view = Some(texture_view);
        self.texture_bind_group = Some(bind_group);
    }

    pub fn clear_chunk_meshes(&mut self) {
        self.chunk_meshes.clear();
    }

    pub fn update_chunks(&mut self, world: &World, camera_pos: Vec3, render_distance: i32) {
        let camera_chunk_x = (camera_pos.x / CHUNK_SIZE as f32).floor() as i32;
        let camera_chunk_z = (camera_pos.z / CHUNK_SIZE as f32).floor() as i32;

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

    

    pub fn set_vsync(&mut self, enabled: bool) {
        let target_mode = if enabled {
            self.default_vsync_mode
        } else {
            self.non_vsync_mode.unwrap_or(self.default_vsync_mode)
        };

        if self.config.present_mode != target_mode {
            self.config.present_mode = target_mode;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn invalidate_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunk_meshes.remove(&(chunk_x, chunk_z));
    }

    pub fn has_chunk_mesh(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.chunk_meshes.contains_key(&(chunk_x, chunk_z))
    }

    pub fn upload_chunk_mesh(
        &mut self,
        coords: (i32, i32),
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) {
        if vertices.is_empty() || indices.is_empty() {
            self.chunk_meshes.remove(&coords);
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
            coords,
            ChunkMesh {
                vertex_buffer,
                index_buffer,
                num_indices: indices.len() as u32,
            },
        );
    }

    pub fn build_chunk_mesh(
        chunk: &Chunk,
        texture_resolver: &TextureResolver,
    ) -> (Vec<Vertex>, Vec<u32>) {
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

                    let faces = [
                        (
                            BlockFace::Top,
                            y + 1 >= WORLD_HEIGHT
                                || !chunk.get_block(x, y + 1, z).is_solid(),
                        ),
                        (
                            BlockFace::Bottom,
                            y == 0 || !chunk.get_block(x, y.wrapping_sub(1), z).is_solid(),
                        ),
                        (
                            BlockFace::North,
                            z + 1 >= CHUNK_SIZE
                                || !chunk.get_block(x, y, z + 1).is_solid(),
                        ),
                        (
                            BlockFace::South,
                            z == 0 || !chunk.get_block(x, y, z - 1).is_solid(),
                        ),
                        (
                            BlockFace::East,
                            x + 1 >= CHUNK_SIZE
                                || !chunk.get_block(x + 1, y, z).is_solid(),
                        ),
                        (
                            BlockFace::West,
                            x == 0 || !chunk.get_block(x - 1, y, z).is_solid(),
                        ),
                    ];

                    for (face, visible) in faces {
                        if visible {
                            Self::emit_face(
                                block,
                                face,
                                pos,
                                color,
                                texture_resolver,
                                &mut vertices,
                                &mut indices,
                            );
                        }
                    }
                }
            }
        }

        (vertices, indices)
    }

    fn emit_face(
        block: BlockType,
        face: BlockFace,
        base_pos: Vec3,
        color: [f32; 3],
        texture_resolver: &TextureResolver,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        let rect = texture_resolver.uv(texture_key_for(block, face));
        let positions = Self::face_vertices(face);
        let uvs = Self::face_uvs(face, rect);
        let normal = Self::face_normal(face);

        let start_index = vertices.len() as u32;
        for (pos_offset, tex) in positions.iter().zip(uvs.iter()) {
            vertices.push(Vertex {
                position: [
                    base_pos.x + pos_offset[0],
                    base_pos.y + pos_offset[1],
                    base_pos.z + pos_offset[2],
                ],
                tex_coords: *tex,
                color,
                normal,
            });
        }

        indices.extend_from_slice(&[
            start_index,
            start_index + 1,
            start_index + 2,
            start_index,
            start_index + 2,
            start_index + 3,
        ]);
    }

    fn face_vertices(face: BlockFace) -> [[f32; 3]; 4] {
        match face {
            BlockFace::Top => [
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [1.0, 1.0, 0.0],
            ],
            BlockFace::Bottom => [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            BlockFace::North => [
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
            ],
            BlockFace::South => [
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ],
            BlockFace::East => [
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, 1.0, 1.0],
                [1.0, 0.0, 1.0],
            ],
            BlockFace::West => [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 0.0],
            ],
        }
    }

    fn face_uvs(face: BlockFace, rect: AtlasUV) -> [[f32; 2]; 4] {
        match face {
            BlockFace::Top => [
                [rect.u_min, rect.v_max],
                [rect.u_min, rect.v_min],
                [rect.u_max, rect.v_min],
                [rect.u_max, rect.v_max],
            ],
            BlockFace::Bottom => [
                [rect.u_min, rect.v_min],
                [rect.u_max, rect.v_min],
                [rect.u_max, rect.v_max],
                [rect.u_min, rect.v_max],
            ],
            BlockFace::North => [
                [rect.u_min, rect.v_max],
                [rect.u_max, rect.v_max],
                [rect.u_max, rect.v_min],
                [rect.u_min, rect.v_min],
            ],
            BlockFace::South => [
                [rect.u_max, rect.v_max],
                [rect.u_max, rect.v_min],
                [rect.u_min, rect.v_min],
                [rect.u_min, rect.v_max],
            ],
            BlockFace::East => [
                [rect.u_max, rect.v_max],
                [rect.u_max, rect.v_min],
                [rect.u_min, rect.v_min],
                [rect.u_min, rect.v_max],
            ],
            BlockFace::West => [
                [rect.u_min, rect.v_max],
                [rect.u_max, rect.v_max],
                [rect.u_max, rect.v_min],
                [rect.u_min, rect.v_min],
            ],
        }
    }

    fn face_normal(face: BlockFace) -> [f32; 3] {
        match face {
            BlockFace::Top => [0.0, 1.0, 0.0],
            BlockFace::Bottom => [0.0, -1.0, 0.0],
            BlockFace::North => [0.0, 0.0, 1.0],
            BlockFace::South => [0.0, 0.0, -1.0],
            BlockFace::East => [1.0, 0.0, 0.0],
            BlockFace::West => [-1.0, 0.0, 0.0],
        }
    }

    // Render the 3D scene into an existing command encoder and texture view.
    pub fn draw_scene(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Scene Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
        if let Some(bind_group) = &self.texture_bind_group {
            render_pass.set_bind_group(1, bind_group, &[]);
        }

        for mesh in self.chunk_meshes.values() {
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
    }

    /// Accessor for the depth view used by the renderer (read-only).
    pub fn depth_view(&self) -> &wgpu::TextureView {
        &self.depth_view
    }
}
