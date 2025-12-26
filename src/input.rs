use winit::{
    event::{
        MouseButton,
        MouseScrollDelta,
    },
    keyboard::KeyCode,
};



pub mod input_settings;



pub trait InputHandler
{
    fn handle_key(&mut self, _key: KeyCode, _pressed: bool) -> bool
    {
        false
    }



    fn handle_mouse_movement(&mut self, _mouse_dx: f64, _mouse_dy: f64) -> bool
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
