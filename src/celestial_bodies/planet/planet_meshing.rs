use crate::{
    celestial_bodies::planet::Planet,
    rendering::mesh::MeshData,
};



impl MeshData
{
    pub fn from_planet(planet: &Planet) -> Self
    {
        MeshData::new_uv_sphere(planet.mass, *planet.physics_state.get_pos(), 12, 12)
    }
}
