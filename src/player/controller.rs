use nalgebra::{
    RealField,
    Vector2,
    Vector3,
};
use std::fmt::Debug;



#[derive(Debug)]
pub struct Controller<S: RealField + Copy>
{
    movement: Vector3<S>,
    steering: Vector2<S>,
}



impl<S: RealField + Copy> Controller<S>
{
    pub fn new() -> Self
    {
        Self {
            movement: Vector3::zeros(),
            steering: Vector2::zeros(),
        }
    }



    pub fn reset_rotation(&mut self)
    {
        self.steering = Vector2::zeros();
    }



    pub fn reset_movement(&mut self)
    {
        self.movement = Vector3::zeros();
    }



    pub fn reset(&mut self)
    {
        self.reset_rotation();
        self.reset_movement();
    }



    pub fn forward(&mut self, amount: S)
    {
        self.movement.x = amount;
    }



    pub fn up(&mut self, amount: S)
    {
        self.movement.y = amount;
    }



    pub fn right(&mut self, amount: S)
    {
        self.movement.z = amount;
    }



    pub fn rotate_right(&mut self, amount: S)
    {
        self.steering.x = amount
    }



    pub fn rotate_up(&mut self, amount: S)
    {
        self.steering.y = amount
    }



    pub fn get_forward(&self) -> S
    {
        self.movement.x
    }



    pub fn get_up(&self) -> S
    {
        self.movement.y
    }



    pub fn get_left(&self) -> S
    {
        self.movement.z
    }



    pub fn steering(&self) -> Vector2<S>
    {
        self.steering
    }



    pub fn get_rotate_right(&mut self) -> S
    {
        self.steering.x
    }



    pub fn get_rotate_up(&mut self) -> S
    {
        self.steering.y
    }
}
