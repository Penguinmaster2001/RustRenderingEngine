use crate::{
    chunking::{
        chunk::ChunkContainer,
        chunk_mesh::ChunkMesh,
        chunk_renderer::ChunkRenderer,
    },
    rendering::Renderer,
};
use cgmath::Point3;



pub struct VoxelWorld
{
    chunks: ChunkContainer,
    pub chunk_renderer: ChunkRenderer,
}



impl VoxelWorld
{
    pub fn new(renderer: &Renderer) -> Self
    {
        let mut chunks = ChunkContainer::new();
        chunks.add_at(0, 0, 0);
        chunks.add_at(1, 0, 0);
        chunks.add_at(1, 0, 1);

        let chunk_renderer = ChunkRenderer::from_chunk_container(&chunks, &renderer);

        Self {
            chunks,
            chunk_renderer,
        }
    }



    pub fn generate_chunks(&mut self, center_pos: Point3<f32>, renderer: &Renderer)
    {
        let (c_x, c_y, c_z) =
            ChunkContainer::world_to_chunk(center_pos.x, center_pos.y, center_pos.z);

        let radius = 2;
        for x in -radius..radius
        {
            for z in -radius..radius
            {
                for y in -radius..radius
                {
                    if self.chunks.add_at(x + c_x, y + c_y, z + c_z)
                    {
                        let chunk = self.chunks.get_chunk(x + c_x, y + c_y, z + c_z).unwrap();

                        let mesh = ChunkMesh::from_chunk(chunk, renderer);

                        self.chunk_renderer.add_chunk(chunk, mesh);
                    }
                }
            }
        }
    }
}
