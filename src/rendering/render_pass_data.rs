use crate::{
    rendering::{
        lighting::{
            SphereLight,
            SunLight,
        },
        mesh::MeshBuffer,
    },
    texture::Texture,
};
use wgpu::{
    BindGroup,
    RenderPipeline,
};



pub struct LightingGroup
{
    pub sphere_lights: Vec<SphereLight>,
    pub sun_light: Vec<SunLight>,
}



pub struct GeometryGroup
{
    pub meshes: Vec<MeshBuffer>,
    pub pipeline_id: usize,
}



pub struct RenderData
{
    pub bind_groups: Vec<BindGroup>,
    pub pipelines: Vec<RenderPipeline>,
    pub geometries: Vec<GeometryGroup>,
    pub depth_texture: Texture,
}



impl RenderData
{
    pub fn new(
        bind_groups: Vec<BindGroup>,
        pipelines: Vec<RenderPipeline>,
        geometries: Vec<GeometryGroup>,
        depth_texture: Texture,
    ) -> Self
    {
        Self {
            bind_groups,
            pipelines,
            geometries,
            depth_texture,
        }
    }
}
