use std::collections::HashMap;

use engine_math::Vector3;

use crate::wrappers::{
    mesh::{BoundStaticMesh, Draw},
    textures::Texture2D,
    types::TextureUnit,
};

#[derive(Debug)]
pub struct Components {
    pub transform: Transform,
    pub renderer: Option<Renderer>,
}

#[derive(Debug, Default)]
pub struct Transform {
    pub pos: Vector3,
    pub scale: Vector3,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct Renderer {
    mesh: String,
    textures: Vec<String>,
}

impl Renderer {
    pub fn new(mesh: String, textures: Vec<String>) -> Self {
        Self { mesh, textures }
    }

    pub fn draw(
        &self,
        meshes: &HashMap<String, BoundStaticMesh>,
        textures: &HashMap<String, Texture2D>,
    ) {
        let mesh = meshes.get(&self.mesh).unwrap();
        if !self.textures.is_empty() {
            textures
                .get(&self.textures[0])
                .unwrap()
                .bind(TextureUnit::Texture0);
        }

        mesh.draw();
    }
}
