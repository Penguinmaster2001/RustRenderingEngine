use crate::{
    rendering::{
        mesh::MeshBuffer,
        renderer::Renderer,
    },
    texture,
};
use nalgebra::Transform3;
use std::ops::Range;
use wgpu::Buffer;



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



    pub fn padded_uniform_size(renderer: &Renderer) -> u64
    {
        let uniform_size = std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress;
        let min_alignment: u64 = renderer.device.limits().min_uniform_buffer_offset_alignment as _;
        (uniform_size + min_alignment - 1) & !(min_alignment - 1) // ceil to alignment
    }



    pub fn create_buffer(renderer: &Renderer) -> Buffer
    {
        let max_models_per_frame = 64;
        let uniform_size = std::mem::size_of::<TransformUniform>() as wgpu::BufferAddress;
        let min_alignment: u64 = renderer.device.limits().min_uniform_buffer_offset_alignment as _;
        let padded_uniform_size = (uniform_size + min_alignment - 1) & !(min_alignment - 1); // ceil to alignment

        let buffer_size = max_models_per_frame * padded_uniform_size;
        renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("transforms_array_buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }



    pub fn create_bind_group(
        transform_buffer: &Buffer,
        renderer: &Renderer,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout)
    {
        let size = wgpu::BufferSize::new(TransformUniform::padded_uniform_size(renderer));

        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("transform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: size,
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
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: transform_buffer,
                        offset: 0,
                        size,
                    }),
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
        Self {
            meshes,
            transform: transform.into(),
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
