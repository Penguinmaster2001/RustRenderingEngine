use crate::chunking::chunk::Chunk;
use crate::chunking::chunk_mesh::ChunkMesh;
use crate::{
    chunking::chunk::ChunkContainer,
    rendering::Renderer,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub struct ChunkRenderer
{
    chunk_meshes: HashMap<Vector3<i64>, ChunkMesh>,
}



impl ChunkRenderer
{
    pub fn from_chunk_container(chunks: &ChunkContainer, renderer: &Renderer) -> Self
    {
        let mut chunk_container = Self {
            chunk_meshes: HashMap::new(),
        };

        for chunk in chunks.chunks.values()
        {
            let mesh = ChunkMesh::from_chunk(chunk, renderer);
            chunk_container.add_chunk(chunk, mesh);
        }

        chunk_container
    }



    pub fn add_chunk(&mut self, chunk: &Chunk, mesh: ChunkMesh)
    {
        self.chunk_meshes.insert(chunk.world_offset, mesh);
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
        for chunk_mesh in chunk_renderer
            .chunk_meshes
            .values()
            .filter(|c| c.index_count > 0)
        {
            self.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
            self.set_index_buffer(chunk_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..chunk_mesh.index_count, 0, 0..1);
        }
    }
}
