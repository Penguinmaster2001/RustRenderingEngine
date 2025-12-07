use crate::{
    blocks::{
        BLOCK_SIZE,
        Block,
    },
    rendering::Renderer,
    vertex::TextureVertex,
};
use cgmath::Vector3;
use wgpu::util::DeviceExt;



const CHUNK_SIZE: usize = 6;
const CHUNK_BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;



pub struct Chunk
{
    blocks: [Block; CHUNK_BLOCK_COUNT],
}



pub struct ChunkMesh
{
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}


fn generate_vertices() -> (Vec<TextureVertex>, Vec<u32>)
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



impl Chunk
{
    pub fn new() -> Self
    {
        let blocks = [Block::new(1); CHUNK_BLOCK_COUNT];

        Self { blocks }
    }



    pub fn block_at(&self, x: u16, y: u16, z: u16) -> Block
    {
        self.blocks
            [((x as usize) * CHUNK_SIZE * CHUNK_SIZE) + ((y as usize) * CHUNK_SIZE) + (z as usize)]
    }
}



impl ChunkMesh
{
    pub fn from_chunk(chunk: &Chunk, renderer: &Renderer) -> Self
    {
        let (vertices, indices) = generate_vertices();

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



pub trait DrawChunk<'a>
{
    fn draw_chunk(&mut self, chunk_mesh: &ChunkMesh);
}



impl<'a, 'b> DrawChunk<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_chunk(&mut self, chunk_mesh: &ChunkMesh)
    {
        self.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
        self.set_index_buffer(chunk_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..chunk_mesh.index_count, 0, 0..1);
    }
}
