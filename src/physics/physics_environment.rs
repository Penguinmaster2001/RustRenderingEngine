use nalgebra::{
    Point3,
    Vector3,
};



pub struct EmptyForceField;



impl EmptyForceField
{
    pub fn new() -> Self
    {
        Self {}
    }
}



impl ForceField for EmptyForceField
{
    fn sample_force<P: Into<Point3<f32>>>(&self, _: P) -> Vector3<f32>
    {
        Vector3::zeros()
    }
}



pub trait ForceField
{
    fn sample_force<P: Into<Point3<f32>>>(&self, point: P) -> Vector3<f32>;
}
