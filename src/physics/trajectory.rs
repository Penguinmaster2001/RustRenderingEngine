use crate::physics::{
    physics_body::PhysicsBody,
    physics_environment::PhysicsEnvironment,
};
use nalgebra::Point3;



pub fn calculate_trajectory<W: PhysicsEnvironment>(
    dt: f32,
    num: usize,
    body: &PhysicsBody,
    world: &W,
) -> Vec<Point3<f32>>
{
    let mut trajectory_state = body.state;

    let mut trajectory = vec![*trajectory_state.get_pos()];
    for _ in 0..num
    {
        trajectory_state.add_acceleration(
            world.sample_force(*trajectory_state.get_pos()) / body.properties.mass,
        );
        trajectory_state.update(dt);
        trajectory.push(*trajectory_state.get_pos());
    }

    trajectory
}
