use std::{ffi::c_void, ptr};

use super::types::DataType;

pub trait Attributes {
    fn get_attributes() -> Vec<AttributePointers>;
}

#[derive(Debug, Clone, Copy)]
pub struct AttributePointers {
    pub location: u32,
    pub size: u32,
    pub data_type: DataType,
    pub normalized: bool,
    pub stride: usize,
    pub ptr: *const c_void,
}

impl AttributePointers {
    pub fn empty() -> Self {
        Self {
            location: 0,
            size: 0,
            data_type: DataType::Float,
            normalized: false,
            stride: 0,
            ptr: ptr::null(),
        }
    }
}
