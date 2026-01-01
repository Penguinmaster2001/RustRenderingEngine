use crate::chunking::blocks::BlockType;



pub trait NoiseSource
{
    fn get<P: Into<(i64, i64, i64)>>(&self, point: P) -> BlockType;
}
