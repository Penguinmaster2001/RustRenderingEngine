use crate::rendering::camera::Camera;



pub struct CameraController<C: Camera>
{
    camera: C,
}
