use crate::rendering::Renderer;
use bytemuck::Zeroable;
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
    pub fn new<P: Into<[f32; 3]>, C: Into<[f32; 4]>>(position: P, color: C, intensity: f32)
    -> Self
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
    pub fn new<D: Into<[f32; 3]>, C: Into<[f32; 4]>>(direction: D, color: C, intensity: f32)
    -> Self
    {
        Self {
            direction: direction.into(),
            _padding0: 0,
            color: color.into(),
            intensity: intensity,
            _padding1: [0, 0, 0],
        }
    }
}



const MAX_SPHERE_LIGHTS: usize = 16;
const MAX_SUN_LIGHTS: usize = 2;



#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform
{
    sphere_lights: [SphereLight; MAX_SPHERE_LIGHTS],
    sphere_light_count: u32,
    _padding0: [u32; 3],
    sun_lights: [SunLight; MAX_SUN_LIGHTS],
    sun_light_count: u32,
    _padding1: [u32; 3],
}



impl LightUniform
{
    pub fn new(sphere_lights: &[SphereLight], sun_lights: &[SunLight]) -> Self
    {
        let mut sphere_light_uniforms = [SphereLight::zeroed(); MAX_SPHERE_LIGHTS];
        let mut sphere_light_count = 0;
        for light in sphere_lights
        {
            if sphere_light_count >= MAX_SPHERE_LIGHTS
            {
                break;
            }

            sphere_light_uniforms[sphere_light_count] = light.clone();
            sphere_light_count += 1;
        }

        let mut sun_light_uniforms = [SunLight::zeroed(); MAX_SUN_LIGHTS];
        let mut sun_light_count = 0;
        for light in sun_lights
        {
            if sun_light_count >= MAX_SUN_LIGHTS
            {
                break;
            }

            sun_light_uniforms[sun_light_count] = light.clone();
            sun_light_count += 1;
        }

        Self {
            sphere_lights: sphere_light_uniforms,
            sphere_light_count: sphere_light_count as u32,
            _padding0: [0; 3],
            sun_lights: sun_light_uniforms,
            sun_light_count: sun_light_count as u32,
            _padding1: [0; 3],
        }
    }



    pub fn create_light_buffer(&self, renderer: &Renderer) -> Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("light_buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }



    pub fn create_light_bind_group(
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
                    label: Some("light_bind_group_layout"),
                });

        let light_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &light_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding,
                    resource: light_buffer.as_entire_binding(),
                }],
                label: Some("light_bind_group"),
            });

        (light_bind_group, light_bind_group_layout)
    }
}
