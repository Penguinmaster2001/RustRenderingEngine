use crate::chunking::chunk_mesh::ChunkMesh;
use crate::{
    chunking::chunk::ChunkContainer,
    rendering::Renderer,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub struct ChunkRenderer
{
    chunk_meshes: HashMap<Vector3<u32>, ChunkMesh>,
}



impl ChunkRenderer
{
    pub fn from_chunk_container(chunks: &ChunkContainer, renderer: &Renderer) -> Self
    {
        let mut chunk_meshes = HashMap::new();

        for (position, chunk) in &chunks.chunks
        {
            chunk_meshes.insert(*position, ChunkMesh::from_chunk(&chunk, &renderer));
        }

        Self {
            chunk_meshes: chunk_meshes,
        }
    }
}



pub trait DrawChunks<'a>
{
    fn draw_chunks(&mut self, chunk_renderer: &ChunkRenderer);
}



impl<'a, 'b> DrawChunks<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_chunks(&mut self, chunk_renderer: &ChunkRenderer)
    {
        for (_, chunk_mesh) in &chunk_renderer.chunk_meshes
        {
            self.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
            self.set_index_buffer(chunk_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..chunk_mesh.index_count, 0, 0..1);
        }
    }
}
