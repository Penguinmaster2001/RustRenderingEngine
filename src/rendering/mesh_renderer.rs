use crate::rendering::mesh::MeshBuffer;



pub trait MeshRenderer
{
    fn get_meshes(&self) -> impl Iterator<Item = &MeshBuffer>;
}



pub trait DrawMeshes<'a>
{
    fn draw_meshes<T: MeshRenderer>(&mut self, mesh_renderer: &T);
}



impl<'a, 'b> DrawMeshes<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_meshes<T: MeshRenderer>(&mut self, mesh_renderer: &T)
    {
        for chunk_mesh in mesh_renderer.get_meshes().filter(|c| c.index_count > 0)
        {
            self.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
            self.set_index_buffer(chunk_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..chunk_mesh.index_count, 0, 0..1);
        }
    }
}
