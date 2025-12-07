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
pub struct Block
{
    pub block_type: u8,
}



impl Block
{
    pub fn new(block_type: u8) -> Self
    {
        Self { block_type }
    }
}
