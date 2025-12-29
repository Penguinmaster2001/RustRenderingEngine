
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
};



struct SunLight {
    direction: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};



struct SphereLight {
    position: vec3<f32>,
    color: vec4<f32>,
    intensity: f32,
};



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
var<uniform> sphereLights: array<SphereLight, 1>;



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
    let dpdx = dpdx(in.world_position);
    let dpdy = dpdy(in.world_position);
    let normal = normalize(cross(dpdy, dpdx));
    
    // return vec4(normal, 1.0);
    
    let lightPos = sphereLights[0].position;
    var lightDir = lightPos - in.world_position;
    let distance = dot(lightDir, lightDir);
    lightDir = normalize(lightDir);
  
    let lambertian = max(dot(lightDir, normal), 0.0);
    var specular = 0.0;
    var specular_color = textureSample(t_specular, s_specular, in.tex_coords);
  
    if (lambertian > 0.0)
    {
      let viewDir = normalize(camera.view_pos.xyz - in.world_position);
  
      let halfDir = normalize(lightDir + viewDir);
      let specAngle = max(dot(halfDir, normal), 0.0);
      let shininess = mix(64.0, 2.0, specular_color.a);
      specular = pow(specAngle, shininess);
    }
    
    let ambientColor = 0.0 * vec4(1.0, 1.0, 1.0, 1.0);
    let diffuseColor = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let lightColor = sphereLights[0].color;
    let lightPower = sphereLights[0].intensity;
    let specColor = specular * specular_color;
    let colorLinear = ambientColor
                       + diffuseColor * lambertian * lightColor * lightPower / distance
                       + specColor * specular * lightColor * lightPower / distance;
                       
    let screenGamma = 2.2;
    
    let colorGammaCorrected = pow(max(colorLinear, vec4(0.0)), vec4(1.0 / screenGamma));

    return colorGammaCorrected;
}
