use crate::rendering::renderer::Renderer;
use nalgebra::{
    Matrix,
    Matrix4,
    Perspective3,
    Point3,
    Vector3,
    matrix,
    vector,
};
use std::f32::consts::FRAC_PI_2;
use wgpu::{
    Buffer,
    util::DeviceExt,
};



pub mod camera_controller;



pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = matrix![
    1.0, 0.0, 0.0, 0.0;
    0.0, 1.0, 0.0, 0.0;
    0.0, 0.0, 0.5, 0.0;
    0.0, 0.0, 0.5, 1.0;
];



pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;



pub trait Camera
{
    fn get_position(&self) -> &Point3<f32>;



    fn calc_matrix(&self) -> Matrix4<f32>;
}



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform
{
    view_projection: [[f32; 4]; 4],
    inv_view_projection: [[f32; 4]; 4],
    view_position: [f32; 4],
    resolution: [u32; 4],
}



impl CameraUniform
{
    pub fn new() -> Self
    {
        Self {
            view_position: [0.0; 4],
            view_projection: Matrix4::identity().into(),
            inv_view_projection: Matrix4::identity().into(),
            resolution: [0; 4],
        }
    }



    pub fn update_view_proj<T: Camera>(
        &mut self,
        camera: &T,
        projection: &Projection,
        resolution: [u32; 2],
    )
    {
        let proj = projection.calc_matrix() * camera.calc_matrix();
        self.view_position = camera.get_position().to_homogeneous().into();
        self.view_projection = proj.into();
        self.inv_view_projection = proj.try_inverse().unwrap().into();
        self.resolution = [resolution[0], resolution[1], 0, 0];
    }



    pub fn create_camera_buffer(&self, renderer: &Renderer) -> wgpu::Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }



    pub fn create_camera_bind_group(
        camera_buffer: &Buffer,
        renderer: &Renderer,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout)
    {
        let camera_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });

        let camera_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        (camera_bind_group, camera_bind_group_layout)
    }
}



impl Default for CameraUniform
{
    fn default() -> Self
    {
        Self::new()
    }
}



#[derive(Debug)]
pub struct UprightCamera
{
    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}



impl UprightCamera
{
    pub fn new<V: Into<Point3<f32>>, Y: Into<f32>, P: Into<f32>>(
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
}



impl Camera for UprightCamera
{
    fn get_position(&self) -> &Point3<f32>
    {
        &self.position
    }



    fn calc_matrix(&self) -> Matrix4<f32>
    {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        Matrix::look_at_rh(
            &self.position,
            &(self.position + vector!(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw)),
            &Vector3::y_axis().into_inner(),
        )
    }
}



pub struct OrbitCamera
{
    pub target: Point3<f32>,
    pub distance: f32,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
}



impl OrbitCamera
{
    pub fn new<V: Into<Point3<f32>>, F: Into<Vector3<f32>>, U: Into<Vector3<f32>>>(
        position: V,
        distance: f32,
        forward: F,
        up: U,
    ) -> Self
    {
        Self {
            target: position.into(),
            distance,
            forward: forward.into(),
            up: up.into(),
        }
    }
}



impl Camera for OrbitCamera
{
    fn get_position(&self) -> &Point3<f32>
    {
        &self.target
    }



    fn calc_matrix(&self) -> Matrix4<f32>
    {
        Matrix::look_at_rh(
            &(self.target - self.forward * self.distance),
            &self.target,
            &self.up,
        )
    }
}



#[derive(Debug)]
pub struct FreeCamera
{
    pub position: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
}



impl FreeCamera
{
    pub fn new<V: Into<Point3<f32>>, F: Into<Vector3<f32>>, U: Into<Vector3<f32>>>(
        position: V,
        forward: F,
        up: U,
    ) -> Self
    {
        Self {
            position: position.into(),
            forward: forward.into(),
            up: up.into(),
        }
    }
}



impl Camera for FreeCamera
{
    fn get_position(&self) -> &Point3<f32>
    {
        &self.position
    }



    fn calc_matrix(&self) -> Matrix4<f32>
    {
        Matrix::look_at_rh(&self.position, &(self.position + self.forward), &self.up)
    }
}



pub struct Projection
{
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}



impl Projection
{
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self
    {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }



    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.aspect = width as f32 / height as f32;
    }



    pub fn calc_matrix(&self) -> Matrix4<f32>
    {
        OPENGL_TO_WGPU_MATRIX
            * Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar).as_matrix()
    }
}
