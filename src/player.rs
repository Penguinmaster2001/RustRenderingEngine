use crate::{
    input::InputHandler,
    player::controller::Controller,
    rendering::camera::{
        Camera,
        CameraController,
    },
    world_gen::voxel_world::VoxelWorld,
};
use cgmath::{
    Point3,
    Vector3,
    Zero,
};
use winit::keyboard::KeyCode;



pub mod controller;



pub struct Player
{
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub camera: Camera,
    pub camera_controller: CameraController,
    controller: Controller<f32>,
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
            camera_controller: CameraController::new(15.0, 1.8),
            controller: Controller::new(),
        }
    }



    pub fn update(&mut self, dt: instant::Duration, world: &VoxelWorld)
    {
        self.update_camera(dt);
    }



    fn update_camera(&mut self, dt: instant::Duration)
    {
        self.camera_controller.update_camera(&mut self.camera, dt);
    }
}



impl InputHandler for Player
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
