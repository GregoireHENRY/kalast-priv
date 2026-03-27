use wgpu::util::DeviceExt;

use crate::gpu::scene::CreateBuffer;

pub type Vertex = crate::mesh::Vertex;

pub struct Mesh {
    pub inner: crate::mesh::Mesh,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x3,
        6 => Uint32,
    ];

    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub state: super::scene::ModelState,
    pub state_buffer: wgpu::Buffer,
}

impl Model {
    pub fn load(
        config: &super::config::ConfigModel,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        state: super::scene::ModelState,
    ) -> Self {
        let path = std::path::Path::new(&config.path);

        let crate::mesh::Model {
            mut meshes,
            mut materials,
        } = crate::mesh::Model::load(path, |p| p * config.pos_factor);

        let materials = materials
            .drain(..)
            .map(|m| {
                let diffuse_texture =
                    super::texture::Texture::from_image(m.diffuse, device, queue, false).unwrap();
                let normal_texture =
                    super::texture::Texture::from_image(m.normal, device, queue, false).unwrap();
                Material::new(diffuse_texture, normal_texture, device, layout)
            })
            .collect();

        let meshes = meshes
            .drain(..)
            .map(|mut mesh| {
                if let Some(color_mode) = config.color_mode {
                    mesh.update_colors(color_mode, config.color);
                }

                if config.flat {
                    mesh.flatten();
                    println!("flattened")
                }

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&mesh.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                Mesh {
                    inner: mesh,
                    vertex_buffer,
                    index_buffer,
                }
            })
            .collect();

        Self {
            meshes,
            materials,
            state,
            state_buffer: state.create_buffer(device),
        }
    }

    pub fn update_buffer(&mut self, device: &wgpu::Device) {
        self.state_buffer = self.state.create_buffer(device);
    }
}

pub struct Material {
    pub diffuse_texture: super::texture::Texture,
    pub normal_texture: super::texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        diffuse_texture: super::texture::Texture,
        normal_texture: super::texture::Texture,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                },
            ],
            label: None,
        });

        Self {
            diffuse_texture,
            normal_texture,
            bind_group,
        }
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: Option<&'a Material>,
        globals: &'a wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: Option<&'a Material>,
        instances: std::ops::Range<u32>,
        globals: &'a wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_model(
        &mut self,
        model: &'a Model,
        globals: &'a wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: std::ops::Range<u32>,
        globals: &'a wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_model_instanced_with_material(
        &mut self,
        model: &'a Model,
        material: &'a Material,
        instances: std::ops::Range<u32>,
        globals: &'a wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: Option<&'a Material>,
        globals: &'b wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, globals, camera, light);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: Option<&'a Material>,
        instances: std::ops::Range<u32>,
        globals: &'b wgpu::BindGroup,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, globals, &[]);
        self.set_bind_group(1, camera, &[]);
        self.set_bind_group(2, light, &[]);

        self.set_stencil_reference(1);

        if let Some(material) = material {
            self.set_bind_group(3, &material.bind_group, &[]);
        }

        if mesh.inner.is_flat() {
            self.draw(0..mesh.inner.vertices.len() as _, instances);
        } else {
            self.draw_indexed(0..mesh.inner.indices.len() as _, 0, instances);
        }
    }

    fn draw_model(
        &mut self,
        model: &'b Model,
        globals: &'b wgpu::BindGroup,
        camera: &'b wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.draw_model_instanced(model, 0..1, globals, camera, light);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: std::ops::Range<u32>,
        globals: &'b wgpu::BindGroup,
        camera: &'b wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = mesh.inner.material_id.map(|id| &model.materials[id]);
            self.draw_mesh_instanced(mesh, material, instances.clone(), globals, camera, light);
        }
    }

    fn draw_model_instanced_with_material(
        &mut self,
        model: &'b Model,
        material: &'b Material,
        instances: std::ops::Range<u32>,
        globals: &'b wgpu::BindGroup,
        camera: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_mesh_instanced(
                mesh,
                Some(material),
                instances.clone(),
                globals,
                camera,
                light,
            );
        }
    }
}
