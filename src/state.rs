use crate::{
    chaos::{
        ChaosConfig,
        chaos_thread::{
            ChaosCommand,
            ChaosHandle,
        },
        heuristics,
        maps::polynomial_maps::{
            ChaoticPolynomialMap,
            PolynomialMap,
        },
    },
    input::{
        InputEvent,
        InputHandler,
        input_settings::InputSettings,
    },
    math,
    model::{
        Model,
        TransformUniform,
    },
    physics::physics_sim::PhysicsSim,
    player::spaceship_controller::SpaceshipController,
    rendering::{
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
        mesh::{
            MeshBuffer,
            MeshData,
        },
        render_pass_data::{
            GeometryGroup,
            RenderData,
        },
        renderer::Renderer,
    },
    texture,
    vertex::{
        ModelVertex,
        Vertex,
    },
};
use nalgebra::Vector3;
use std::{
    sync::Arc,
    vec,
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



const DIM: usize = 3;



pub struct State
{
    frame_num: u32,
    projection: camera::Projection,
    camera_uniform: camera::CameraUniform,
    render_data: RenderData,
    physics_sim: PhysicsSim,
    camera_controller: CameraController<OrbitCamera>,
    pub renderer: Renderer,
    cursor_locked: bool,
    points: ChaosHandle<f32, PolynomialMap<f32, DIM>, DIM>,
}



impl State
{
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>
    {
        let renderer = Renderer::new(window).await?;

        let texture_bind_group_layout = State::create_texture_bind_group_layout(&renderer);

        let diffuse_bind_group =
            State::create_diffuse_bind_group(&renderer, &texture_bind_group_layout);

        let [point_shader] = renderer.create_shaders(&[include_str!("point_shader.wgsl")]);

        let spaceship = SpaceshipController::new(InputSettings {
            sensitivity: 0.003,
            speed: 5.0,
        });

        let camera_controller = CameraController::new(OrbitCamera::new(
            Vector3::zeros(),
            10.0,
            Vector3::x_axis().into_inner(),
            Vector3::y_axis().into_inner(),
        ));

        let projection = camera::Projection::new(
            renderer.config.width,
            renderer.config.height,
            50.0 * math::DEG_TO_RAD as f32,
            0.001,
            100.0,
        );

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera_controller.camera, &projection);

        let camera_buffer = camera_uniform.create_camera_buffer(&renderer);

        let (camera_bind_group, camera_bind_group_layout) =
            CameraUniform::create_camera_bind_group(&camera_buffer, &renderer);

        let sphere_lights = [
            SphereLight::new((00.0, 30.0, 0.0), (1.0, 1.0, 1.0, 1.0), 100.0),
            SphereLight::new((30.0, 30.0, 0.0), (1.0, 0.0, 0.0, 1.0), 100.0),
            SphereLight::new((60.0, 30.0, 0.0), (0.0, 1.0, 0.0, 1.0), 100.0),
            SphereLight::new((90.0, 30.0, 0.0), (0.0, 0.0, 1.0, 1.0), 100.0),
        ];
        let sun_lights = [
            SunLight::new((8.0, -12.0, 3.0), (1.0, 0.8, 0.2, 1.0), 0.5),
            SunLight::new((-8.0, -12.0, -2.0), (0.2, 0.8, 1.0, 1.0), 0.2),
        ];

        let light_uniform = LightUniform::new(&sphere_lights, &sun_lights);
        let light_buffer = light_uniform.create_light_buffer(&renderer);

        let (light_bind_group, light_bind_group_layout) =
            LightUniform::create_light_bind_group(&light_buffer, &renderer);

        let transform_buffer = TransformUniform::create_buffer(&renderer);
        let (transform_bind_group, transform_bind_group_layout) =
            TransformUniform::create_bind_group(&transform_buffer, &renderer);

        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                        &transform_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let depth_texture = texture::Texture::create_depth_texture(
            &renderer.device,
            &renderer.config,
            "depth_texture",
        );

        let point_render_pipeline = renderer.create_render_pipeline(
            "point_render_pipeline",
            &render_pipeline_layout,
            &[ModelVertex::desc()],
            wgpu::PrimitiveTopology::PointList,
            &point_shader,
        );

        let physics_sim = PhysicsSim::new_physics_sim(spaceship);

        let spaceship_model = Model::new(
            vec![MeshBuffer::from_data(
                &MeshData::new_screen_quad(),
                &renderer,
            )],
            TransformUniform::default(),
        );

        let render_data = RenderData::new(
            camera_buffer,
            transform_buffer,
            vec![
                diffuse_bind_group,
                camera_bind_group,
                light_bind_group,
                transform_bind_group,
            ],
            vec![point_render_pipeline],
            vec![GeometryGroup {
                models: vec![spaceship_model],
                pipeline_id: 0,
            }],
            depth_texture,
        );

        let points = ChaosHandle::new_chaos_thread(ChaosConfig {
            max_iterations: None,
            points_generator: ChaoticPolynomialMap::new_chaotic_polynomial_map,
            chaos_heuristic: |p| heuristics::not_divergent_or_collapsed(p, 0.2..200.0, 0.05),
        });

        Ok(Self {
            frame_num: 0,
            camera_controller,
            points,
            physics_sim,
            renderer,
            cursor_locked: false,
            projection,
            camera_uniform,
            render_data,
        })
    }



    fn create_texture_bind_group_layout(renderer: &Renderer) -> BindGroupLayout
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
        renderer: &Renderer,
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



    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.renderer.resize(width, height);
        self.projection.resize(width, height);

        if width > 0 && height > 0
        {
            self.render_data.depth_texture = texture::Texture::create_depth_texture(
                &self.renderer.device,
                &self.renderer.config,
                "depth_texture",
            );
        }
    }



    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        if let Some(mut player) = self.physics_sim.get_state()
        {
            player.input_settings.speed = 50.0 * self.camera_controller.camera.distance;
            self.camera_controller.focus_body(&player);
            self.camera_uniform
                .update_view_proj(&self.camera_controller, &self.projection);
        }

        self.renderer.queue.write_buffer(
            &self.render_data.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        if let Some(points) = self.points.get_state()
        {
            self.render_data.geometries[0].models[0].meshes[0] = MeshBuffer::from_points(
                &points
                    .points
                    .iter()
                    .map(|p| ModelVertex {
                        position: [p[0], p[1], if DIM >= 3 { p[2] } else { 0.0 }],
                        tex_coords: [0.0, 0.0],
                        normal: [1.0, 0.0, 0.0],
                    })
                    .collect::<Vec<_>>(),
                &self.renderer,
            );
        }

        self.frame_num += 1;
        self.renderer.render(&self.render_data)
    }



    pub fn handle_input(&mut self, _: &ActiveEventLoop, event: InputEvent)
    {
        if let InputEvent::Keyboard {
            code: KeyCode::Escape,
            pressed: true,
        } = event
        {
            #[expect(unused_must_use)]
            if self.cursor_locked
            {
                self.cursor_locked = false;
                self.renderer.window.set_cursor_grab(CursorGrabMode::None);
            }
            else
            {
                self.cursor_locked = true;
                self.renderer.window.set_cursor_grab(CursorGrabMode::Locked);
            }
        }
        else if self.cursor_locked
        {
            self.physics_sim.send(event);
            if let InputEvent::MouseWheel { delta } = event
            {
                self.camera_controller.handle_mouse_scroll(&delta);
            }
            else if let InputEvent::Keyboard {
                code,
                pressed: true,
            } = event
            {
                match code
                {
                    KeyCode::KeyP => self.points.send(ChaosCommand::CreateNew),
                    KeyCode::KeyR => self.points.send(ChaosCommand::RestartCurrent),
                    KeyCode::KeyK => self.points.send(ChaosCommand::Pause),
                    KeyCode::KeyL => self.points.send(ChaosCommand::Step),
                    _ => (),
                }
            }
        }
    }
}
