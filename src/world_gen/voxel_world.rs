use crate::{
    chunking::{
        chunk::ChunkContainer,
        chunk_mesh::ChunkMesh,
        chunk_renderer::ChunkRenderer,
    },
    rendering::Renderer,
};
use cgmath::{
    Point3,
    Vector3,
};



pub struct VoxelWorld
{
    chunks: ChunkContainer,
    pub chunk_renderer: ChunkRenderer,
}



impl VoxelWorld
{
    pub fn new() -> Self
    {
        Self {
            chunks: ChunkContainer::new(),
            chunk_renderer: ChunkRenderer::new(),
        }
    }



    pub fn generate_chunks(&mut self, center_pos: &Point3<f32>, renderer: &Renderer)
    {
        let center_chunk = ChunkContainer::world_to_chunk(center_pos);

        let radius = 2;
        for x in -radius..radius
        {
            for z in -radius..radius
            {
                for y in -radius..radius
                {
                    let pos = center_chunk + Vector3::new(x, y, z);
                    if self.chunks.add_at(&pos)
                    {
                        let chunk = self.chunks.get_chunk(&pos).unwrap();

                        let mesh = ChunkMesh::from_chunk(chunk, renderer);

                        self.chunk_renderer.add_chunk(chunk, mesh);
                    }
                }
            }
        }
    }
}
