use crate::{
    chunking::blocks::{
        BLOCK_SIZE,
        Block,
        BlockType,
    },
    world_gen::world_noise::HeightNoise,
};
use cgmath::Point3;
use std::collections::HashMap;



pub const CHUNK_BLOCK_SIZE: u8 = 32;
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE;
pub const CHUNK_BLOCK_COUNT: u32 =
    CHUNK_BLOCK_SIZE as u32 * CHUNK_BLOCK_SIZE as u32 * CHUNK_BLOCK_SIZE as u32;



pub struct Chunk
{
    pub world_offset: Point3<i64>,
    blocks: [Block; CHUNK_BLOCK_COUNT as usize],
}



impl Chunk
{
    pub fn from_offset(world_offset: &Point3<i64>) -> Self
    {
        let height_map = HeightNoise::new(); //HeightMap::new();
        let blocks = [Block::new(BlockType::Empty); CHUNK_BLOCK_COUNT as usize];

        let mut chunk = Self {
            world_offset: *world_offset,
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
    pub chunks: HashMap<Point3<i64>, Chunk>,
}



impl ChunkContainer
{
    pub fn new() -> Self
    {
        Self {
            chunks: HashMap::new(),
        }
    }



    pub fn add_at(&mut self, pos: &Point3<i64>) -> bool
    {
        if !self.chunks.contains_key(pos)
        {
            let chunk = Chunk::from_offset(pos);
            self.chunks.insert(*pos, chunk);
            true
        }
        else
        {
            false
        }
    }



    pub fn world_to_chunk(pos: &Point3<f32>) -> Point3<i64>
    {
        (pos / (CHUNK_BLOCK_SIZE as f32 * BLOCK_SIZE))
            .cast()
            .unwrap_or((0, 0, 0).into())
    }



    pub fn get_chunk_at_world(&self, pos: &Point3<f32>) -> Option<&Chunk>
    {
        let key = ChunkContainer::world_to_chunk(pos).into();

        self.chunks.get(&key)
    }



    pub fn get_chunk(&self, pos: &Point3<i64>) -> Option<&Chunk>
    {
        self.chunks.get(pos)
    }



    pub fn chunk_at(self, x: i64, y: i64, z: i64) -> bool
    {
        let key = (x, y, z).into();

        self.chunks.contains_key(&key)
    }
}
