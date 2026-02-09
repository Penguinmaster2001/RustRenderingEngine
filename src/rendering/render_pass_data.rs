use crate::{
    model::Model,
    rendering::lighting::{
        SphereLight,
        SunLight,
    },
    texture::Texture,
};
use wgpu::{
    BindGroup,
    Buffer,
    RenderPipeline,
};



pub struct LightingGroup
{
    pub sphere_lights: Vec<SphereLight>,
    pub sun_light: Vec<SunLight>,
}



pub struct GeometryGroup
{
    pub models: Vec<Model>,
    pub pipeline_id: usize,
}



pub struct RenderData
{
    pub camera_buffer: Buffer,
    pub transform_buffer: Buffer,
    pub bind_groups: Vec<BindGroup>,
    pub pipelines: Vec<RenderPipeline>,
    pub geometries: Vec<GeometryGroup>,
    pub depth_texture: Texture,
}



impl RenderData
{
    pub fn new(
        camera_buffer: Buffer,
        transform_buffer: Buffer,
        bind_groups: Vec<BindGroup>,
        pipelines: Vec<RenderPipeline>,
        geometries: Vec<GeometryGroup>,
        depth_texture: Texture,
    ) -> Self
    {
        Self {
            camera_buffer,
            transform_buffer,
            bind_groups,
            pipelines,
            geometries,
            depth_texture,
        }
    }
}
