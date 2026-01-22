use crate::{
    rendering::mesh::MeshData,
    vertex::TextureVertex,
};
use nalgebra::{
    Point3,
    Vector3,
};
use std::f32::consts::PI;



impl MeshData
{
    pub fn new_uv_sphere<P: Into<Point3<f32>>>(
        radius: f32,
        center: P,
        lats: usize,
        longs: usize,
    ) -> Self
    {
        let center = center.into();
        let vertex_count = lats * longs;
        let mut vertices = Vec::with_capacity(vertex_count);
        let mut indices = Vec::with_capacity(vertex_count * 3);

        let mut this_row = 0;
        let mut prev_row = 0;
        let mut point = 0;

        for lat in 0..=lats
        {
            let v = lat as f32 / lats as f32;
            let (w, y) = f32::sin_cos(PI * v);

            for long in 0..=longs
            {
                let u = long as f32 / longs as f32;
                let (x, z) = f32::sin_cos(u * PI * 2.0);

                let vert = TextureVertex {
                    position: (center + Vector3::new(x * radius * w, y * radius, z * radius * w))
                        .into(),
                    tex_coords: [u, v],
                };

                vertices.push(vert);
                // normals.Add(vert.Normalized());
                point += 1;

                if lat > 0 && long > 0
                {
                    indices.push((prev_row + long - 1) as u32);
                    indices.push((prev_row + long) as u32);
                    indices.push((this_row + long - 1) as u32);

                    indices.push((prev_row + long) as u32);
                    indices.push((this_row + long) as u32);
                    indices.push((this_row + long - 1) as u32);
                }
            }

            prev_row = this_row;
            this_row = point;
        }


        Self {
            vertices,
            indices,
            vertex_count: vertex_count as u32,
        }
    }
}
