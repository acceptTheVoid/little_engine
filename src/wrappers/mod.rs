pub mod attribute_pointer;
pub mod buffer_object;
pub mod gl;
pub mod mesh;
pub mod shader;
pub mod textures;
pub mod types;
pub mod vertex_array;

pub fn to_ptr<T, P>(t: &T) -> *const P {
    t as *const T as _
}
