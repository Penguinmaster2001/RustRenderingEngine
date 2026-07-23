use crate::{
    input::{
        InputEvent,
        InputHandler,
    },
    physics::physics_environment::{
        ConstantForceField,
        ForceField,
    },
    player::spaceship_controller::SpaceshipController,
    threading::{
        WorkerHandle,
        WorkerThread,
    },
};
use nalgebra::Vector3;
use std::time::{
    Duration,
    Instant,
};
use winit::keyboard::KeyCode;



pub type PhysicsSim = WorkerHandle<InputEvent, SpaceshipController>;
pub type PhysicsThread<F> =
    WorkerThread<instant::Duration, InputEvent, SpaceshipController, (Instant, F)>;



impl PhysicsSim
{
    pub fn new_physics_sim(spaceship: SpaceshipController) -> Self
    {
        PhysicsSim::new(
            Duration::from_secs_f32(1.0 / 180.0),
            spaceship,
            (Instant::now(), ConstantForceField::default()),
            run,
        )
    }
}



fn run<F>(physics_thread: PhysicsThread<F>)
where
    F: ForceField,
{
    let mut physics_thread = physics_thread;
    let mut scale = 4u32.pow(0);
    loop
    {
        let now = Instant::now();
        let dt = now - physics_thread.internal_state.0;
        if dt * scale < physics_thread.config
        {
            continue;
        }
        physics_thread.internal_state.0 = now;
        if let Ok(mut player) = physics_thread.state.write()
        {
            for event in physics_thread.input_rx.try_iter()
            {
                match event
                {
                    InputEvent::MouseMotion { delta } => player.handle_mouse_movement(delta),
                    InputEvent::Keyboard { code, pressed } =>
                    {
                        player.handle_key(code, pressed);
                        if pressed
                        {
                            match code
                            {
                                KeyCode::BracketRight => scale *= 4,
                                KeyCode::BracketLeft => scale /= 4,
                                _ => (),
                            };
                            scale = scale.clamp(1, 4u32.pow(11));
                            true
                        }
                        else
                        {
                            false
                        }
                    }
                    _ => false,
                };
            }
            let force = physics_thread
                .internal_state
                .1
                .sample_force(*player.body.state.get_pos())
                / player.body.properties.mass;

            player.body.state.add_acceleration(force);
            player.update(physics_thread.config);
            player.body.state.velocity = player.body.state.velocity.lerp(&Vector3::zeros(), 0.1);
        }
    }
}
