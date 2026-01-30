use crate::{
    chunking::blocks::BlockType,
    texture::Texture,
};
use nalgebra::Vector3;



pub trait Vertex
{
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}



#[rustfmt::skip]
pub const UVS: [[f32; 2]; 4] = [
    [1.0, 0.0],
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 1.0],
];



impl TextureVertex
{
    pub fn new<P: Into<[f32; 3]>, U: Into<[f32; 2]>>(pos: P, uv: U) -> Self
    {
        Self {
            position: pos.into(),
            tex_coords: uv.into(),
        }
    }



    pub fn from_vector_and_block(v: Vector3<f32>, i: usize, block: &BlockType) -> Self
    {
        Self {
            position: [v.x, v.y, v.z],
            tex_coords: Texture::get_uvs(block, i),
        }
    }
}



impl Vertex for TextureVertex
{
    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}



impl Vertex for ModelVertex
{
    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
