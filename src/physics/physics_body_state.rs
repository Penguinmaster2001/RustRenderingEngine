use nalgebra::{
    Point3,
    Quaternion,
    Scalar,
    SimdRealField,
    Vector3,
};



pub struct PhysicsBodyState<S: Scalar>
{
    position: Point3<S>,
    velocity: Vector3<S>,
    acceleration: Vector3<S>,
    _rotation: Quaternion<S>,
    _angular_velocity: Vector3<S>,
    _angular_acceleration: Vector3<S>,
}



impl<S: Scalar> PhysicsBodyState<S>
{
    pub fn get_pos(&self) -> &Point3<S>
    {
        &self.position
    }



    pub fn get_vel(&self) -> &Vector3<S>
    {
        &self.velocity
    }



    pub fn get_acc(&self) -> &Vector3<S>
    {
        &self.acceleration
    }
}



impl<S: SimdRealField> PhysicsBodyState<S>
{
    pub fn new() -> Self
    {
        Self {
            position: Point3::origin(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            _rotation: Quaternion::identity(),
            _angular_velocity: Vector3::zeros(),
            _angular_acceleration: Vector3::zeros(),
        }
    }



    pub fn translate<T: Into<Vector3<S>>>(&mut self, translation: T)
    {
        self.position += translation.into();
    }



    pub fn add_acceleration<A: Into<Vector3<S>>>(&mut self, acceleration: A)
    {
        self.acceleration += acceleration.into();
    }
}



impl<S: SimdRealField + Copy> PhysicsBodyState<S>
{
    pub fn update<T: Into<S>>(&mut self, dt: T)
    {
        let dt = dt.into();
        self.position += self.velocity.scale(dt) + self.acceleration.scale(dt * dt);
        self.velocity += self.acceleration.scale(dt);

        self.acceleration = Vector3::zeros();
    }
}
