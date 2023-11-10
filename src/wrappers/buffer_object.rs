use gl::types::*;

use super::types::{BufferObjectType, DrawType};

#[derive(Debug, Clone)]
pub struct BufferObject {
    pub id: GLuint,
    pub bo_type: BufferObjectType,
    pub draw_type: DrawType,
}

impl BufferObject {
    pub fn new(bo_type: BufferObjectType, draw_type: DrawType) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self {
            id,
            bo_type,
            draw_type,
        }
    }

    pub fn vertex_buffer_object() -> Self {
        Self::new(BufferObjectType::ArrayBuffer, DrawType::StaticDraw)
    }

    pub fn element_buffer_object() -> Self {
        Self::new(BufferObjectType::ElementArrayBuffer, DrawType::StaticDraw)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.bo_type.into(), self.id);
        }
    }

    pub fn upload_data<T>(&self, data: &[T]) {
        let size = (std::mem::size_of::<T>() * data.len()) as _;
        unsafe {
            gl::BufferData(
                self.bo_type.into(),
                size,
                data.as_ptr().cast(),
                self.draw_type.into(),
            );
        }
    }
}
