use crate::{
    camera,
    chunking::chunk_renderer::DrawChunks,
    rendering::Renderer,
    texture::{
        self,
        Texture,
    },
    vertex::{
        TextureVertex,
        Vertex,
    },
    world_gen::voxel_world::VoxelWorld,
};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{
        MouseButton,
        MouseScrollDelta,
    },
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::{
        CursorGrabMode,
        Window,
    },
};



pub struct State
{
    pub renderer: Renderer,
    render_pipeline: wgpu::RenderPipeline,
    diffuse_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    camera: camera::Camera,
    projection: camera::Projection,
    pub camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    voxel_world: VoxelWorld,
    pub mouse_pos: PhysicalPosition<f64>,
    pub mouse_pressed: bool,
}



impl State
{
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>
    {
        let renderer = Renderer::new(window.clone()).await?;

        window.set_cursor_grab(CursorGrabMode::Locked)?;

        let diffuse_bytes = include_bytes!("../res/textures/8pxBlocks.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            diffuse_bytes,
            "block_atlas",
        )
        .unwrap();

        let texture_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let diffuse_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

        let shader = renderer
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = camera::Projection::new(
            renderer.config.width,
            renderer.config.height,
            cgmath::Deg(110.0),
            0.01,
            1000.0,
        );
        let camera_controller = camera::CameraController::new(25.0, 1.8);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    label: Some("camera_bind_group_layout"),
                });

        let camera_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let depth_texture = texture::Texture::create_depth_texture(
            &renderer.device,
            &renderer.config,
            "depth_texture",
        );

        let render_pipeline =
            renderer
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),

                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"), // 1.
                        buffers: &[TextureVertex::desc() /* InstanceRaw::desc() */], // 2.
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },

                    fragment: Some(wgpu::FragmentState {
                        // 3.
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            // 4.
                            format: renderer.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),

                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },

                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less, // 1.
                        stencil: wgpu::StencilState::default(),     // 2.
                        bias: wgpu::DepthBiasState::default(),
                    }),

                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },

                    multiview: None, // 5.
                    cache: None,     // 6.
                });

        let voxel_world = VoxelWorld::new();

        Ok(Self {
            voxel_world,
            renderer,
            render_pipeline,
            diffuse_bind_group,
            depth_texture,
            camera,
            projection,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 },
            mouse_pressed: false,
        })
    }



    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.renderer.resize(width, height);
        self.projection.resize(width, height);

        if width > 0 && height > 0
        {
            self.depth_texture = texture::Texture::create_depth_texture(
                &self.renderer.device,
                &self.renderer.config,
                "depth_texture",
            );
        }
    }



    pub fn render(&self) -> Result<(), wgpu::SurfaceError>
    {
        self.renderer.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.renderer.is_surface_configured
        {
            return Ok(());
        }

        let output = self.renderer.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.renderer
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
                            r: self.mouse_pos.x / self.renderer.config.width as f64,
                            g: self.mouse_pos.y / self.renderer.config.height as f64,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            render_pass.draw_chunks(&self.voxel_world.chunk_renderer);
        }

        self.renderer
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }



    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool)
    {
        if (code, is_pressed) == (KeyCode::Escape, true)
        {
            event_loop.exit();
        }
        else
        {
            self.camera_controller.handle_key(code, is_pressed);
        }
    }



    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool)
    {
        match button
        {
            MouseButton::Left => self.mouse_pressed = pressed,
            _ =>
            {}
        }
    }



    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta)
    {
        self.camera_controller.handle_scroll(delta);
    }



    pub fn update(&mut self, dt: instant::Duration)
    {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.renderer.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        self.voxel_world.update(&self.camera, &self.renderer, dt);
    }
}
