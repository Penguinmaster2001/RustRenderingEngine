use crate::physics::physics_body::{
    physics_body_properties::PhysicsBodyProperties,
    physics_body_state::PhysicsBodyState,
};



pub mod physics_body_properties;
pub mod physics_body_state;



pub struct PhysicsBody
{
    pub properties: PhysicsBodyProperties,
    pub state: PhysicsBodyState<f32>,
}
