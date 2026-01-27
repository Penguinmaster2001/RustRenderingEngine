use crate::{
    player::spaceship_controller::SpaceshipController,
    rendering::camera::{
        Camera,
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
