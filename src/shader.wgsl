// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
};

@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

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



// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    let dpdx = dpdx(in.world_position);
    let dpdy = dpdy(in.world_position);
    let normal = normalize(cross(dpdy, dpdx));
    
    // return vec4(normal, 1.0);
    
    let lightPos = 20.0 * vec3(-0.5, 1.5, 0.3);
    var lightDir = lightPos - in.world_position;
    let distance = dot(lightDir, lightDir);
    lightDir = normalize(lightDir);
  
    let lambertian = max(dot(lightDir, normal), 0.0);
    // var specular = 0.0;
  
    // if (lambertian > 0.0)
    // {
    //   let viewDir = normalize(-in.world_position);
  
    //   let halfDir = normalize(lightDir + viewDir);
    //   let specAngle = max(dot(halfDir, normal), 0.0);
    //   let shininess = 1.0;
    //   specular = pow(specAngle, shininess);
    // }
    
    let ambientColor = 0.01 * vec4(1.0, 1.0, 1.0, 1.0);
    let diffuseColor = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let lightColor = vec4(1.0);
    let lightPower = 100.0;
    // let specColor = vec4(1.0);
    let colorLinear = ambientColor
                       + diffuseColor * lambertian * lightColor * lightPower / distance;
                       // + specColor * specular * lightColor * lightPower / distance;
                       
    let screenGamma = 1.5;
    
    let colorGammaCorrected = pow(colorLinear, vec4(1.0 / screenGamma));

    return colorGammaCorrected;
}
