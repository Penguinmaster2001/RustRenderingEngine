// Vertex shader

struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(1) @interpolate(linear, center) texel_coords: vec4<f32>
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput
{
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.texel_coords = out.clip_position;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>
{
    let offset = vec2<f32>(-0.5);
    let mult = 1.0;
    return vec4<f32>(mult * (in.texel_coords.x - offset.x) , mult * (in.texel_coords.y - offset.y), 0.3, 1.0);
}
