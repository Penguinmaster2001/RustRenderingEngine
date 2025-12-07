use crate::{
    chunking::{
        blocks::BLOCK_SIZE,
        chunk::{
            CHUNK_SIZE,
            Chunk,
        },
    },
    rendering::Renderer,
    vertex::TextureVertex,
};
use cgmath::Vector3;
use wgpu::util::DeviceExt;



fn vert_index(x: u8, y: u8, z: u8) -> u32
{
    ((((x as u32) * (CHUNK_SIZE + 1) as u32) + y as u32) * (CHUNK_SIZE + 1) as u32) + z as u32
}



fn generate_faces(x: u8, y: u8, z: u8, chunk: &Chunk, indices: &mut Vec<u32>)
{
    if chunk.block_is_solid(x, y, z)
    {
        // Front Face
        indices.push(vert_index(x + 0, y + 0, z + 0));
        indices.push(vert_index(x + 1, y + 0, z + 0));
        indices.push(vert_index(x + 1, y + 0, z + 1));

        indices.push(vert_index(x + 0, y + 0, z + 0));
        indices.push(vert_index(x + 1, y + 0, z + 1));
        indices.push(vert_index(x + 0, y + 0, z + 1));
    }
}



fn generate_indices(chunk: &Chunk) -> Vec<u32>
{
    let mut indices = vec![];

    for x in 0..CHUNK_SIZE
    {
        for y in 0..CHUNK_SIZE
        {
            for z in 0..CHUNK_SIZE
            {
                generate_faces(x, y, z, chunk, &mut indices);
            }
        }
    }

    indices
}



fn generate_vertices() -> Vec<TextureVertex>
{
    let mut verts = vec![];

    for x in 0..(CHUNK_SIZE + 1)
    {
        for y in 0..(CHUNK_SIZE + 1)
        {
            for z in 0..(CHUNK_SIZE + 1)
            {
                let offset = Vector3::new(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    z as f32 * BLOCK_SIZE,
                );

                verts.push(TextureVertex {
                    position: [offset.x, offset.y, offset.z],
                    tex_coords: [0.0, 0.0],
                });
            }
        }
    }

    verts
}



pub struct ChunkMesh
{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}



impl ChunkMesh
{
    pub fn from_chunk(chunk: &Chunk, renderer: &Renderer) -> Self
    {
        let vertices = generate_vertices();

        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let indices = generate_indices(chunk);

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("chunk Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as _,
        }
    }
}
