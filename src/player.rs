use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    player::spaceship_controller::SpaceshipController,
    rendering::camera::FreeCamera,
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
    pub camera: FreeCamera,
    controller: SpaceshipController,
}



impl Player
{
    pub fn new<T: Into<Point3<f32>>>(position: T) -> Self
    {
        let position = position.into();
        Self {
            camera: FreeCamera::new(
                position,
                Vector3::x_axis().into_inner(),
                Vector3::y_axis().into_inner(),
            ),
            controller: SpaceshipController::new(InputSettings {
                sensitivity: 1.5,
                speed: 50.0,
            }),
        }
    }



    pub fn update(&mut self, dt: instant::Duration, world: &VoxelWorld)
    {
        let force = world.sample_force(self.get_position()) / 2.0;
        self.controller.physics_state.add_acceleration(force);
        self.controller.update(dt);
        self.controller.update_camera(&mut self.camera);
    }



    pub(crate) fn get_position(&self) -> &Point3<f32>
    {
        self.controller.physics_state.get_pos()
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
