use nalgebra::{
    Point3,
    Vector3,
};



pub trait ForceField
{
    fn sample_force<P: Into<Point3<f32>>>(&self, point: P) -> Vector3<f32>;
}



#[derive(Default)]
pub struct ConstantForceField
{
    pub force: Vector3<f32>,
}



impl ConstantForceField
{
    pub fn new(force: Vector3<f32>) -> Self
    {
        Self { force }
    }
}



impl ForceField for ConstantForceField
{
    fn sample_force<P: Into<Point3<f32>>>(&self, _: P) -> Vector3<f32>
    {
        self.force
    }
}
