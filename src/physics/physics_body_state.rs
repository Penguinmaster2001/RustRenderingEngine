use cgmath::{
    BaseFloat,
    EuclideanSpace,
    One,
    Point3,
    Quaternion,
    Vector3,
    Zero,
};



pub struct PhysicsBodyState<S: BaseFloat>
{
    position: Point3<S>,
    velocity: Vector3<S>,
    acceleration: Vector3<S>,
    rotation: Quaternion<S>,
    angular_velocity: Vector3<S>,
    angular_acceleration: Vector3<S>,
}



impl<S: BaseFloat> PhysicsBodyState<S>
{
    pub fn new() -> Self
    {
        Self {
            position: Point3::origin(),
            velocity: Vector3::zero(),
            acceleration: Vector3::zero(),
            rotation: Quaternion::one(),
            angular_velocity: Vector3::zero(),
            angular_acceleration: Vector3::zero(),
        }
    }



    pub fn update(&mut self, dt: instant::Duration) {}
}
