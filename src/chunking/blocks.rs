pub mod block_mesh;



pub const BLOCK_SIZE: f32 = 1.0;



pub enum BlockFace
{
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}



#[derive(Clone, Copy, PartialEq)]
pub enum BlockType
{
    Empty,
    Grass,
    Stone,
}



#[derive(Clone, Copy)]
pub struct Block
{
    pub block_type: BlockType,
}



impl Block
{
    pub fn new(block_type: BlockType) -> Self
    {
        Self { block_type }
    }
}
