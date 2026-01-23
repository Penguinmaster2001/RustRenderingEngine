use crate::physics::physics_body::{
    physics_body_properties::PhysicsBodyProperties,
    physics_body_state::PhysicsBodyState,
};



pub mod physics_body_properties;
pub mod physics_body_state;



#[derive(Clone, Copy)]
pub struct PhysicsBody
{
    pub properties: PhysicsBodyProperties,
    pub state: PhysicsBodyState<f32>,
}



impl PhysicsBody
{
    pub fn new(properties: PhysicsBodyProperties, state: PhysicsBodyState<f32>) -> Self
    {
        Self { properties, state }
    }
}



impl Default for PhysicsBody
{
    fn default() -> Self
    {
        Self::new(
            PhysicsBodyProperties::default(),
            PhysicsBodyState::default(),
        )
    }
}
