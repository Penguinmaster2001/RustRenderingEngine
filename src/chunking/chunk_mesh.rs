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



fn generate_vertices(chunk: &Chunk) -> (Vec<TextureVertex>, Vec<u32>)
{
    let mut verts = vec![];
    let mut indices = vec![];

    for x in 0..CHUNK_SIZE
    {
        for y in 0..CHUNK_SIZE
        {
            for z in 0..CHUNK_SIZE
            {
                let offset = Vector3::new(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    z as f32 * BLOCK_SIZE,
                );

                let index = verts.len() as u32;

                verts.push(TextureVertex {
                    position: [offset.x, offset.y, offset.z],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x, offset.y, offset.z + BLOCK_SIZE],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x, offset.y + BLOCK_SIZE, offset.z],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x, offset.y + BLOCK_SIZE, offset.z + BLOCK_SIZE],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x + BLOCK_SIZE, offset.y, offset.z],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x + BLOCK_SIZE, offset.y, offset.z + BLOCK_SIZE],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [offset.x + BLOCK_SIZE, offset.y + BLOCK_SIZE, offset.z],
                    tex_coords: [0.0, 0.0],
                });

                verts.push(TextureVertex {
                    position: [
                        offset.x + BLOCK_SIZE,
                        offset.y + BLOCK_SIZE,
                        offset.z + BLOCK_SIZE,
                    ],
                    tex_coords: [0.0, 0.0],
                });

                indices.push(index + 0);
                indices.push(index + 4);
                indices.push(index + 6);

                indices.push(index + 0);
                indices.push(index + 6);
                indices.push(index + 2);

                indices.push(index + 4);
                indices.push(index + 5);
                indices.push(index + 7);

                indices.push(index + 4);
                indices.push(index + 7);
                indices.push(index + 6);

                indices.push(index + 1);
                indices.push(index + 5);
                indices.push(index + 4);

                indices.push(index + 1);
                indices.push(index + 4);
                indices.push(index + 0);
            }
        }
    }

    (verts, indices)
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
        let (vertices, indices) = generate_vertices(chunk);

        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

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
