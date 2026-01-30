use crate::{
    rendering::renderer::Renderer,
    vertex::Vertex,
};
use wgpu::util::DeviceExt;



pub mod primatives;



pub struct MeshData<V: Vertex>
{
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
    pub vertex_count: u32,
}



pub struct MeshBuffer
{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}



impl MeshBuffer
{
    pub fn from_verts<V: Vertex + bytemuck::NoUninit>(
        vertices: &[V],
        indices: &[u32],
        renderer: &Renderer,
    ) -> Self
    {
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("chunk Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as _,
        }
    }



    pub fn from_data<V: Vertex + bytemuck::NoUninit>(
        chunk_mesh_data: &MeshData<V>,
        renderer: &Renderer,
    ) -> Self
    {
        MeshBuffer::from_verts(
            &chunk_mesh_data.vertices,
            &chunk_mesh_data.indices,
            renderer,
        )
    }
}
