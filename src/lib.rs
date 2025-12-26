use crate::app::App;
use winit::event_loop::EventLoop;



pub mod app;
pub mod chunking;
pub mod instance;
pub mod model;
pub mod player;
pub mod rendering;
pub mod resources;
pub mod state;
pub mod texture;
pub mod vertex;
pub mod world_gen;



pub fn run() -> anyhow::Result<()>
{
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
