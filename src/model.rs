use crate::{
    rendering::{
        mesh::MeshBuffer,
        renderer::Renderer,
    },
    texture,
};
use nalgebra::Transform3;
use std::ops::Range;
use wgpu::{
    Buffer,
    util::DeviceExt,
};



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct TransformUniform
{
    pub transform: [[f32; 4]; 4],
}



impl TransformUniform
{
    pub fn new() -> Self
    {
        Self {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }



    pub fn create_buffer(&self, renderer: &Renderer) -> Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("transform_buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }



    pub fn create_empty_buffer(renderer: &Renderer) -> Buffer
    {
        renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("transform_buffer"),
                contents: bytemuck::cast_slice(&[TransformUniform::default()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }



    pub fn create_bind_group(
        transform_buffer: &Buffer,
        renderer: &Renderer,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout)
    {
        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("transform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                }],
                label: Some("transform_bind_group"),
            });

        (bind_group, layout)
    }
}



impl Default for TransformUniform
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl From<Transform3<f32>> for TransformUniform
{
    fn from(value: Transform3<f32>) -> Self
    {
        Self {
            transform: *value.into_inner().as_mut(),
        }
    }
}



#[derive(Default)]
pub struct Model
{
    pub meshes: Vec<MeshBuffer>,
    pub transform: TransformUniform,
}



impl Model
{
    pub fn new<T: Into<TransformUniform>>(meshes: Vec<MeshBuffer>, transform: T) -> Self
    {
        let t = transform.into();
        println!("{:?}", t);

        Self {
            meshes,
            transform: t,
        }
    }
}



pub struct Material
{
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}



pub struct ModelMesh
{
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}



pub trait DrawModel<'a>
{
    fn draw_mesh(
        &mut self,
        mesh: &'a ModelMesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a ModelMesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}



impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup,
    )
    {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }



    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    )
    {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
