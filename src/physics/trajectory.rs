use crate::{
    physics::{
        physics_body::PhysicsBody,
        physics_environment::ForceField,
    },
    rendering::{
        mesh::MeshBuffer,
        renderer::Renderer,
    },
    vertex::TextureVertex,
};



pub fn calculate_trajectory<W: ForceField>(
    dt: f32,
    num: usize,
    body: &PhysicsBody,
    world: &W,
    renderer: &Renderer,
) -> MeshBuffer
{
    let mut trajectory_state = body.state;

    let len = num / 10000;

    let mut trajectory = vec![TextureVertex::new(*trajectory_state.get_pos(), [0.0, 0.0])];
    for i in 0..num
    {
        trajectory_state.add_acceleration(
            world.sample_force(*trajectory_state.get_pos()) / body.properties.mass,
        );
        trajectory_state.update(dt);

        if i % len == 0
        {
            trajectory.push(TextureVertex::new(
                *trajectory_state.get_pos(),
                [i as f32 / num as f32, 0.0],
            ));
        }
    }

    MeshBuffer::from_verts(
        &trajectory,
        &(0..trajectory.len() as u32).collect::<Vec<u32>>(),
        renderer,
    )
}
