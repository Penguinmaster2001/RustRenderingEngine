use crate::{
    chunking::{
        blocks::Block,
        chunk::{
            Chunk,
            ChunkContainer,
        },
        chunk_renderer::ChunkRenderer,
    },
    player::Player,
    rendering::{
        Renderer,
        mesh::MeshBuffer,
    },
    world_gen::world_generator::WorldGenerator,
};
use nalgebra::{
    Point3,
    Vector3,
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
            generator: WorldGenerator::new(10, 8),
        }
    }



    pub fn update(&mut self, player: &Player, renderer: &Renderer, _dt: instant::Duration)
    {
        self.generator.generate_chunks(player.get_position(), 4);

        let generated_chunks = self.generator.drain_results();

        for (chunk, mesh_data) in generated_chunks
        {
            let mesh = MeshBuffer::from_data(&mesh_data, renderer);
            self.chunk_renderer.add_chunk(&chunk, mesh);
            self.chunks.update_chunk(chunk);
        }
    }



    pub fn sample_force(&self, point: &Point3<f32>) -> Vector3<f32>
    {
        let mut force = Vector3::zeros();
        for (pos, chunk) in &self.chunks.chunks
        {
            if chunk.empty
            {
                continue;
            }
            let to_center = (ChunkContainer::chunk_to_world(pos) + chunk.center_of_mass) - point;
            let distance = to_center.magnitude();
            if distance < 0.1
            {
                continue;
            }
            force += to_center * (chunk.mass / (distance * distance * distance));
        }

        force
    }



    pub fn collide_line_segment(
        &self,
        _start: Point3<f32>,
        _direction: Vector3<f32>,
    ) -> Option<(&Chunk, &Block, Point3<f32>)>
    {
        todo!();
    }
}
