use crate::chunking::{
    blocks::{
        Block,
        BlockType,
    },
    generator::HeightMap,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub const CHUNK_SIZE: u8 = 8;
pub const CHUNK_BLOCK_COUNT: u32 = CHUNK_SIZE as u32 * CHUNK_SIZE as u32 * CHUNK_SIZE as u32;



pub struct Chunk
{
    blocks: [Block; CHUNK_BLOCK_COUNT as usize],
}



impl Chunk
{
    pub fn new() -> Self
    {
        let height_map = HeightMap::new();
        let blocks = [Block::new(BlockType::Empty); CHUNK_BLOCK_COUNT as usize];

        let mut chunk = Self { blocks };

        for x in 0..CHUNK_SIZE
        {
            for z in 0..CHUNK_SIZE
            {
                let height = height_map.height(x, z);
                for y in 0..(height as u8)
                {
                    chunk.block_at_mut(x, y, z).and_then(|b| {
                        b.block_type = BlockType::Full;
                        Some(b)
                    });
                }
            }
        }

        chunk
    }



    pub fn block_at(&self, x: u8, y: u8, z: u8) -> Option<&Block>
    {
        self.blocks.get(
            ((((x as usize) * CHUNK_SIZE as usize) + y as usize) * CHUNK_SIZE as usize)
                + z as usize,
        )
    }



    pub fn block_at_mut(&mut self, x: u8, y: u8, z: u8) -> Option<&mut Block>
    {
        self.blocks.get_mut(
            ((((x as usize) * CHUNK_SIZE as usize) + y as usize) * CHUNK_SIZE as usize)
                + z as usize,
        )
    }



    pub fn block_is_solid(&self, x: u8, y: u8, z: u8) -> bool
    {
        match self.block_at(x, y, z)
        {
            None => false,
            Some(b) => b.block_type != BlockType::Empty,
        }
    }
}



pub struct ChunkContainer
{
    pub chunks: HashMap<Vector3<u32>, Chunk>,
}



impl ChunkContainer
{
    pub fn new() -> Self
    {
        let chunk = Chunk::new();
        let mut chunks = HashMap::new();
        chunks.insert((0, 0, 0).into(), chunk);
        Self { chunks }
    }
}
