use crate::celestial_bodies::planet::Planet;
use nalgebra::{
    Point3,
    Vector3,
};



pub mod planet;



pub struct CelestialBodyContainer
{
    bodies: Vec<Planet>,
}



impl CelestialBodyContainer
{
    pub fn sample_force(&self, point: &Point3<f32>) -> Vector3<f32>
    {
        let mut force = Vector3::zeros();
        for body in &self.bodies
        {
            let to_center = body.physics_state.get_pos() - point;
            let distance = to_center.magnitude();
            if distance < 0.1
            {
                continue;
            }
            force += to_center * (body.mass / (distance * distance * distance));
        }

        force
    }
}
