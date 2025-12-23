use cgmath::Vector3;



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
const UVS: [[f32; 2]; 4] = [
    [1.0, 0.0],
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 1.0],
];



impl TextureVertex
{
    pub fn from_vector(v: Vector3<f32>, i: usize) -> Self
    {
        Self {
            position: [v.x, v.y, v.z],
            tex_coords: UVS[i],
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
