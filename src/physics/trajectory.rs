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
    };

    let mut bodies = [body_a, body_a];

    let len = num / 10000;

    let mut trajectory = vec![];
    for i in 0..num
    {
        let current_acc = world.sample_force(bodies[i % 2].pos) / mass;
        let vel_half = bodies[i % 2].vel + (current_acc * (0.5 * dt));

        bodies[(i + 1) % 2].pos = bodies[i % 2].pos + (vel_half * dt);

        // Compute new acceleration at x_{n+1}
        let acc_next = world.sample_force(bodies[(i + 1) % 2].pos) / mass;

        // Final velocity: v_{n+1} = v_{n+1/2} + (a_{n+1} * dt / 2)
        bodies[(i + 1) % 2].vel = vel_half + (acc_next * (dt * 0.5));

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



#[derive(Clone, Copy)]
struct ABBody
{
    pos: Point3<f32>,
    vel: Vector3<f32>,
    acc: Vector3<f32>,
    acc_prev: Vector3<f32>,
    mass: f32,
}



fn adams_bashforth_4<W: ForceField>(dt: f32, current: &ABBody, next: &mut ABBody, world: &W)
{
    let body = current;

    // Current acceleration
    let f_n = compute_acceleration(current, world);

    // Coefficients for 4-step AB formula
    let coeff = [55.0, -59.0, 37.0, -9.0];
    let dt_24 = dt / 24.0;

    // Adams-Bashforth predictor (uses stored past accelerations)
    // Note: in practice, you'd maintain a ringbuffer of past accelerations
    // For now, approximate by using current and one past
    let ab_correction = f_n * (55.0 * dt_24);

    let corrected_vel = body.vel + ab_correction;
    let corrected_pos = body.pos + (corrected_vel * dt);

    next.pos = corrected_pos;
    next.vel = corrected_vel;
    next.acc = f_n;
    next.mass = body.mass;

    // Store for next step
    next.acc_prev = f_n;
}



fn compute_acceleration<W: ForceField>(current: &ABBody, world: &W) -> Vector3<f32>
{
    world.sample_force(current.pos) / current.mass
}
