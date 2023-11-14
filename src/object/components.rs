use std::collections::HashMap;

use engine_math::Vector3;

use crate::wrappers::{
    mesh::{BoundStaticMesh, Draw},
    textures::Texture2D,
    types::TextureUnit,
};

#[derive(Debug, Clone)]
pub struct Components {
    pub transform: Transform,
    pub renderer: Option<Renderer>,
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: Vector3,
    pub scale: Vector3,
    pub rotation: Vector3,
    pub enabled: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vector3::default(),
            scale: Vector3::from(1.),
            enabled: true,
            rotation: Vector3::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    mesh: String,
    textures: Vec<String>,
}

impl Renderer {
    pub fn new(mesh: String, textures: Vec<String>) -> Self {
        Self { mesh, textures }
    }

    pub fn request(&self) -> (&str, Option<&str>) {
        (&self.mesh, self.textures.get(0).map(|s| s.as_str()))
    }

    pub fn draw(&self, mesh: &BoundStaticMesh, texture: Option<&Texture2D>) {
        texture.map(|t| t.bind(TextureUnit::Texture0));

        mesh.draw();
    }
}
