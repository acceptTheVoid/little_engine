use std::fmt::Display;

use engine_math::Vec4;
use gl::types::GLuint;

pub type Index = GLuint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Uniform {
    Vec4(Vec4),
    Float(f32),
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
