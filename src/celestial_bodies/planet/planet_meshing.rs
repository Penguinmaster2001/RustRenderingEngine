use crate::{
    celestial_bodies::planet::Planet,
    model::Model,
    rendering::{
        mesh::{
            MeshBuffer,
            MeshData,
        },
        renderer::Renderer,
    },
    vertex::ModelVertex,
};



pub struct CelestialMeshContainer
{
    pub models: Vec<Model>,
}



impl CelestialMeshContainer
{
    pub fn new() -> Self
    {
        Self { models: vec![] }
    }



    pub fn add_model(&mut self, model: Model) -> usize
    {
        let id = self.models.len();

        self.models.push(model);

        id
    }



    pub fn add_planets(&mut self, planets: &Vec<Planet>, render_state: &Renderer)
    {
        for planet in planets
        {
            let planet_mesh = MeshBuffer::from_data(&MeshData::from_planet(planet), render_state);
            self.add_model(Model::new(
                vec![planet_mesh],
                planet.physics_state.get_transform(),
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



impl MeshData<ModelVertex>
{
    pub fn from_planet(planet: &Planet) -> Self
    {
        MeshData::new_uv_sphere(planet.radius, [0.0, 0.0, 0.0], 64, 32)
    }
}
