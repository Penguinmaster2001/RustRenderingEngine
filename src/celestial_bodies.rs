use crate::{
    celestial_bodies::planet::Planet,
    physics::{
        physics_body::physics_body_state::PhysicsBodyState,
        physics_environment::ForceField,
    },
};
use nalgebra::{
    Point3,
    Vector3,
};
use rand::Rng;



pub mod planet;



#[derive(Clone)]
pub struct CelestialBodyContainer
{
    pub bodies: Vec<Planet>,
}



impl CelestialBodyContainer
{
    pub fn new() -> Self
    {
        Self { bodies: vec![] }
    }



    pub fn generate_planets<R: Rng>(&mut self, num: usize, ave_mass: f32, bounds: f32, rng: &mut R)
    {
        for _ in 0..num
        {
            let mass = rng.random_range(0.5 * ave_mass..2.0 * ave_mass);
            let mut planet = Planet {
                radius: 10.0 * mass.powf(1.0 / 3.0),
                mass,
                physics_state: PhysicsBodyState::new(),
            };
            planet.physics_state.translate([
                rng.random_range(-bounds..bounds),
                rng.random_range(-bounds..bounds),
                rng.random_range(-bounds..bounds),
            ]);
            self.bodies.push(planet)
        }
    }
}



impl Default for CelestialBodyContainer
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl ForceField for CelestialBodyContainer
{
    fn sample_force<P: Into<Point3<f32>>>(&self, point: P) -> Vector3<f32>
    {
        let point = point.into();
        let mut force = Vector3::zeros();
        for body in &self.bodies
        {
            let to_center = body.physics_state.get_pos() - point;
            let distance = to_center.magnitude();
            if distance < body.radius
            {
                continue;
            }
            force += to_center * (body.mass / (distance * distance * distance));
        }

        force
    }
}
