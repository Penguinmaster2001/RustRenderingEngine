use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    physics::physics_body_state::PhysicsBodyState,
    player::controller::Controller,
    rendering::camera::{
        self,
        Camera,
    },
};
use nalgebra::Vector3;
use std::time::Duration;
use winit::keyboard::KeyCode;



pub struct SpaceshipController
{
    controller: Controller<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    physics_state: PhysicsBodyState<f32>,
    input_settings: InputSettings,
}



impl SpaceshipController
{
    pub fn new(input_settings: InputSettings) -> Self
    {
        Self {
            controller: Controller::new(),
            forward: Vector3::x_axis().into_inner(),
            up: Vector3::y_axis().into_inner(),
            physics_state: PhysicsBodyState::new(),
            input_settings,
        }
    }



    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration)
    {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * self.controller.get_forward() * self.input_settings.speed * dt;
        camera.position += right * self.controller.get_left() * self.input_settings.speed * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += self.controller.get_up() * self.input_settings.speed * dt;

        // Rotate
        camera.yaw += self.controller.get_rotate_right() * self.input_settings.sensitivity * dt;
        camera.pitch += -self.controller.get_rotate_up() * self.input_settings.sensitivity * dt;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -camera::SAFE_FRAC_PI_2
        {
            camera.pitch = -camera::SAFE_FRAC_PI_2;
        }
        else if camera.pitch > camera::SAFE_FRAC_PI_2
        {
            camera.pitch = camera::SAFE_FRAC_PI_2;
        }

        self.controller.reset_rotation();
    }
}



impl InputHandler for SpaceshipController
{
    fn handle_mouse_movement(&mut self, mouse_dx: f64, mouse_dy: f64) -> bool
    {
        self.controller.rotate_right(mouse_dx as f32);
        self.controller.rotate_up(mouse_dy as f32);
        true
    }



    fn handle_key(&mut self, key: KeyCode, pressed: bool) -> bool
    {
        let amount = if pressed { 1.0 } else { 0.0 };
        match key
        {
            KeyCode::KeyW | KeyCode::ArrowUp =>
            {
                self.controller.forward(amount);
            }
            KeyCode::KeyS | KeyCode::ArrowDown =>
            {
                self.controller.forward(-amount);
            }
            KeyCode::KeyA | KeyCode::ArrowLeft =>
            {
                self.controller.right(-amount);
            }
            KeyCode::KeyD | KeyCode::ArrowRight =>
            {
                self.controller.right(amount);
            }
            KeyCode::Space =>
            {
                self.controller.up(amount);
            }
            KeyCode::ShiftLeft =>
            {
                self.controller.up(-amount);
            }
            _ =>
            {
                return false;
            }
        };

        true
    }
}
