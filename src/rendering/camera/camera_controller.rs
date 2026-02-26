use crate::{
    input::InputHandler,
    player::spaceship_controller::SpaceshipController,
    rendering::camera::{
        Camera,
        FreeCamera,
        OrbitCamera,
    },
};



pub struct CameraController<C: Camera>
{
    pub camera: C,
}



impl<C: Camera> CameraController<C>
{
    pub fn new(camera: C) -> Self
    {
        Self { camera }
    }
}



impl CameraController<OrbitCamera>
{
    pub fn focus_body(&mut self, body: &SpaceshipController)
    {
        self.camera.target = *body.body.state.get_pos();
        self.camera.forward = body.forward.into_inner();
        self.camera.up = body.up.into_inner();
    }
}



impl InputHandler for CameraController<OrbitCamera>
{
    fn handle_mouse_scroll(&mut self, delta: &winit::event::MouseScrollDelta) -> bool
    {
        let delta = match delta
        {
            winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
        };

        self.camera.distance = (self.camera.distance + (2.0 * -delta)).clamp(10.0, 20000.0);
        true
    }
}



impl InputHandler for CameraController<FreeCamera>
{
    fn handle_key(&mut self, _key: winit::keyboard::KeyCode, _pressed: bool) -> bool
    {
        false
    }



    fn handle_mouse_movement(&mut self, _delta: nalgebra::Vector2<f32>) -> bool
    {
        false
    }
}



impl<C: Camera> Camera for CameraController<C>
{
    fn get_position(&self) -> &nalgebra::Point3<f32>
    {
        self.camera.get_position()
    }



    fn calc_matrix(&self) -> nalgebra::Matrix4<f32>
    {
        self.camera.calc_matrix()
    }
}
