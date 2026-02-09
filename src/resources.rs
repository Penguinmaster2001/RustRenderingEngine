use crate::{
    model::{
        self,
        Material,
        TransformUniform,
    },
    rendering::{
        mesh::MeshBuffer,
        renderer::Renderer,
    },
    texture,
    vertex::ModelVertex,
};
use std::{
    io::{
        BufReader,
        Cursor,
    },
    path::{
        Path,
        PathBuf,
    },
};
use wgpu::util::DeviceExt;



pub async fn load_string(path: &PathBuf) -> anyhow::Result<String>
{
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}



pub async fn load_binary(path: &PathBuf) -> anyhow::Result<Vec<u8>>
{
    let data = std::fs::read(path)?;

    Ok(data)
}



pub async fn load_texture(
    path: &PathBuf,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture>
{
    let data = load_binary(path).await?;
    texture::Texture::from_bytes(
        device,
        queue,
        &data,
        path.file_name().unwrap().to_str().unwrap(),
    )
}



pub async fn load_model(
    path: &str,
    renderer: &Renderer,
    // layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<model::Model>
{
    let path = Path::new(path);
    let file_name = path.file_name().unwrap();
    let base = Path::new(env!("OUT_DIR")).join(path.parent().unwrap());
    let base = base.as_path();
    let obj_text = load_string(&base.join(file_name)).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, _obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            println!("{}", p);
            let mat_text = load_string(&base.join(p)).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut _materials: Vec<Material> = Vec::new();
    // for m in obj_materials?
    // {
    //     let diffuse_texture = load_texture(&base.join(m.diffuse_texture), device, queue).await?;
    //     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         layout,
    //         entries: &[
    //             wgpu::BindGroupEntry {
    //                 binding: 0,
    //                 resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
    //             },
    //             wgpu::BindGroupEntry {
    //                 binding: 1,
    //                 resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
    //             },
    //         ],
    //         label: None,
    //     });

    //     materials.push(model::Material {
    //         name: m.name,
    //         diffuse_texture,
    //         bind_group,
    //     })
    // }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    if m.mesh.normals.is_empty()
                    {
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [0.0, 0.0, 0.0],
                        }
                    }
                    else
                    {
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>();

            let vertex_buffer =
                renderer
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("{:?} Vertex Buffer", file_name)),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
            let index_buffer =
                renderer
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("{:?} Index Buffer", file_name)),
                        contents: bytemuck::cast_slice(&m.mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

            // model::ModelMesh {
            //     name: file_name.to_str().unwrap().to_owned(),
            //     vertex_buffer,
            //     index_buffer,
            //     num_elements: m.mesh.indices.len() as u32,
            //     material: m.mesh.material_id.unwrap_or(0),
            // }

            MeshBuffer {
                vertex_buffer,
                index_buffer,
                index_count: m.mesh.indices.len() as u32,
            }
        })
        .collect::<Vec<_>>();

    Ok(model::Model {
        meshes,
        transform: TransformUniform::new(),
    })
}
