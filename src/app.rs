use std::sync::Arc;
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

use crate::state::State;



pub struct App
{
    state: Option<State>,
    last_time: std::time::Instant,
}



impl App
{
    pub fn new() -> Self
    {
        Self {
            state: None,
            last_time: instant::Instant::now(),
        }
    }
}



impl ApplicationHandler<State> for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }



    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State)
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
        let state = if let Some(state) = &mut self.state
        {
            state
        }
        else
        {
            return;
        };

        match event
        {
            DeviceEvent::MouseMotion { delta: (dx, dy) } =>
            {
                state.camera_controller.handle_mouse(dx, dy);
            }
            _ =>
            {}
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
                self.last_time = instant::Instant::now();
                state.update(dt);
                match state.render()
                {
                    Ok(_) => (),

                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                    {
                        let size = state.renderer.window.inner_size();
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

            WindowEvent::CursorMoved {
                device_id: _device_id,
                position,
            } =>
            {
                state.mouse_pos = position;
            }
            _ => (),
        }
    }
}
