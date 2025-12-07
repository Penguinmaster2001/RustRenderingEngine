use crate::chunking::blocks::{
    Block,
    BlockType,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub const CHUNK_SIZE: u8 = 8;
pub const CHUNK_BLOCK_COUNT: u32 = CHUNK_SIZE as u32 * CHUNK_SIZE as u32 * CHUNK_SIZE as u32;



pub struct Chunk
{
    blocks: [Block; CHUNK_BLOCK_COUNT as usize],
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



impl Chunk
{
    pub fn new() -> Self
    {
        let blocks = [Block::new(BlockType::Full); CHUNK_BLOCK_COUNT as usize];

        Self { blocks }
    }



    pub fn block_at(&self, x: u8, y: u8, z: u8) -> Option<&Block>
    {
        self.blocks.get(
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
