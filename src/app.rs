use crate::{
    input::InputHandler,
    state::State,
};
use std::{
    sync::Arc,
    time::Instant,
};
use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent,
        DeviceId,
        KeyEvent,
        WindowEvent,
    },
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};



pub struct App<'a>
{
    state: Option<State<'a>>,
    last_time: Instant,
}



impl<'a> App<'a>
{
    pub fn new() -> Self
    {
        Self {
            state: None,
            last_time: Instant::now(),
        }
    }
}



impl<'a> Default for App<'a>
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl ApplicationHandler<State<'static>> for App<'static>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = match event_loop.create_window(window_attributes)
        {
            Ok(window) => Arc::new(window),
            Err(error) =>
            {
                log::error!("Failed to create window: {error}");
                event_loop.exit();
                return;
            }
        };

        self.state = match pollster::block_on(State::new(window))
        {
            Ok(state) => Some(state),
            Err(error) =>
            {
                log::error!("Failed to create state: {error}");
                event_loop.exit();
                return;
            }
        };
    }



    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State<'static>)
    {
        self.state = Some(event);
    }



    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    )
    {
        let Some(state) = &mut self.state
        else
        {
            return;
        };

        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event
        {
            state.player.handle_mouse_movement(dx, dy);
        }
    }



    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    )
    {
        let state = match &mut self.state
        {
            Some(canvas) => canvas,
            None => return,
        };

        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested =>
            {
                let dt = self.last_time.elapsed();
                self.last_time = Instant::now();
                state.update(dt);
                match state.render()
                {
                    Ok(_) => (),

                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                    {
                        let size = state.renderer_state.window.inner_size();
                        state.resize(size.width, size.height);
                    }

                    Err(e) =>
                    {
                        log::error!("Unable to render {}", e);
                    }
                }
            }

            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => state.handle_mouse_button(button, btn_state.is_pressed()),

            WindowEvent::MouseWheel { delta, .. } =>
            {
                state.handle_mouse_scroll(&delta);
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),

            _ => (),
        }
    }
}
