use crate::{
    celestial_bodies::{
        CelestialBodyContainer,
        planet::planet_meshing::CelestialMeshContainer,
    },
    input::{
        InputEvent,
        InputHandler,
        input_settings::InputSettings,
    },
    math,
    physics::{
        physics_sim::PhysicsSim,
        trajectory::calculate_trajectory,
    },
    player::spaceship_controller::SpaceshipController,
    rendering::{
        RenderState,
        camera::{
            self,
            CameraUniform,
            OrbitCamera,
            camera_controller::CameraController,
        },
        lighting::{
            LightUniform,
            SphereLight,
            SunLight,
        },
        mesh::MeshBuffer,
        renderer::Renderer,
    },
    texture::{
        self,
        Texture,
    },
    vertex::{
        TextureVertex,
        Vertex,
    },
};
use nalgebra::{
    Point3,
    Vector3,
    Vector4,
};
use rand::rngs::ThreadRng;
use std::{
    sync::Arc,
    time::Duration,
};
use wgpu::BindGroupLayout;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::{
        CursorGrabMode,
        Window,
    },
};



pub struct State
{
    pub renderer_state: RenderState,
    pub render_pipeline: wgpu::RenderPipeline,
    pub line_render_pipeline: wgpu::RenderPipeline,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub depth_texture: Texture,
    projection: camera::Projection,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub light_bind_group: wgpu::BindGroup,
    celestial_meshes: CelestialMeshContainer,
    physics_sim: PhysicsSim,
    camera_controller: CameraController<OrbitCamera>,
    renderer: Renderer,
    celestial_bodies: CelestialBodyContainer,
}



impl State
{
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>
    {
        let renderer_state = RenderState::new(window).await?;
        renderer_state
            .window
            .set_cursor_grab(CursorGrabMode::Locked)?;

        let texture_bind_group_layout = State::create_texture_bind_group_layout(&renderer_state);

        let diffuse_bind_group =
            State::create_diffuse_bind_group(&renderer_state, &texture_bind_group_layout);

        let (shader, line_shader) = State::create_shaders(&renderer_state);

        let space_ship = SpaceshipController::new(InputSettings {
            sensitivity: 0.01,
            speed: 200.0,
        });

        let camera_controller = CameraController::new(OrbitCamera::new(
            Vector3::zeros(),
            10.0,
            Vector3::x_axis().into_inner(),
            Vector3::y_axis().into_inner(),
        ));

        let projection = camera::Projection::new(
            renderer_state.config.width,
            renderer_state.config.height,
            110.0 * math::DEG_TO_RAD as f32,
            0.01,
            1000.0,
        );

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera_controller.camera, &projection);

        let camera_buffer = camera_uniform.create_camera_buffer(&renderer_state);

        let (camera_bind_group, camera_bind_group_layout) =
            CameraUniform::create_camera_bind_group(&camera_buffer, &renderer_state);

        let sphere_lights = &[
            SphereLight::new(
                Point3::new(0.0, 30.0, 0.0),
                Vector4::new(1.0, 1.0, 1.0, 1.0),
                100.0,
            ),
            SphereLight::new(
                Point3::new(30.0, 30.0, 0.0),
                Vector4::new(1.0, 0.0, 0.0, 1.0),
                100.0,
            ),
            SphereLight::new(
                Point3::new(60.0, 30.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 1.0),
                100.0,
            ),
            SphereLight::new(
                Point3::new(90.0, 30.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 1.0),
                100.0,
            ),
        ];

        let sun_lights = &[
            SunLight::new((8.0, -12.0, 3.0), (1.0, 0.8, 0.2, 1.0), 0.5),
            SunLight::new((-8.0, -12.0, -2.0), (0.2, 0.8, 1.0, 1.0), 0.2),
        ];

        let light_uniform = LightUniform::new(sphere_lights, sun_lights);
        let light_buffer = light_uniform.create_light_buffer(&renderer_state);

        let (light_bind_group, light_bind_group_layout) =
            LightUniform::create_light_bind_group(&light_buffer, &renderer_state);

        let render_pipeline_layout =
            renderer_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let depth_texture = texture::Texture::create_depth_texture(
            &renderer_state.device,
            &renderer_state.config,
            "depth_texture",
        );

        let render_pipeline =
            renderer_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),

                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[TextureVertex::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },

                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: renderer_state.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),

                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
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

        let line_render_pipeline = Renderer::create_line_pipeline(
            &renderer_state,
            &texture_bind_group_layout,
            &camera_bind_group_layout,
            &light_bind_group_layout,
            &line_shader,
        );

        let mut rng = ThreadRng::default();
        let mut celestial_bodies = CelestialBodyContainer::new();
        celestial_bodies.generate_planets(50, 1_000_000.0, 50_000.0, &mut rng);

        let mut celestial_meshes = CelestialMeshContainer::new();
        celestial_meshes.add_planets(&celestial_bodies.bodies, &renderer_state);

        let renderer = Renderer;

        let physics_sim = PhysicsSim::new(
            Duration::from_secs_f32(1.0 / 120.0),
            space_ship,
            celestial_bodies.clone(),
        );

        Ok(Self {
            camera_controller,
            celestial_bodies,
            celestial_meshes,
            physics_sim,
            renderer,
            renderer_state,
            render_pipeline,
            line_render_pipeline,
            diffuse_bind_group,
            depth_texture,
            projection,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            light_bind_group,
        })
    }



    fn create_texture_bind_group_layout(renderer: &RenderState) -> BindGroupLayout
    {
        renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    // Diffuse
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
                    // Specular
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            })
    }



    fn create_diffuse_bind_group(
        renderer: &RenderState,
        texture_bind_group_layout: &BindGroupLayout,
    ) -> wgpu::BindGroup
    {
        let diffuse_bytes = include_bytes!("../res/textures/8pxBlocksDiffuse.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            diffuse_bytes,
            "diffuse_atlas",
        )
        .unwrap();

        let specular_bytes = include_bytes!("../res/textures/8pxBlocksSpecular.png");
        let specular_texture = texture::Texture::from_bytes(
            &renderer.device,
            &renderer.queue,
            specular_bytes,
            "specular_atlas",
        )
        .unwrap();

        renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&specular_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&specular_texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
    }



    fn create_shaders(renderer: &RenderState) -> (wgpu::ShaderModule, wgpu::ShaderModule)
    {
        (
            renderer
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
                }),
            renderer
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Line Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("line_shader.wgsl").into()),
                }),
        )
    }



    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.renderer_state.resize(width, height);
        self.projection.resize(width, height);

        if width > 0 && height > 0
        {
            self.depth_texture = texture::Texture::create_depth_texture(
                &self.renderer_state.device,
                &self.renderer_state.config,
                "depth_texture",
            );
        }
    }



    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        let mut vertices = vec![];

        if let Some(player) = self.physics_sim.get_player()
        {
            self.camera_controller.focus_body(&player);
            self.camera_uniform
                .update_view_proj(&self.camera_controller, &self.projection);

            let body = player.body;
            let trajectory = calculate_trajectory(
                self.physics_sim.dt.as_secs_f32(),
                200 * 60 * 10,
                &body,
                &self.celestial_bodies,
            );

            vertices = trajectory
                .iter()
                .map(|p| TextureVertex::new(p.0, [p.1, 0.0]))
                .collect::<Vec<TextureVertex>>();
        }
        self.renderer_state.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        self.renderer.render(
            self,
            self.celestial_meshes.meshes.iter(),
            [MeshBuffer::from_verts(
                &vertices,
                &(0..vertices.len() as u32).collect::<Vec<u32>>(),
                &self.renderer_state,
            )]
            .iter(),
        )
    }



    pub fn handle_input(&mut self, event_loop: &ActiveEventLoop, event: InputEvent)
    {
        if let InputEvent::Keyboard {
            code: KeyCode::Escape,
            pressed: true,
        } = event
        {
            event_loop.exit();
        }
        else
        {
            self.physics_sim.handle_input(&event);
            if let InputEvent::MouseWheel { delta } = event
            {
                self.camera_controller.handle_mouse_scroll(&delta);
            }
        }
    }
}
