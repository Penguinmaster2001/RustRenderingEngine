use crate::{
    camera::Camera,
    chunking::{
        chunk::ChunkContainer,
        chunk_renderer::ChunkRenderer,
    },
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
            generator: WorldGenerator::new(2),
        }
    }



    pub fn update(&mut self, camera: &Camera, renderer: &Renderer, dt: instant::Duration)
    {
        let generated_chunks =
            self.generator
                .generate_chunks(&camera.position, renderer, &self.chunks, 4);

        for (chunk, mesh) in generated_chunks
        {
            self.chunk_renderer.add_chunk(&chunk, mesh);
            self.chunks.update_chunk(chunk);
        }
    }
}
