use crate::{
    chunking::chunk::Chunk,
    rendering::mesh::MeshBuffer,
};
use nalgebra::Point3;
use std::collections::HashMap;



pub struct ChunkRenderer
{
    chunk_meshes: HashMap<Point3<i64>, MeshBuffer>,
}



impl ChunkRenderer
{
    pub fn new() -> Self
    {
        Self {
            chunk_meshes: HashMap::new(),
        }
    }



    pub fn add_chunk(&mut self, chunk: &Chunk, mesh: MeshBuffer)
    {
        self.chunk_meshes.insert(chunk.world_offset, mesh);
    }
}



impl Default for ChunkRenderer
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl ChunkRenderer
{
    pub fn get_meshes(&self) -> Vec<&MeshBuffer>
    {
        self.chunk_meshes.values().collect()
    }
}
