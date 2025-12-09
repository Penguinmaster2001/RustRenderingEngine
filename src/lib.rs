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



const VERTICES: &[TextureVertex] = &[
    // Changed
    TextureVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    TextureVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    TextureVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    TextureVertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
    TextureVertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];



#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4
];



pub fn run() -> anyhow::Result<()>
{
    env_logger::init();

    let event_loop = EventLoop::with_user_event().with_wayland().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
