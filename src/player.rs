use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    player::spaceship_controller::SpaceshipController,
    rendering::camera::Camera,
    world_gen::voxel_world::VoxelWorld,
};
use cgmath::{
    Point3,
    Vector3,
    Zero,
};
use winit::keyboard::KeyCode;



pub mod controller;
pub mod player_controller;
pub mod spaceship_controller;



pub struct Player
{
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub camera: Camera,
    controller: SpaceshipController,
}



impl Player
{
    pub fn new<T: Into<Point3<f32>>>(position: T) -> Self
    {
        let position = position.into();
        Self {
            position,
            velocity: Vector3::zero(),
            camera: Camera::new(position, cgmath::Deg(-90.0), cgmath::Deg(-20.0)),
            controller: SpaceshipController::new(InputSettings {
                sensitivity: 1.5,
                speed: 25.0,
            }),
        }
    }



    pub fn update(&mut self, dt: instant::Duration, _world: &VoxelWorld)
    {
        self.update_camera(dt);
    }



    fn update_camera(&mut self, dt: instant::Duration)
    {
        self.controller.update_camera(&mut self.camera, dt);
    }
}



impl InputHandler for Player
{
    fn handle_mouse_movement(&mut self, mouse_dx: f64, mouse_dy: f64) -> bool
    {
        self.controller.handle_mouse_movement(mouse_dx, mouse_dy)
    }



    fn handle_key(&mut self, key: KeyCode, pressed: bool) -> bool
    {
        self.controller.handle_key(key, pressed)
    }
}
