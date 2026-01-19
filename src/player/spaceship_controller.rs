use crate::{
    input::{
        InputHandler,
        input_settings::InputSettings,
    },
    physics::physics_body_state::PhysicsBodyState,
    player::controller::Controller,
    rendering::camera::FreeCamera,
};
use nalgebra::{
    Unit,
    UnitQuaternion,
    Vector3,
};
use std::time::Duration;
use winit::keyboard::KeyCode;



pub struct SpaceshipController
{
    controller: Controller<f32>,
    forward: Unit<Vector3<f32>>,
    right: Unit<Vector3<f32>>,
    up: Unit<Vector3<f32>>,
    physics_state: PhysicsBodyState<f32>,
    input_settings: InputSettings,
}



impl SpaceshipController
{
    pub fn new(input_settings: InputSettings) -> Self
    {
        Self {
            controller: Controller::new(),
            forward: Vector3::x_axis(),
            right: Vector3::z_axis(),
            up: Vector3::y_axis(),
            physics_state: PhysicsBodyState::new(),
            input_settings,
        }
    }



    pub fn update_camera(&mut self, camera: &mut FreeCamera, dt: Duration)
    {
        let dt = dt.as_secs_f32();

        let translation = (self.forward.into_inner()
            * self.controller.get_forward()
            * self.input_settings.speed
            * dt)
            + (self.right.into_inner()
                * self.controller.get_left()
                * self.input_settings.speed
                * dt)
            + (self.up.into_inner() * self.controller.get_up() * self.input_settings.speed * dt);

        self.physics_state.translate(translation);

        let right_rotation = UnitQuaternion::from_axis_angle(
            &self.up,
            -self.controller.get_rotate_right() * self.input_settings.sensitivity * dt,
        );

        let up_rotation = UnitQuaternion::from_axis_angle(
            &self.right,
            -self.controller.get_rotate_up() * self.input_settings.sensitivity * dt,
        );

        self.forward = Unit::new_normalize(
            up_rotation.transform_vector(&right_rotation.transform_vector(&self.forward)),
        );
        self.up = Unit::new_normalize(up_rotation.transform_vector(&self.up));
        self.right = Unit::new_normalize(right_rotation.transform_vector(&self.right));

        camera.forward = self.forward.into_inner();
        camera.up = self.up.into_inner();
        camera.position = self.physics_state.get_pos().clone();

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
