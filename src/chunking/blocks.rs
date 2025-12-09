pub const BLOCK_SIZE: f32 = 0.25;



pub enum BlockFaces
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
    Full,
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
