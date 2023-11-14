use std::fmt::Display;

use engine_math::{Matrix4, Vector2, Vector3, Vector4};
use gl::types::GLuint;

pub type Vec2 = Vector2;
pub type Vec3 = Vector3;
pub type Vec4 = Vector4;
pub type Mat4 = Matrix4;

pub type Index = GLuint;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    KeyPressed(glfw::Key),
    KeyReleased(glfw::Key),
    CursorMoved(f64, f64),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerEvent {
    IngameEvent(EventType),
    Close,
    Resize(i32, i32),
    EventsClear,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureUnit {
    Texture0 = gl::TEXTURE0 as isize,
    Texture1 = gl::TEXTURE1 as isize,
}

impl From<TextureUnit> for u32 {
    fn from(value: TextureUnit) -> Self {
        match value {
            TextureUnit::Texture0 => gl::TEXTURE0,
            TextureUnit::Texture1 => gl::TEXTURE1,
        }
    }
}

impl From<usize> for TextureUnit {
    fn from(value: usize) -> Self {
        match value {
            0 => TextureUnit::Texture0,
            1 => TextureUnit::Texture1,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Uniform {
    Vector4(Vector4),
    Matrix4(Matrix4),
    Float(f32),
    Int(i32),
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum TextureOptions {
    Repeat = gl::REPEAT as _,
    MirroredRepeat = gl::MIRRORED_REPEAT as _,
    ClampToEdge = gl::CLAMP_TO_EDGE as _,
    ClampToBorder(Vector4) = gl::CLAMP_TO_BORDER as _,
}

impl From<TextureOptions> for i32 {
    fn from(value: TextureOptions) -> Self {
        (match value {
            TextureOptions::Repeat => gl::REPEAT,
            TextureOptions::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureOptions::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureOptions::ClampToBorder(_) => gl::CLAMP_TO_BORDER,
        } as i32)
    }
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self::Repeat
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum FilterOptions {
    Nearest = gl::NEAREST as _,
    Linear = gl::LINEAR as _,
}

impl From<FilterOptions> for i32 {
    fn from(value: FilterOptions) -> Self {
        value as _
    }
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStatus {
    CompileStatus = gl::COMPILE_STATUS as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramStatus {
    LinkStatus = gl::LINK_STATUS as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
}

impl Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShaderType::VertexShader => "Vertex Shader",
                ShaderType::FragmentShader => "Fragment Shader",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferObjectType {
    ArrayBuffer = gl::ARRAY_BUFFER as isize,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER as isize,
}

impl From<BufferObjectType> for u32 {
    fn from(value: BufferObjectType) -> Self {
        value as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Float = gl::FLOAT as isize,
}

impl From<DataType> for u32 {
    fn from(value: DataType) -> Self {
        value as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawType {
    StreamDraw = gl::STREAM_DRAW as isize,
    StaticDraw = gl::STATIC_DRAW as isize,
    DynamicDraw = gl::DYNAMIC_DRAW as isize,
}

impl From<DrawType> for u32 {
    fn from(value: DrawType) -> Self {
        value as Self
    }
}
