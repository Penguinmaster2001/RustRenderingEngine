
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
    let uv = in.tex_coords;
    
    let ndc = vec2<f32>(2.0 * uv.x - 1.0, 2.0 * uv.y - 1.0);
    
    let near = camera.inv_view_proj * vec4<f32>(ndc.x, ndc.y, -1.0, 1.0);
    let far = camera.inv_view_proj * vec4<f32>(ndc.x, ndc.y,  1.0, 1.0);
    
    let near_pos = near.xyz / near.w;
    let far_pos = far.xyz / far.w;
    
    let origin = (camera.inv_view_proj * camera.view_pos).xyz;
    let dir = normalize(far_pos - origin);
    
    var RayPos = origin; // camera.view_pos.xyz;
    var RayDir = dir; // vec3(uv.x, uv.y, -1.0);
    var Time = 0.0;

    RayDir = normalize(RayDir);

    if (TraceGeodesic(&RayPos, &RayDir, &Time))
    {
        return vec4(vec3(0.0), 1.0);
    }
    
    let color = textureSample(t_diffuse, s_diffuse, dir_to_sphere(RayDir));
    
    return color;
}



fn dir_to_sphere(dir: vec3<f32>) -> vec2<f32>
{
    return vec2<f32>(0.5 + atan2(dir.y, dir.x) / TAU, acos(dir.z) / PI);
}



fn diag(a: vec4<f32>) -> mat4x4<f32>
{
    return mat4x4(a.x, 0.0, 0.0, 0.0,
                  0.0, a.y, 0.0, 0.0,
                  0.0, 0.0, a.z, 0.0,
                  0.0, 0.0, 0.0, a.w);
}

fn Metric(x: vec4<f32>) -> mat4x4<f32>
{
    //Kerr-Newman metric in Kerr-Schild coordinates 
    const a = 5.0;
    const m = 0.2;
    const Q = 0.0;
    let p = x.yzw;
    let rho = dot(p,p) - a*a;
    let r2 = 0.5*(rho + sqrt(rho*rho + 4.0*a*a*p.z*p.z));
    let r = sqrt(r2);
    let k = vec4(1, (r*p.x + a*p.y)/(r2 + a*a), (r*p.y - a*p.x)/(r2 + a*a), p.z/r);
    let f = r2*(2.0*m*r - Q*Q)/(r2*r2 + a*a*p.z*p.z);
    return f * mat4x4(k.x * k, k.y * k, k.z * k, k.w * k) + diag(vec4(-1,1,1,1));
}

fn inv_metric(x: vec4<f32>) -> mat4x4<f32>
{
    return inverse_sym(Metric(x));
}

fn Hamiltonian(x: vec4<f32>, p: vec4<f32>) -> f32
{
    let g_inv = inv_metric(x);
    return 0.5 * dot(g_inv * p, p);
}



/*
float Lagrangian(vec4 x, vec4 dxdt)
{
    return 0.5*dot(Metric(x)*dxdt,dxdt);
}
*/



fn HamiltonianGradient(x: ptr<function, vec4<f32>>, p: ptr<function, vec4<f32>>) -> vec4<f32>
{
    const eps = 0.0001;
    return (vec4(Hamiltonian(*x + vec4(eps, 0.0, 0.0, 0.0), *p),
                 Hamiltonian(*x + vec4(0.0, eps, 0.0, 0.0), *p),
                 Hamiltonian(*x + vec4(0.0, 0.0, eps, 0.0), *p),
                 Hamiltonian(*x + vec4(0.0, 0.0, 0.0, eps), *p)) - Hamiltonian(*x, *p)) / eps;
}



fn IntegrationStep(x: ptr<function, vec4<f32>>, p: ptr<function, vec4<f32>>)
{
    const TimeStep = 0.05;
    *p = *p - TimeStep * HamiltonianGradient(x, p);
    *x = *x + TimeStep * inv_metric(*x) * *p;
}



fn GetNullMomentum(x: vec4<f32>, dir: vec3<f32>) -> vec4<f32>
{
    return Metric(x) * vec4(1.0, normalize(dir));
}



fn GetDirection(x: vec4<f32>, p: vec4<f32>) -> vec3<f32>
{
    let dxdt = inv_metric(x) * p;
    return normalize(dxdt.yzw);
}



fn TraceGeodesic(pos: ptr<function, vec3<f32>>, dir: ptr<function, vec3<f32>>, time: ptr<function, f32>) -> bool
{
    var x = vec4(*time, *pos);
    var p = GetNullMomentum(x, *dir);

    const steps = 256;
    for(var i = 0; i < steps; i++)
    {
        IntegrationStep(&x, &p);
    }

    *pos = x.yzw;
    *time = x.x;
    *dir = GetDirection(x, p);

    return false;
}



fn inverse_sym(m: mat4x4<f32>) -> mat4x4<f32>
{
	var n11 = m[0][0]; var n12 = m[1][0]; var n13 = m[2][0]; var n14 = m[3][0];
	var n22 = m[1][1]; var n23 = m[2][1]; var n24 = m[3][1];
	var n33 = m[2][2]; var n34 = m[3][2];
	var n44 = m[3][3];

	var t11 = 2.0 * n23 * n34 * n24 - n24 * n33 * n24 - n22 * n34 * n34 - n23 * n23 * n44 + n22 * n33 * n44;
	var t12 = n14 * n33 * n24 - n13 * n34 * n24 - n14 * n23 * n34 + n12 * n34 * n34 + n13 * n23 * n44 - n12 * n33 * n44;
	var t13 = n13 * n24 * n24 - n14 * n23 * n24 + n14 * n22 * n34 - n12 * n24 * n34 - n13 * n22 * n44 + n12 * n23 * n44;
	var t14 = n14 * n23 * n23 - n13 * n24 * n23 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34;

	var det = n11 * t11 + n12 * t12 + n13 * t13 + n14 * t14;
	var idet = 1.0f / det;

	var ret: mat4x4<f32>;

	ret[0][0] = t11 * idet;
	ret[0][1] = (n24 * n33 * n14 - n23 * n34 * n14 - n24 * n13 * n34 + n12 * n34 * n34 + n23 * n13 * n44 - n12 * n33 * n44) * idet;
	ret[0][2] = (n22 * n34 * n14 - n24 * n23 * n14 + n24 * n13 * n24 - n12 * n34 * n24 - n22 * n13 * n44 + n12 * n23 * n44) * idet;
	ret[0][3] = (n23 * n23 * n14 - n22 * n33 * n14 - n23 * n13 * n24 + n12 * n33 * n24 + n22 * n13 * n34 - n12 * n23 * n34) * idet;

	ret[1][0] = ret[0][1];
	ret[1][1] = (2.0 * n13 * n34 * n14 - n14 * n33 * n14 - n11 * n34 * n34 - n13 * n13 * n44 + n11 * n33 * n44) * idet;
	ret[1][2] = (n14 * n23 * n14 - n12 * n34 * n14 - n14 * n13 * n24 + n11 * n34 * n24 + n12 * n13 * n44 - n11 * n23 * n44) * idet;
	ret[1][3] = (n12 * n33 * n14 - n13 * n23 * n14 + n13 * n13 * n24 - n11 * n33 * n24 - n12 * n13 * n34 + n11 * n23 * n34) * idet;

	ret[2][0] = ret[0][2];
	ret[2][1] = ret[1][2];
    ret[2][2] = (2.0 * n12 * n24 * n14 - n14 * n22 * n14 - n11 * n24 * n24 - n12 * n12 * n44 + n11 * n22 * n44) * idet;
	ret[2][3] = (n13 * n22 * n14 - n12 * n23 * n14 - n13 * n12 * n24 + n11 * n23 * n24 + n12 * n12 * n34 - n11 * n22 * n34) * idet;

	ret[3][0] = ret[0][3];
	ret[3][1] = ret[1][3];
	ret[3][2] = ret[2][3];
	ret[3][3] = (2.0 * n12 * n23 * n13 - n13 * n22 * n13 - n11 * n23 * n23 - n12 * n12 * n33 + n11 * n22 * n33) * idet;

	return ret;
}
