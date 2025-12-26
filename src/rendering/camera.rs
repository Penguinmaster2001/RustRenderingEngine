use cgmath::*;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;
use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::keyboard::KeyCode;

use crate::player::controller::Controller;



#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);



const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform
{
    view_projection: [[f32; 4]; 4],
    view_position: [f32; 4],
}



impl CameraUniform
{
    pub fn new() -> Self
    {
        Self {
            view_position: [0.0; 4],
            view_projection: cgmath::Matrix4::identity().into(),
        }
    }



    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection)
    {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_projection = (projection.calc_matrix() * camera.calc_matrix()).into()
    }
}



#[derive(Debug)]
pub struct Camera
{
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}



impl Camera
{
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self
    {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }



    pub fn calc_matrix(&self) -> Matrix4<f32>
    {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}



pub struct Projection
{
    aspect: f32,
    fov_y: Rad<f32>,
    z_near: f32,
    z_far: f32,
}



impl Projection
{
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self
    {
        Self {
            aspect: width as f32 / height as f32,
            fov_y: fovy.into(),
            z_near: znear,
            z_far: zfar,
        }
    }



    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.aspect = width as f32 / height as f32;
    }



    pub fn calc_matrix(&self) -> Matrix4<f32>
    {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fov_y, self.aspect, self.z_near, self.z_far)
    }
}



#[derive(Debug)]
pub struct CameraController
{
    controller: Controller<f32>,
    speed: f32,
    sensitivity: f32,
}



impl CameraController
{
    pub fn new(speed: f32, sensitivity: f32) -> Self
    {
        Self {
            controller: Controller::new(),
            speed,
            sensitivity,
        }
    }



    pub fn handle_key(&mut self, key: KeyCode, pressed: bool) -> bool
    {
        let amount = if pressed { 1.0 } else { 0.0 };
        match key
        {
            KeyCode::KeyW | KeyCode::ArrowUp =>
            {
                self.controller.forward(amount);
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown =>
            {
                self.controller.forward(-amount);
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft =>
            {
                self.controller.right(-amount);
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight =>
            {
                self.controller.right(amount);
                true
            }
            KeyCode::Space =>
            {
                self.controller.up(amount);
                true
            }
            KeyCode::ShiftLeft =>
            {
                self.controller.up(-amount);
                true
            }
            _ => false,
        }
    }



    pub fn handle_mouse(&mut self, mouse_dx: f64, mouse_dy: f64)
    {
        self.controller.rotate_right(mouse_dx as f32);
        self.controller.rotate_up(mouse_dy as f32);
    }



    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration)
    {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * self.controller.get_forward() * self.speed * dt;
        camera.position += right * self.controller.get_left() * self.speed * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += self.controller.get_up() * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.controller.get_rotate_right()) * self.sensitivity * dt;
        camera.pitch += Rad(-self.controller.get_rotate_up()) * self.sensitivity * dt;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2)
        {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        }
        else if camera.pitch > Rad(SAFE_FRAC_PI_2)
        {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }

        self.controller.reset_rotation();
    }
}
