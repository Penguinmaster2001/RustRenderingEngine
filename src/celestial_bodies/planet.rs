use crate::physics::physics_body::physics_body_state::PhysicsBodyState;



pub mod planet_meshing;



pub struct Planet
{
    pub mass: f32,
    pub physics_state: PhysicsBodyState<f32>,
}
