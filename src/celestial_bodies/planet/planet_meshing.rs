use crate::{
    celestial_bodies::planet::Planet,
    rendering::{
        RenderState,
        mesh::{
            MeshBuffer,
            MeshData,
        },
    },
};



pub struct CelestialMeshContainer
{
    pub meshes: Vec<MeshBuffer>,
}



impl CelestialMeshContainer
{
    pub fn new() -> Self
    {
        Self { meshes: vec![] }
    }



    pub fn add_mesh(&mut self, mesh: MeshBuffer) -> usize
    {
        let id = self.meshes.len();

        self.meshes.push(mesh);

        id
    }



    pub fn add_planets(&mut self, planets: &Vec<Planet>, render_state: &RenderState)
    {
        for planet in planets
        {
            self.add_mesh(MeshBuffer::from_data(
                &MeshData::from_planet(planet),
                render_state,
            ));
        }
    }
}



impl Default for CelestialMeshContainer
{
    fn default() -> Self
    {
        Self::new()
    }
}



impl MeshData
{
    pub fn from_planet(planet: &Planet) -> Self
    {
        MeshData::new_uv_sphere(
            5.0 * planet.mass.powf(1.0 / 3.0),
            *planet.physics_state.get_pos(),
            16,
            32,
        )
    }
}
