use nalgebra::{
    Point3,
    Vector3,
};

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
    v: f32,
) -> MeshBuffer
{
    let mut trajectory_state = body.state;

    let len = num / 10000;

    let mut trajectory = vec![TextureVertex::new(*trajectory_state.get_pos(), [0.0, v])];
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
                [i as f32 / num as f32, v],
            ));
        }
    }

    MeshBuffer::from_verts(
        &trajectory,
        &(0..trajectory.len() as _).collect::<Vec<u32>>(),
        renderer,
    )
}



#[derive(Clone, Copy)]
struct Body
{
    pos: Point3<f32>,
    vel: Vector3<f32>,
    acc: Vector3<f32>,
}



pub fn calculate_trajectory_leapfrog<W: ForceField>(
    dt: f32,
    num: usize,
    body: &PhysicsBody,
    world: &W,
    renderer: &Renderer,
    v: f32,
) -> MeshBuffer
{
    let mass = body.properties.mass;
    let body_a = Body {
        pos: *body.state.get_pos(),
        vel: *body.state.get_vel(),
        acc: *body.state.get_acc(),
    };

    let mut bodies = [body_a, body_a];

    let len = num / 10000;

    let mut trajectory = vec![];
    for i in 0..num
    {
        let current_acc = bodies[i % 2].acc + world.sample_force(bodies[i % 2].pos) / mass;
        let vel_half = bodies[i % 2].vel + (current_acc * (0.5 * dt));

        bodies[(i + 1) % 2].pos = bodies[i % 2].pos + (vel_half * dt);

        // Compute new acceleration at x_{n+1}
        let acc_next = world.sample_force(bodies[(i + 1) % 2].pos) / mass;

        // Final velocity: v_{n+1} = v_{n+1/2} + (a_{n+1} * dt / 2)
        bodies[(i + 1) % 2].vel = vel_half + (acc_next * (dt * 0.5));

        // Store for next step
        bodies[(i + 1) % 2].acc = Vector3::zeros();

        if i % len == 0
        {
            trajectory.push(TextureVertex::new(
                bodies[i % 2].pos,
                [i as f32 / num as f32, v],
            ));
        }
    }

    MeshBuffer::from_verts(
        &trajectory,
        &(0..trajectory.len() as _).collect::<Vec<u32>>(),
        renderer,
    )
}
