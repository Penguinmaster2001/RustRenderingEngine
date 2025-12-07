#[rustfmt::skip]
pub const CUBE_VERTEX_OFFSETS: [f32; 3 * 8] = [
    0.0, 0.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 1.0,
    1.0, 0.0, 0.0,
    1.0, 0.0, 1.0,
    1.0, 1.0, 0.0,
    1.0, 1.0, 1.0,
];



pub const BLOCK_SIZE: f32 = 1.0;



#[derive(Clone, Copy)]
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
