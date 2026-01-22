use crate::{
    chunking::{
        blocks::{
            BLOCK_SIZE,
            Block,
            BlockFace,
            block_mesh,
        },
        chunk::{
            CHUNK_BLOCK_SIZE,
            CHUNK_WORLD_SIZE,
            Chunk,
        },
    },
    rendering::mesh::MeshData,
    vertex::TextureVertex,
};
use nalgebra::{
    Point3,
    Vector3,
};



impl MeshData
{
    pub fn from_chunk(chunk: &Chunk) -> Self
    {
        let mut mesh_data = Self {
            vertices: vec![],
            indices: vec![],
            vertex_count: 0,
        };

        if !chunk.empty
        {
            for x in 0..CHUNK_BLOCK_SIZE
            {
                for y in 0..CHUNK_BLOCK_SIZE
                {
                    for z in 0..CHUNK_BLOCK_SIZE
                    {
                        mesh_data.generate_block([x, y, z], chunk);
                    }
                }
            }
        }

        mesh_data
    }



    fn generate_block<P: Into<Point3<u8>>>(&mut self, block_pos: P, chunk: &Chunk)
    {
        let block_pos = block_pos.into();
        let [x, y, z] = block_pos.into();

        let offset = Vector3::new(
            x as f32 * BLOCK_SIZE,
            y as f32 * BLOCK_SIZE,
            z as f32 * BLOCK_SIZE,
        ) + (CHUNK_WORLD_SIZE
            * Vector3::new(
                chunk.world_offset.x as f32,
                chunk.world_offset.y as f32,
                chunk.world_offset.z as f32,
            ));

        let mut num_faces = 0;

        if let Some(block) = chunk.solid_block_at(block_pos)
        {
            // Generate faces on this block
            // Right face of block to left
            if x == 0
            {
                // Check adjacent chunk
            }
            else if !chunk.block_is_solid([x - 1, y, z])
            {
                self.add_face(block, BlockFace::Back, offset);
                num_faces += 1;
            }

            // Top face of block below
            if y == 0
            {
                // Check adjacent chunk
            }
            else if !chunk.block_is_solid([x, y - 1, z])
            {
                self.add_face(block, BlockFace::Bottom, offset);
                num_faces += 1;
            }

            // Front face of block behind
            if z == 0
            {
                // Check adjacent chunk
            }
            else if !chunk.block_is_solid([x, y, z - 1])
            {
                self.add_face(block, BlockFace::Right, offset);
                num_faces += 1;
            }
        }
        else
        {
            // Generate faces on neighboring blocks
            // Right face of block to left
            if x == 0 || x >= CHUNK_BLOCK_SIZE
            {
                // Check adjacent chunk
            }
            else if let Some(block) = chunk.solid_block_at([x - 1, y, z])
            {
                self.add_face(
                    block,
                    BlockFace::Front,
                    offset - Vector3::new(BLOCK_SIZE, 0.0, 0.0),
                );
                num_faces += 1;
            }

            // Top face of block below
            if y == 0 || y >= CHUNK_BLOCK_SIZE
            {
                // Check adjacent chunk
            }
            else if let Some(block) = chunk.solid_block_at([x, y - 1, z])
            {
                self.add_face(
                    block,
                    BlockFace::Top,
                    offset - Vector3::new(0.0, BLOCK_SIZE, 0.0),
                );
                num_faces += 1;
            }

            // Front face of block behind
            if z == 0 || z >= CHUNK_BLOCK_SIZE
            {
                // Check adjacent chunk
            }
            else if let Some(block) = chunk.solid_block_at([x, y, z - 1])
            {
                self.add_face(
                    block,
                    BlockFace::Left,
                    offset - Vector3::new(0.0, 0.0, BLOCK_SIZE),
                );
                num_faces += 1;
            }
        }

        self.add_indices(num_faces);
    }



    fn add_face<T: Into<Vector3<f32>> + Copy>(
        &mut self,
        block: &Block,
        face: BlockFace,
        position: T,
    )
    {
        for vertex in block_mesh::BLOCK_FACE_DATA[face as usize]
            .iter()
            .enumerate()
            .map(|(i, v)| {
                TextureVertex::from_vector_and_block(
                    position.into() + (BLOCK_SIZE * v),
                    i,
                    &block.block_type,
                )
            })
        {
            self.vertices.push(vertex);
        }
    }



    fn add_indices(&mut self, num_faces: u8)
    {
        for _ in 0..num_faces
        {
            self.indices.push(self.vertex_count);
            self.indices.push(1 + self.vertex_count);
            self.indices.push(2 + self.vertex_count);
            self.indices.push(2 + self.vertex_count);
            self.indices.push(3 + self.vertex_count);
            self.indices.push(self.vertex_count);

            self.vertex_count += 4;
        }
    }
}
