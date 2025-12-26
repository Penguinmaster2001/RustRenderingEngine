use cgmath::{
    Point3,
    Vector3,
};

use crate::{
    chunking::{
        blocks::Block,
        chunk::{
            Chunk,
            ChunkContainer,
        },
        chunk_mesh::ChunkMesh,
        chunk_renderer::ChunkRenderer,
    },
    player::Player,
    rendering::Renderer,
    world_gen::world_generator::WorldGenerator,
};



pub struct VoxelWorld
{
    chunks: ChunkContainer,
    pub chunk_renderer: ChunkRenderer,
    generator: WorldGenerator,
}



impl VoxelWorld
{
    pub fn new() -> Self
    {
        Self {
            chunks: ChunkContainer::new(),
            chunk_renderer: ChunkRenderer::new(),
            generator: WorldGenerator::new(20, 6),
        }
    }



    pub fn update(&mut self, player: &Player, renderer: &Renderer, dt: instant::Duration)
    {
        self.generator.generate_chunks(&player.position, 6);

        let generated_chunks = self.generator.drain_results();

        for (chunk, mesh_data) in generated_chunks
        {
            let mesh = ChunkMesh::from_data(&mesh_data, renderer);
            self.chunk_renderer.add_chunk(&chunk, mesh);
            self.chunks.update_chunk(chunk);
        }
    }



    pub fn collide_line_segment(
        &self,
        start: Point3<f32>,
        direction: Vector3<f32>,
    ) -> Option<(&Chunk, &Block, Point3<f32>)>
    {
        todo!();
    }
}
