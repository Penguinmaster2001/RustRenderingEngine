use crate::{
    rendering::{
        RenderState,
        mesh_renderer::DrawMeshes,
        render_pass_data::RenderData,
    },
    texture,
    vertex::{
        TextureVertex,
        Vertex,
    },
};
use wgpu;



pub struct Renderer
{
    pub render_state: RenderState,
}



impl Renderer
{
    pub fn new(render_state: RenderState) -> Self
    {
        Self { render_state }
    }



    pub fn render(&self, data: &RenderData) -> Result<(), wgpu::SurfaceError>
    {
        self.render_state.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.render_state.is_surface_configured
        {
            return Ok(());
        }

        let output = self.render_state.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.render_state
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
                    view: &data.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for (index, bind_group) in data.bind_groups.iter().enumerate()
            {
                render_pass.set_bind_group(index as u32, bind_group, &[]);
            }

            // data.geometries.sort_by_key(|g| g.pipeline_id);
            for c in data
                .geometries
                .chunk_by(|g1, g2| g1.pipeline_id == g2.pipeline_id)
            {
                render_pass.set_pipeline(&data.pipelines[c[0].pipeline_id]);
                for mesh in c.iter().map(|g| &g.meshes)
                {
                    render_pass.draw_meshes(mesh.iter());
                }
            }

            // render_pass.set_pipeline(&state.render_pipeline);

            // render_pass.set_bind_group(0, &state.diffuse_bind_group, &[]);
            // render_pass.set_bind_group(1, &state.camera_bind_group, &[]);
            // render_pass.set_bind_group(2, &state.light_bind_group, &[]);

            // render_pass.draw_meshes(meshes);

            // render_pass.set_pipeline(&state.line_render_pipeline);

            // render_pass.draw_meshes(lines);
        }

        self.render_state
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }



    pub fn create_line_pipeline(
        renderer_state: &RenderState,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline
    {
        let render_pipeline_layout =
            renderer_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Line Render Pipeline Layout"),
                    bind_group_layouts: &[
                        texture_bind_group_layout,
                        camera_bind_group_layout,
                        light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        renderer_state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Line Render Pipeline"),
                layout: Some(&render_pipeline_layout),

                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    buffers: &[TextureVertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },

                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: renderer_state.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                depth_stencil: Some(wgpu::DepthStencilState {
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),

                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },

                multiview: None,
                cache: None,
            })
    }
}
