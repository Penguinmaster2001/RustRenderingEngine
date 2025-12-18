use std::collections::VecDeque;

use crate::{
    chunking::{
        chunk::{
            Chunk,
            ChunkContainer,
        },
        chunk_mesh::ChunkMesh,
    },
    rendering::Renderer,
};
use cgmath::{
    Point3,
    Vector3,
};



pub struct WorldGenerator
{
    pub load_radius: i64,
    to_generate: VecDeque<Point3<i64>>,
}



impl WorldGenerator
{
    pub fn new(load_radius: i64) -> Self
    {
        Self {
            load_radius,
            to_generate: VecDeque::new(),
        }
    }



    pub fn generate_chunks(
        &mut self,
        center_pos: &Point3<f32>,
        renderer: &Renderer,
        chunks: &ChunkContainer,
        num: u32,
    ) -> Vec<(Chunk, ChunkMesh)>
    {
        let center_chunk = ChunkContainer::world_to_chunk(center_pos);

        let radius = self.load_radius;
        for x in -radius..radius
        {
            for z in -radius..radius
            {
                for y in -radius..radius
                {
                    let pos = center_chunk + Vector3::new(x, y, z);
                    if !chunks.chunk_at(&pos)
                    {
                        self.to_generate.push_back(pos);
                    }
                }
            }
        }

        let mut num = num;

        let mut generated_chunks = vec![];

        while let Some(pos) = self.to_generate.pop_front()
            && num > 0
        {
            if !chunks.chunk_at(&pos)
            {
                num -= 1;

                let chunk = Chunk::from_offset(&pos);
                let mesh = ChunkMesh::from_chunk(&chunk, renderer);

                generated_chunks.push((chunk, mesh));
            }
        }

        generated_chunks
    }
}
