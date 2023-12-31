use gl::types::*;

use super::{attribute_pointer::AttributePointers, buffer_object::*};

#[derive(Debug, Clone)]
pub struct VertexArray {
    id: GLuint,
    vertex_bo: BufferObject,
    indices_bo: BufferObject,
    associate_vertices: GLint,
}

impl VertexArray {
    pub fn new(vertex_bo: BufferObject, indices_bo: BufferObject) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {
            id,
            vertex_bo,
            indices_bo,
            associate_vertices: 0,
        }
    }

    pub fn upload_data<T, U>(&mut self, vertex_data: &[T], indices_data: &[U]) {
        self.associate_vertices = indices_data.len() as _;
        self.vertex_bo.bind();
        self.vertex_bo.upload_data(vertex_data);
        self.indices_bo.bind();
        self.indices_bo.upload_data(indices_data);
    }

    pub fn vertex_attrib_pointer(&self, vap: AttributePointers) {
        let normalized = if vap.normalized { gl::TRUE } else { gl::FALSE };
        unsafe {
            gl::VertexAttribPointer(
                vap.location,
                vap.size as i32,
                vap.data_type.into(),
                normalized,
                vap.stride as i32,
                vap.ptr,
            );
            gl::EnableVertexAttribArray(vap.location);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.associate_vertices,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}
