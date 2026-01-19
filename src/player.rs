use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    math,
    player::spaceship_controller::SpaceshipController,
    rendering::camera::{
        FreeCamera,
        UprightCamera,
    },
    world_gen::voxel_world::VoxelWorld,
};
use nalgebra::{
    Point3,
    Vector3,
};
use winit::keyboard::KeyCode;



pub mod controller;
pub mod player_controller;
pub mod spaceship_controller;



pub struct Player
{
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub camera: FreeCamera,
    controller: SpaceshipController,
}



impl Player
{
    pub fn new<T: Into<Point3<f32>>>(position: T) -> Self
    {
        let position = position.into();
        Self {
            position,
            velocity: Vector3::zeros(),
            camera: FreeCamera::new(
                position,
                Vector3::x_axis().into_inner(),
                Vector3::y_axis().into_inner(),
            ),
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
