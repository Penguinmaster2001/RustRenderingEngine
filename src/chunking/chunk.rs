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
    pub empty: bool,
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
            empty: true,
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
                        b.block_type = if 0 == y % 2
                        {
                            BlockType::Grass
                        }
                        else
                        {
                            BlockType::Stone
                        };

                        Some(b)
                    });

                    chunk.empty = false;
                }
            }
        }

        chunk
    }



    pub fn block_at<T: Into<Point3<u8>>>(&self, block: T) -> Option<&Block>
    {
        let block = block.into();
        self.blocks.get(
            ((((block.x as usize) * CHUNK_BLOCK_SIZE as usize) + block.y as usize)
                * CHUNK_BLOCK_SIZE as usize)
                + block.z as usize,
        )
    }



    pub fn solid_block_at<T: Into<Point3<u8>>>(&self, block: T) -> Option<&Block>
    {
        match self.block_at(block)
        {
            Some(b) => (b.block_type != BlockType::Empty).then_some(b),
            None => None,
        }
    }



    pub fn block_at_mut(&mut self, x: u8, y: u8, z: u8) -> Option<&mut Block>
    {
        self.blocks.get_mut(
            ((((x as usize) * CHUNK_BLOCK_SIZE as usize) + y as usize) * CHUNK_BLOCK_SIZE as usize)
                + z as usize,
        )
    }



    pub fn block_is_solid<T: Into<Point3<u8>>>(&self, block: T) -> bool
    {
        match self.block_at(block)
        {
            Some(b) => b.block_type != BlockType::Empty,
            None => false,
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



    pub fn update_chunk(&mut self, chunk: Chunk) -> Option<Chunk>
    {
        self.chunks.insert(chunk.world_offset, chunk)
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
        (pos / (BLOCK_SIZE * CHUNK_BLOCK_SIZE as f32))
            .cast()
            .unwrap()
    }



    pub fn chunk_to_world(pos: &Point3<i64>) -> Point3<f32>
    {
        pos.cast().unwrap() * BLOCK_SIZE * CHUNK_BLOCK_SIZE as f32
    }



    pub fn get_chunk_at_world(&self, pos: &Point3<f32>) -> Option<&Chunk>
    {
        let key = ChunkContainer::world_to_chunk(pos).into();

        self.chunks.get(&key)
    }



    pub fn get_chunk<C: Into<Point3<i64>>>(&self, pos: C) -> Option<&Chunk>
    {
        self.chunks.get(&pos.into())
    }



    pub fn chunk_at<C: Into<Point3<i64>>>(&self, pos: C) -> bool
    {
        self.chunks.contains_key(&pos.into())
    }



    pub fn get_block<C: Into<Point3<i64>>, B: Into<Point3<u8>>>(
        &self,
        chunk: C,
        block: B,
    ) -> Option<&Block>
    {
        self.get_chunk(chunk).and_then(|c| c.block_at(block))
    }
}
