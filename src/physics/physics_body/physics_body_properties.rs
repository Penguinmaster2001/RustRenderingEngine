#[derive(Clone, Copy)]
pub struct PhysicsBodyProperties
{
    pub mass: f32,
}



impl PhysicsBodyProperties
{
    pub fn new(mass: f32) -> Self
    {
        Self { mass }
    }
}



impl Default for PhysicsBodyProperties
{
    fn default() -> Self
    {
        Self::new(1.0)
    }
}
