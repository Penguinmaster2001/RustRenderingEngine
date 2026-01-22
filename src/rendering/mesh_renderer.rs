use crate::rendering::mesh::MeshBuffer;



pub trait DrawMeshes<'a>
{
    fn draw_meshes<'m, I: Iterator<Item = &'m MeshBuffer>>(&mut self, meshes: I);
}



impl<'a, 'b> DrawMeshes<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_meshes<'m, I: Iterator<Item = &'m MeshBuffer>>(&mut self, meshes: I)
    {
        for mesh in meshes.filter(|c| c.index_count > 0)
        {
            self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }
}
