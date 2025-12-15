use crate::{
    chunking::blocks::{
        BLOCK_SIZE,
        Block,
        BlockType,
    },
    world_gen::world_noise::HeightNoise,
};
use cgmath::Vector3;
use std::collections::HashMap;



pub const CHUNK_BLOCK_SIZE: u8 = 32;
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE;
pub const CHUNK_BLOCK_COUNT: u32 =
    CHUNK_BLOCK_SIZE as u32 * CHUNK_BLOCK_SIZE as u32 * CHUNK_BLOCK_SIZE as u32;



pub struct Chunk
{
    pub world_offset: Vector3<i64>,
    blocks: [Block; CHUNK_BLOCK_COUNT as usize],
}



impl Chunk
{
    pub fn from_offset(x: i64, y: i64, z: i64) -> Self
    {
        let height_map = HeightNoise::new(); //HeightMap::new();
        let blocks = [Block::new(BlockType::Empty); CHUNK_BLOCK_COUNT as usize];

        let mut chunk = Self {
            world_offset: (x, y, z).into(),
            blocks,
        };

        for x in 0..CHUNK_BLOCK_SIZE
        {
            for z in 0..CHUNK_BLOCK_SIZE
            {
                let mut height = height_map.get(
                    (x as f32 * BLOCK_SIZE) as i64
                        + (chunk.world_offset.x as f32 * CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE)
                            as i64,
                    (z as f32 * BLOCK_SIZE) as i64
                        + (chunk.world_offset.z as f32 * CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE)
                            as i64,
                ) - (chunk.world_offset.y * CHUNK_BLOCK_SIZE as i64);

                if height < 0
                {
                    continue;
                }

                if height > CHUNK_BLOCK_SIZE as i64
                {
                    height = CHUNK_BLOCK_SIZE as i64;
                }

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
            ((((x as usize) * CHUNK_BLOCK_SIZE as usize) + y as usize) * CHUNK_BLOCK_SIZE as usize)
                + z as usize,
        )
    }



    pub fn block_at_mut(&mut self, x: u8, y: u8, z: u8) -> Option<&mut Block>
    {
        self.blocks.get_mut(
            ((((x as usize) * CHUNK_BLOCK_SIZE as usize) + y as usize) * CHUNK_BLOCK_SIZE as usize)
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
    pub chunks: HashMap<Vector3<i64>, Chunk>,
}



impl ChunkContainer
{
    pub fn new() -> Self
    {
        Self {
            chunks: HashMap::new(),
        }
    }



    pub fn add_at(&mut self, x: i64, y: i64, z: i64) -> bool
    {
        let key = (x, y, z).into();

        if !self.chunks.contains_key(&key)
        {
            let chunk = Chunk::from_offset(x, y, z);
            self.chunks.insert(key, chunk);
            true
        }
        else
        {
            false
        }
    }



    pub fn world_to_chunk(x: f32, y: f32, z: f32) -> (i64, i64, i64)
    {
        let x = x as i64 / (CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE) as i64;
        let y = y as i64 / (CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE) as i64;
        let z = z as i64 / (CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE) as i64;

        (x, y, z)
    }



    pub fn get_chunk_at_world(&self, x: f32, y: f32, z: f32) -> Option<&Chunk>
    {
        let key = ChunkContainer::world_to_chunk(x, y, z).into();

        self.chunks.get(&key)
    }



    pub fn get_chunk(&self, x: i64, y: i64, z: i64) -> Option<&Chunk>
    {
        let key = (x, y, z).into();

        self.chunks.get(&key)
    }



    pub fn chunk_at(self, x: i64, y: i64, z: i64) -> bool
    {
        let key = (x, y, z).into();

        self.chunks.contains_key(&key)
    }
}
