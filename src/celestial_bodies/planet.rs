use crate::physics::physics_body_state::PhysicsBodyState;



pub struct Planet
{
    pub mass: f32,
    pub physics_state: PhysicsBodyState<f32>,
}
