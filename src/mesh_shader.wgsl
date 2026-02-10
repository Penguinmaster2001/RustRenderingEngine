
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
    
    let world_position = model_transform * vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.world_position = world_position.xyz;
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    let normal = in.normal;
    
    let view_direction = normalize(camera.view_pos.xyz - in.world_position);
    
    var total_specular = 0.0;
    var total_lambertian = 0.0;
    var total_light_color = vec4(0.0);
    let specular_color = textureSample(t_specular, s_specular, in.tex_coords);
    
    for (var i = 0u; i < lights.sphere_light_count; i++)
    {
        let lightPos = lights.sphere_lights[i].position;
        var light_direction = lightPos - in.world_position;
        let distance = dot(light_direction, light_direction);
        light_direction = normalize(light_direction);
        
        let lambertian = max(dot(light_direction, normal), 0.0);
        total_lambertian += lambertian;
        
        if (lambertian > 0.0)
        {
            let halfDir = normalize(light_direction + view_direction);
            let specAngle = max(dot(halfDir, normal), 0.0);
            let shininess = mix(64.0, 2.0, specular_color.a);
            total_specular += pow(specAngle, shininess);
        }
        
        total_light_color += lights.sphere_lights[i].color * lights.sphere_lights[i].intensity / distance;
    }
    
    for (var i = 0u; i < lights.sun_light_count; i++)
    {
        let light_direction = -normalize(lights.sun_lights[i].direction);
        
        let lambertian = max(dot(light_direction, normal), 0.0);
        total_lambertian += lambertian;
        
        if (lambertian > 0.0)
        {
            let halfDir = normalize(light_direction + view_direction);
            let specAngle = max(dot(halfDir, normal), 0.0);
            let shininess = mix(64.0, 2.0, specular_color.a);
            total_specular += pow(specAngle, shininess);
        }
        
        total_light_color += lights.sun_lights[i].color * lights.sun_lights[i].intensity;
    }
    
    let ambientColor = 0.01 * vec4(0.01, 0.01, 0.02, 1.0);
    let diffuseColor = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let color_linear = ambientColor * diffuseColor
                     + (diffuseColor * total_lambertian + specular_color * total_specular)
                        * total_light_color;
                       
    let screen_gamma = 2.2;
    
    let colorGammaCorrected = pow(max(color_linear, vec4(0.0)), vec4(1.0 / screen_gamma));

    return colorGammaCorrected;
}
