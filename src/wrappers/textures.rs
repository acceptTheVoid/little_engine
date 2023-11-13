use std::path::Path;

use engine_math::Vector4;
use gl::types::{GLfloat, GLuint};
use image::{DynamicImage, GenericImageView};

use super::{
    shader::Shader,
    types::{FilterOptions, TextureOptions, TextureUnit, Uniform},
};

#[derive(Debug, Clone)]
pub struct Texture2D {
    id: GLuint,
}

impl Texture2D {
    pub fn load<P>(file_name: P, name: &str) -> BuilderTexture2D
    where
        P: AsRef<Path>,
    {
        let img = image::open(file_name).unwrap();
        BuilderTexture2D {
            img,
            name: name.into(),
            parameters: TextureParameters::default(),
        }
    }

    pub fn bind(&self, texture_unit: TextureUnit) {
        unsafe {
            gl::ActiveTexture(texture_unit.into());
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextureParameters {
    pub wrap_s: TextureOptions,
    pub wrap_t: TextureOptions,
    pub mag_fiter: FilterOptions,
    pub min_filter: FilterOptions,
}

impl TextureParameters {
    pub fn clamped_to_edge() -> Self {
        Self {
            wrap_s: TextureOptions::ClampToEdge,
            wrap_t: TextureOptions::ClampToEdge,
            ..Default::default()
        }
    }

    pub fn clamped_to_border(col: Vector4) -> Self {
        Self {
            wrap_s: TextureOptions::ClampToBorder(col),
            wrap_t: TextureOptions::ClampToBorder(col),
            ..Default::default()
        }
    }

    pub fn nearest() -> Self {
        Self {
            mag_fiter: FilterOptions::Nearest,
            min_filter: FilterOptions::Nearest,
            ..Default::default()
        }
    }

    pub fn linear() -> Self {
        Self {
            mag_fiter: FilterOptions::Linear,
            min_filter: FilterOptions::Linear,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuilderTexture2D {
    img: DynamicImage,
    parameters: TextureParameters,
    name: String,
}

impl BuilderTexture2D {
    pub fn flipv(mut self) -> Self {
        self.img = self.img.flipv();
        self
    }

    pub fn fliph(mut self) -> Self {
        self.img = self.img.fliph();
        self
    }

    pub fn set_parameters(mut self, parameters: TextureParameters) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn process(self, shader: &Shader) -> Texture2D {
        let Self {
            img,
            name,
            parameters,
        } = self;
        unsafe {
            let (width, height) = img.dimensions();
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, parameters.wrap_s.into());
            if let TextureOptions::ClampToBorder(col) = parameters.wrap_s {
                gl::TexParameterfv(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_BORDER_COLOR,
                    &col as *const Vector4 as *const GLfloat,
                );
            }

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, parameters.wrap_t.into());
            if let TextureOptions::ClampToBorder(col) = parameters.wrap_t {
                gl::TexParameterfv(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_BORDER_COLOR,
                    &col as *const Vector4 as *const GLfloat,
                );
            }

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                parameters.min_filter.into(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                parameters.mag_fiter.into(),
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                width as _,
                height as _,
                0,
                gl::RGB as _,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr().cast(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            shader.set_uniform(&name, Uniform::Int(0));

            Texture2D { id }
        }
    }
}
