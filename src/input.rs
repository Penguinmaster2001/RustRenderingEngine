use nalgebra::Vector2;
use winit::{
    event::{
        DeviceEvent,
        KeyEvent,
        MouseButton,
        MouseScrollDelta,
        WindowEvent,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
};



pub mod input_settings;



pub trait InputHandler
{
    fn handle_key(&mut self, _key: KeyCode, _pressed: bool) -> bool
    {
        false
    }



    fn handle_mouse_movement(&mut self, _delta: Vector2<f32>) -> bool
    {
        false
    }



    fn handle_mouse_button(&mut self, _button: MouseButton, _pressed: bool) -> bool
    {
        false
    }



    fn handle_mouse_scroll(&mut self, _delta: &MouseScrollDelta) -> bool
    {
        false
    }
}



pub enum InputEvent
{
    MouseWheel
    {
        delta: MouseScrollDelta
    },
    MouseMotion
    {
        delta: Vector2<f32>
    },
    MouseButton
    {
        button: MouseButton, pressed: bool
    },
    Keyboard
    {
        code: KeyCode, pressed: bool
    },
}



impl TryFrom<WindowEvent> for InputEvent
{
    type Error = ();



    fn try_from(event: WindowEvent) -> Result<Self, Self::Error>
    {
        match event
        {
            WindowEvent::MouseInput { state, button, .. } => Ok(InputEvent::MouseButton {
                button,
                pressed: state.is_pressed(),
            }),
            WindowEvent::MouseWheel { delta, .. } => Ok(InputEvent::MouseWheel { delta }),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => Ok(InputEvent::Keyboard {
                code,
                pressed: state.is_pressed(),
            }),
            _ => Err(()),
        }
    }
}



impl TryFrom<DeviceEvent> for InputEvent
{
    type Error = ();



    fn try_from(event: DeviceEvent) -> Result<Self, Self::Error>
    {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event
        {
            Ok(InputEvent::MouseMotion {
                delta: Vector2::new(dx as f32, dy as f32),
            })
        }
        else
        {
            Err(())
        }
    }
}
