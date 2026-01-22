use crate::{
    rendering::mesh_renderer::{
        DrawMeshes,
        MeshRenderer,
    },
    state::State,
};



pub struct Renderer<'r>
{
    pub mesh_renderers: Vec<&'r dyn MeshRenderer>,
}



impl<'r> Renderer<'r>
{
    pub fn render(&self, state: &State) -> Result<(), wgpu::SurfaceError>
    {
        state.renderer_state.window.request_redraw();

        // We can't render unless the surface is configured
        if !state.renderer_state.is_surface_configured
        {
            return Ok(());
        }

        let output = state.renderer_state.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            state
                .renderer_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.006,
                            g: 0.006,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &state.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&state.render_pipeline);

            render_pass.set_bind_group(0, &state.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &state.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &state.light_bind_group, &[]);

            for mesh_renderer in &self.mesh_renderers
            {
                render_pass.draw_meshes(*mesh_renderer);
            }
        }

        state
            .renderer_state
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
