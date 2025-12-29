use crate::rendering::Renderer;
use cgmath::{
    Point3,
    Vector4,
};
use wgpu::{
    Buffer,
    util::DeviceExt,
};



#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SphereLight
{
    position: [f32; 3],
    _padding0: u32,
    color: [f32; 4],
    intensity: f32,
    _padding1: [u32; 3],
}



impl SphereLight
{
    pub fn new(position: Point3<f32>, color: Vector4<f32>, intensity: f32) -> Self
    {
        Self {
            position: position.into(),
            _padding0: 0,
            color: color.into(),
            intensity: intensity,
            _padding1: [0, 0, 0],
        }
    }



    pub fn create_sphere_light_buffer(lights: &[SphereLight], renderer: &Renderer) -> Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("SphereLightVB"),
                contents: bytemuck::cast_slice(lights),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }



    pub fn create_sphere_light_bind_group(
        light_buffer: Buffer,
        renderer: &Renderer,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout)
    {
        let binding = 0;

        let light_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("sphere_light_bind_group_layout"),
                });

        let light_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &light_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding,
                    resource: light_buffer.as_entire_binding(),
                }],
                label: Some("sphere_light_bind_group"),
            });

        (light_bind_group, light_bind_group_layout)
    }
}



#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SunLight
{
    direction: [f32; 3],
    _padding0: u32,
    color: [f32; 4],
    intensity: f32,
    _padding1: [u32; 3],
}



impl SunLight
{
    pub fn create_sphere_light_buffer(lights: &[SunLight], renderer: &Renderer) -> Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("SunLightVB"),
                contents: bytemuck::cast_slice(lights),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }
}
