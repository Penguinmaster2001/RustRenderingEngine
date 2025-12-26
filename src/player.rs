use crate::rendering::camera::{
    Camera,
    CameraController,
};
use cgmath::Point3;
use winit::{
    event::MouseScrollDelta,
    keyboard::KeyCode,
};



pub struct Player
{
    pub position: Point3<f32>,
    pub camera: Camera,
    pub camera_controller: CameraController,
}



impl Player
{
    pub fn new<T: Into<Point3<f32>>>(position: T) -> Self
    {
        let position = position.into();
        Self {
            position,
            camera: Camera::new(position, cgmath::Deg(-90.0), cgmath::Deg(-20.0)),
            camera_controller: CameraController::new(25.0, 1.8),
        }
    }



    pub fn handle_mouse(&mut self, mouse_dx: f64, mouse_dy: f64)
    {
        self.camera_controller.handle_mouse(mouse_dx, mouse_dy);
    }



    pub fn handle_key(&mut self, key: KeyCode, pressed: bool)
    {
        self.camera_controller.handle_key(key, pressed);
    }



    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta)
    {
        self.camera_controller.handle_scroll(delta);
    }



    pub fn update(&mut self, dt: instant::Duration)
    {
        self.camera_controller.update_camera(&mut self.camera, dt);
    }
}
