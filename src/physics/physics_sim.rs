use crate::{
    input::{
        InputEvent,
        InputHandler,
    },
    physics::physics_environment::ForceField,
    player::spaceship_controller::SpaceshipController,
};
use std::{
    sync::{
        Arc,
        RwLock,
        RwLockReadGuard,
        mpsc,
    },
    thread,
    time::Instant,
};
use winit::keyboard::KeyCode;



pub struct PhysicsSim
{
    pub dt: instant::Duration,
    input_tx: mpsc::Sender<InputEvent>,
    _handle: thread::JoinHandle<()>,
    player: Arc<RwLock<SpaceshipController>>,
}



impl PhysicsSim
{
    pub fn new<F: 'static + ForceField + Send>(
        dt: instant::Duration,
        player: SpaceshipController,
        world: F,
    ) -> Self
    {
        let player = Arc::new(RwLock::new(player));
        let (input_tx, input_rx) = mpsc::channel();
        let mut physics_thread = PhysicsThread::new(dt, player.clone(), world, input_rx);
        Self {
            dt,
            input_tx,
            _handle: thread::spawn(move || {
                physics_thread.run();
            }),
            player,
        }
    }



    pub fn handle_input(&self, event: &InputEvent)
    {
        self.input_tx
            .send(*event)
            .expect("Should be able to send input event.");
    }



    pub fn get_player(&'_ self) -> Option<RwLockReadGuard<'_, SpaceshipController>>
    {
        self.player.read().ok()
    }
}



pub struct PhysicsThread<F: ForceField>
{
    dt: instant::Duration,
    last_time: Instant,
    player: Arc<RwLock<SpaceshipController>>,
    world: F,
    input_rx: mpsc::Receiver<InputEvent>,
}



impl<F: ForceField> PhysicsThread<F>
{
    pub fn new(
        dt: instant::Duration,
        player: Arc<RwLock<SpaceshipController>>,
        world: F,
        input_rx: mpsc::Receiver<InputEvent>,
    ) -> Self
    {
        Self {
            dt,
            last_time: Instant::now(),
            player,
            world,
            input_rx,
        }
    }



    pub fn run(&mut self)
    {
        let mut scale = 4u32.pow(0);
        loop
        {
            let now = Instant::now();
            let dt = now - self.last_time;
            if dt * scale < self.dt
            {
                continue;
            }
            self.last_time = now;
            if let Ok(mut player) = self.player.write()
            {
                for event in self.input_rx.try_iter()
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
                let force = self.world.sample_force(*player.body.state.get_pos())
                    / player.body.properties.mass;

                player.body.state.add_acceleration(force);
                player.update(self.dt);
                // println!("Update, {:?}, {:?}", dt, self.dt);
                println!("{:?}", player.body.state);
            }
        }
    }
}
