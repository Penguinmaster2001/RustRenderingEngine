use crate::physics::{
    physics_body::PhysicsBody,
    physics_environment::ForceField,
};
use nalgebra::Point3;



pub fn calculate_trajectory<W: ForceField>(
    dt: f32,
    num: usize,
    body: &PhysicsBody,
    world: &W,
) -> Vec<(Point3<f32>, f32)>
{
    let mut trajectory_state = body.state;

    let len = num / 10000;

    let mut trajectory = vec![(*trajectory_state.get_pos(), 0.0)];
    for i in 0..num
    {
        trajectory_state.add_acceleration(
            world.sample_force(*trajectory_state.get_pos()) / body.properties.mass,
        );
        trajectory_state.update(dt);

        if i % len == 0
        {
            trajectory.push((*trajectory_state.get_pos(), i as f32 / num as f32));
        }
    }

    trajectory
}
