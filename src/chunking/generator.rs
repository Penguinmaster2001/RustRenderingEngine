use crate::chunking::chunk::CHUNK_BLOCK_SIZE;



pub struct HeightMap
{
    height_map: [u32; CHUNK_BLOCK_SIZE as usize * CHUNK_BLOCK_SIZE as usize],
}



impl HeightMap
{
    pub fn new() -> Self
    {
        let mut height_map = [0; CHUNK_BLOCK_SIZE as usize * CHUNK_BLOCK_SIZE as usize];
        for i in 0..(CHUNK_BLOCK_SIZE as usize * CHUNK_BLOCK_SIZE as usize)
        {
            height_map[i] = rand::random_range(0..(CHUNK_BLOCK_SIZE as u32));
        }

        Self { height_map }
    }



    pub fn height(&self, x: u8, z: u8) -> u32
    {
        self.height_map[(x as usize * CHUNK_BLOCK_SIZE as usize) + z as usize]
    }
}
