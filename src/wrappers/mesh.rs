use std::mem::size_of;

use crate::engine::UnsafeEngine;

use super::{
    attribute_pointer::{AttributePointers, Attributes},
    buffer_object::BufferObject,
    shader::Shader,
    textures::{BuilderTexture2D, Texture2D},
    types::{Index, Vec2, Vec3},
    vertex_array::VertexArray,
};

pub trait Draw {
    fn draw(&self, _: &UnsafeEngine);
}

#[derive(Debug, Clone)]
pub struct BoundStaticMesh {
    vao: VertexArray,
    texture: Vec<Texture2D>,
}

impl Draw for BoundStaticMesh {
    fn draw(&self, _: &UnsafeEngine) {
        self.texture
            .iter()
            .enumerate()
            .for_each(|(idx, t)| t.bind(idx.into()));
        self.vao.bind();
        self.vao.draw();
    }
}

#[derive(Debug, Clone)]
pub struct Mesh<'a> {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
    texture: Vec<BuilderTexture2D<'a>>,
}

impl<'a> Mesh<'a> {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<Index>,
        texture: Vec<BuilderTexture2D<'a>>,
    ) -> Self {
        Self {
            vertices,
            indices,
            texture,
        }
    }

    pub fn create_static(self, shader: &Shader) -> BoundStaticMesh {
        let Mesh {
            vertices,
            indices,
            texture,
        } = self;

        let texture = texture
            .into_iter()
            .enumerate()
            .map(|(idx, t)| t.process(shader, idx))
            .collect();

        let vbo = BufferObject::vertex_buffer_object();
        let ebo = BufferObject::element_buffer_object();
        let mut vao = VertexArray::new(vbo, ebo);

        vao.bind();
        vao.upload_data(&vertices, &indices);
        Vertex::get_attributes()
            .iter()
            .for_each(|v| vao.vertex_attrib_pointer(*v));

        BoundStaticMesh { vao, texture }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub col: Vec3,
    pub tex: Vec2,
}

impl Attributes for Vertex {
    fn get_attributes() -> Vec<AttributePointers> {
        let size = size_of::<Vertex>() as _;
        vec![
            AttributePointers {
                size: 3,
                stride: size,
                ..AttributePointers::empty()
            },
            AttributePointers {
                location: 1,
                size: 3,
                stride: size,
                ptr: size_of::<Vec3>() as _,
                ..AttributePointers::empty()
            },
            AttributePointers {
                location: 2,
                size: 2,
                stride: size,
                ptr: (size_of::<Vec3>() * 2) as _,
                ..AttributePointers::empty()
            },
        ]
    }
}
