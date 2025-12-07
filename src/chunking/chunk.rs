use crate::chunking::blocks::{
    Block,
    BlockType,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub const CHUNK_SIZE: usize = 6;
pub const CHUNK_BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;



pub struct Chunk
{
    blocks: [Block; CHUNK_BLOCK_COUNT],
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
        let blocks = [Block::new(BlockType::Full); CHUNK_BLOCK_COUNT];

        Self { blocks }
    }



    pub fn block_at(&self, x: u16, y: u16, z: u16) -> Block
    {
        self.blocks
            [((x as usize) * CHUNK_SIZE * CHUNK_SIZE) + ((y as usize) * CHUNK_SIZE) + (z as usize)]
    }
}
