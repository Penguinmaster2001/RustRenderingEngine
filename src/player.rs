use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    physics::physics_environment::ForceField,
    player::spaceship_controller::SpaceshipController,
};
use nalgebra::{
    Point3,
    Vector2,
};
use winit::keyboard::KeyCode;



pub mod controller;
pub mod player_controller;
pub mod spaceship_controller;



pub struct Player
{
    pub controller: SpaceshipController,
}



impl Player
{
    pub fn new() -> Self
    {
        Self {
            controller: SpaceshipController::new(InputSettings {
                sensitivity: 0.01,
                speed: 200.0,
            }),
        }
    }



    pub fn update(&mut self, dt: instant::Duration, world: &impl ForceField)
    {
        let force = world.sample_force(*self.get_position()) / self.controller.body.properties.mass;

        self.controller.body.state.add_acceleration(force);
        self.controller.update(dt);
    }



    pub fn get_position(&self) -> &Point3<f32>
    {
        self.controller.body.state.get_pos()
    }
}



impl Default for Player
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl InputHandler for Player
{
    fn handle_mouse_movement(&mut self, delta: Vector2<f32>) -> bool
    {
        self.controller.handle_mouse_movement(delta)
    }



    fn handle_key(&mut self, key: KeyCode, pressed: bool) -> bool
    {
        self.controller.handle_key(key, pressed)
    }
}
