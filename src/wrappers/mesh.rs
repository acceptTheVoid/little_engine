use std::mem::size_of;

use engine_math::Vec3;

use crate::engine::UnsafeEngine;

use super::{
    attribute_pointer::{AttributePointers, Attributes},
    buffer_object::BufferObject,
    types::Index,
    vertex_array::VertexArray,
};

pub trait Draw {
    fn draw(&self, _: &UnsafeEngine);
}

#[derive(Debug, Clone)]
pub struct BoundStaticMesh {
    vao: VertexArray,
}

impl Draw for BoundStaticMesh {
    fn draw(&self, _: &UnsafeEngine) {
        self.vao.bind();
        self.vao.draw();
    }
}

#[derive(Debug, Clone)]
pub struct BoundDynamicMesh {
    // TODO
    // vertices: Vec<Vertex>,
    // indices: Vec<Index>,
    vao: VertexArray,
}

impl Draw for BoundDynamicMesh {
    fn draw(&self, _: &UnsafeEngine) {
        self.vao.bind();
        self.vao.draw();
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Index>) -> Self {
        Self { vertices, indices }
    }

    pub fn create_static(self, e: &UnsafeEngine) -> BoundStaticMesh {
        let Mesh { vertices, indices } = self;

        let vbo = BufferObject::vertex_buffer_object();
        let ebo = BufferObject::element_buffer_object();
        let mut vao = VertexArray::new(vbo, ebo);

        vao.bind();
        vao.upload_data(&vertices, &indices);
        Vertex::get_attributes()
            .iter()
            .for_each(|v| vao.vertex_attrib_pointer(*v));

        BoundStaticMesh { vao }
    }

    #[allow(unuse)]
    pub fn create_dynamic(self, _: &UnsafeEngine) -> BoundDynamicMesh {
        let Mesh { vertices, indices } = self;

        let vbo = BufferObject::vertex_buffer_object();
        let ebo = BufferObject::element_buffer_object();
        let mut vao = VertexArray::new(vbo, ebo);

        vao.bind();
        vao.upload_data(&vertices, &indices);
        Vertex::get_attributes()
            .iter()
            .for_each(|v| vao.vertex_attrib_pointer(*v));

        BoundDynamicMesh { vao }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub col: Vec3,
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
        ]
    }
}
