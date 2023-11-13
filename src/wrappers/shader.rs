use std::{ffi::CString, fmt::Display, fs, io, mem, path::Path, ptr};

use engine_math::Matrix4;
use gl::types::GLint;

use crate::{
    engine::{Meshes, Textures},
    object::Object,
};

use super::{
    gl::GL,
    types::{ProgramStatus, ShaderStatus, ShaderType, Uniform},
};

#[derive(Debug, Clone)]
pub struct ShaderSource {
    vertex_shader: String,
    fragment_shader: String,
}

impl ShaderSource {
    #[allow(unused)]
    pub fn from_strings(vertex_shader: String, fragment_shader: String) -> Self {
        Self {
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn from_files<P>(vertex_shader_path: P, fragment_shader_path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let vertex_shader = fs::read_to_string(vertex_shader_path)?;
        let fragment_shader = fs::read_to_string(fragment_shader_path)?;

        Ok(Self {
            vertex_shader,
            fragment_shader,
        })
    }

    pub fn compile(self, _: &GL) -> Shader {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_shader_source = CString::new(self.vertex_shader.as_bytes()).unwrap();
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_source.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            Shader::get_shader_status(
                vertex_shader,
                ShaderStatus::CompileStatus,
                ShaderType::VertexShader,
            )
            .unwrap();

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_shader_source = CString::new(self.fragment_shader.as_bytes()).unwrap();
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_source.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            Shader::get_shader_status(
                fragment_shader,
                ShaderStatus::CompileStatus,
                ShaderType::FragmentShader,
            )
            .unwrap();

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            Shader::get_program_status(shader_program, ProgramStatus::LinkStatus).unwrap();

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader {
                shader: shader_program,
            }
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    shader: u32,
}

impl Shader {
    pub fn draw_associated(&self, objects: &[Object], meshes: &Meshes, textures: &Textures) {
        self.use_program();
        objects.iter().for_each(|obj| obj.draw(meshes, textures));
    }

    fn use_program(&self) {
        unsafe { gl::UseProgram(self.shader) };
    }

    pub fn set_uniform(&self, name: &str, uniform: Uniform) {
        use engine_math::Matrix;

        unsafe {
            let name = CString::new(name).unwrap();
            self.use_program();
            let location = gl::GetUniformLocation(self.shader, name.as_ptr());
            match uniform {
                Uniform::Vector4(v) => gl::Uniform4f(location, v.x, v.y, v.z, v.w),
                Uniform::Matrix4(m) => {
                    // let m = dbg!(m.transpose());
                    let m = m.transpose();
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, &m as *const Matrix4 as *const f32)
                }
                Uniform::Float(f) => gl::Uniform1f(location, f),
                Uniform::Int(i) => gl::Uniform1i(location, i),
            };
        }
    }

    unsafe fn get_shader_status(
        shader: u32,
        status: ShaderStatus,
        shader_type: ShaderType,
    ) -> Result<(), Error> {
        let mut success: GLint = 0;
        gl::GetShaderiv(shader, status as _, &mut success);

        if success == 0 {
            let mut info = vec![0i8; 512];
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info.as_mut_ptr());
            return Err(Error::CompilationError(format!(
                "{shader_type} compilation failed: {}",
                CString::from_vec_unchecked(mem::transmute(info))
                    .to_str()
                    .unwrap()
            )));
        }

        Ok(())
    }

    unsafe fn get_program_status(shader: u32, status: ProgramStatus) -> Result<(), Error> {
        let mut success: GLint = 0;
        gl::GetProgramiv(shader, status as _, &mut success);

        if success == 0 {
            let mut info = vec![0i8; 512];
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info.as_mut_ptr());
            return Err(Error::LinkingError(format!(
                "Linking failed: {}",
                CString::from_vec_unchecked(mem::transmute(info))
                    .to_str()
                    .unwrap()
            )));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CompilationError(String),
    LinkingError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::CompilationError(s) => s,
                Error::LinkingError(s) => s,
            }
        )
    }
}

impl std::error::Error for Error {}
