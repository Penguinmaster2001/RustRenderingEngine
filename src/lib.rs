use crate::{
    app::App,
    vertex::TextureVertex,
};
use winit::{
    event_loop::EventLoop,
    platform::wayland::EventLoopBuilderExtWayland,
};



pub mod app;
pub mod camera;
pub mod chunking;
pub mod instance;
pub mod model;
pub mod rendering;
pub mod resources;
pub mod state;
pub mod texture;
pub mod vertex;
pub mod world_gen;



pub fn run() -> anyhow::Result<()>
{
    env_logger::init();

    let event_loop = EventLoop::with_user_event().with_wayland().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
