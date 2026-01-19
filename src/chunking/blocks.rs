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



#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType
{
    Empty,
    Grass,
    Stone,
    Max,
}



impl BlockType
{
    pub fn mass(&self) -> f32
    {
        match self
        {
            BlockType::Empty => 0.0,
            BlockType::Grass => 0.5,
            BlockType::Stone => 1.0,
            BlockType::Max => 0.0,
        }
    }
}



impl TryFrom<u8> for BlockType
{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        match value
        {
            0 => Ok(BlockType::Empty),
            1 => Ok(BlockType::Grass),
            2 => Ok(BlockType::Stone),
            _ => Err(()),
        }
    }
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



    pub fn is_solid(&self) -> bool
    {
        self.block_type != BlockType::Empty
    }



    pub fn solid(block: Option<&Block>) -> bool
    {
        if let Some(block) = block
        {
            return block.is_solid();
        }

        false
    }
}
