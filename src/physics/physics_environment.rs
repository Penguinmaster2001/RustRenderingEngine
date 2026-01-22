use nalgebra::{
    Point3,
    Vector3,
};



pub trait PhysicsEnvironment
{
    fn sample_force<P: Into<Point3<f32>>>(&self, point: P) -> Vector3<f32>;
}
