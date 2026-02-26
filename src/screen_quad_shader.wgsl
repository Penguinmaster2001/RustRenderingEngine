
struct CameraUniform {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<u32>,
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



@group(3) @binding(0)
var<uniform> model_transform: mat4x4<f32>;



struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}



struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}



@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    let world_position = vec4<f32>(model.position, 1.0);
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    return out;
}



const PI = 3.14159265359;
const TAU = 6.28318530718;



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    let uv = in.tex_coords;  // Use interpolated tex_coords
    
    // NDC from UV (Y flipped for viewport origin top-left)
    let ndc = vec2<f32>(2.0 * uv.x - 1.0, 2.0 * uv.y - 1.0);
    
    // Unproject near/far planes to world space
    let near = camera.inv_view_proj * vec4<f32>(ndc.x, ndc.y, -1.0, 1.0);
    let far = camera.inv_view_proj * vec4<f32>(ndc.x, ndc.y,  1.0, 1.0);
    
    let near_pos = near.xyz / near.w;
    let far_pos = far.xyz / far.w;
    
    // Ray: origin at camera, direction towards far
    let origin = camera.view_pos.xyz;
    let dir = normalize(far_pos - origin);
    
    // Sample environment map (equirectangular)
    let color = textureSample(t_diffuse, s_diffuse, dir_to_sphere(dir));
    
    return color;  // Full color, no arbitrary discard
}



fn dir_to_sphere(dir: vec3<f32>) -> vec2<f32>
{
    return vec2<f32>(0.5 + atan2(dir.y, dir.x) / TAU, acos(dir.z) / PI);
}
