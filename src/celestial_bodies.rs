use crate::{
    celestial_bodies::planet::Planet,
    physics::physics_body_state::PhysicsBodyState,
};
use nalgebra::{
    Point3,
    Vector3,
};
use rand::Rng;



pub mod planet;



pub struct CelestialBodyContainer
{
    bodies: Vec<Planet>,
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
            let mut planet = Planet {
                mass: rng.random_range(0.8 * ave_mass..1.2 * ave_mass),
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



impl Default for CelestialBodyContainer
{
    fn default() -> Self
    {
        Self::new()
    }
}
