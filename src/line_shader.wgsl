
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
};



struct SunLight {
    direction: vec3<f32>,
    color: vec4<f32>,
    intensity: f32,
};



struct SphereLight {
    position: vec3<f32>,
    color: vec4<f32>,
    intensity: f32,
};



struct Lights
{
    sphere_lights: array<SphereLight, 16>,
    sphere_light_count: u32,
    sun_lights: array<SunLight, 2>,
    sun_light_count: u32,
}



@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var t_specular: texture_2d<f32>;
@group(0) @binding(3)
var s_specular: sampler;



@group(1) @binding(0)
var<uniform> camera: CameraUniform;



@group(2) @binding(0)
var<uniform> lights: Lights;



struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}



struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}



@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    let world_position = vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.world_position = world_position.xyz;
    out.tex_coords = model.tex_coords;
    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    return vec4(0.5, 0.2, 0.8, 1.0);
}
