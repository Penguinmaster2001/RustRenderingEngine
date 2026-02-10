use crate::{
    model::TransformUniform,
    rendering::{
        mesh_renderer::DrawMeshes,
        render_pass_data::RenderData,
    },
    texture,
};
use std::sync::Arc;
use wgpu;
use winit::window::Window;



pub struct Renderer
{
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Arc<Window>,
    pub is_surface_configured: bool,
}



impl Renderer
{
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self>
    {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            is_surface_configured: false,
        })
    }



    pub fn resize(&mut self, width: u32, height: u32)
    {
        if width > 0 && height > 0
        {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }


    pub fn create_render_pipeline<'a>(
        &self,
        label: &str,
        layout: &wgpu::PipelineLayout,
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
        topology: wgpu::PrimitiveTopology,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline
    {
        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(layout),

                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    buffers,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },

                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),

                primitive: wgpu::PrimitiveState {
                    topology,
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



    pub fn render(&self, data: &RenderData) -> Result<(), wgpu::SurfaceError>
    {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured
        {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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
                if index == 3
                {
                    continue;
                }
                render_pass.set_bind_group(index as u32, bind_group, &[]);
            }

            let padded_uniform_size = TransformUniform::padded_uniform_size(self);

            let mut transform_uniforms = Vec::with_capacity(64);
            for c in data
                .geometries
                .chunk_by(|g1, g2| g1.pipeline_id == g2.pipeline_id)
            {
                for model_vec in c
                {
                    for model in &model_vec.models
                    {
                        transform_uniforms.push(model.transform);
                    }
                }
            }

            self.queue.write_buffer(
                &data.transform_buffer,
                0,
                bytemuck::cast_slice(
                    transform_uniforms
                        .iter()
                        .map(|t| [*t; 4])
                        .collect::<Vec<[TransformUniform; 4]>>()
                        .as_slice(),
                ),
            );

            let mut model_index = 0;
            for c in data
                .geometries
                .chunk_by(|g1, g2| g1.pipeline_id == g2.pipeline_id)
            {
                render_pass.set_pipeline(&data.pipelines[c[0].pipeline_id]);
                for model_vec in c
                {
                    for model in &model_vec.models
                    {
                        let offset_bytes = model_index * padded_uniform_size;
                        render_pass.set_bind_group(3, &data.bind_groups[3], &[offset_bytes as u32]);
                        render_pass.draw_meshes(model.meshes.iter());

                        // println!("{offset_bytes}\t{:?}", model.transform);
                        model_index += 1;
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }



    pub fn create_shaders<const N: usize>(&self, files: &[&str; N]) -> [wgpu::ShaderModule; N]
    {
        let mut n = -1;
        files.map(|f| {
            n += 1;
            self.device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(format!("shader_{}", n).as_str()),
                    source: wgpu::ShaderSource::Wgsl(f.into()),
                })
        })
    }
}
