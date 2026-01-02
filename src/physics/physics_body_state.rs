use nalgebra::{
    Point3,
    Quaternion,
    RealField,
    Vector3,
};



pub struct PhysicsBodyState<S: RealField>
{
    position: Point3<S>,
    velocity: Vector3<S>,
    acceleration: Vector3<S>,
    rotation: Quaternion<S>,
    angular_velocity: Vector3<S>,
    angular_acceleration: Vector3<S>,
}



impl<S: RealField> PhysicsBodyState<S>
{
    pub fn new() -> Self
    {
        Self {
            position: Point3::origin(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            rotation: Quaternion::identity(),
            angular_velocity: Vector3::zeros(),
            angular_acceleration: Vector3::zeros(),
        }
    }



    pub fn update(&mut self, dt: instant::Duration) {}
}
