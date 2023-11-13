use std::collections::HashMap;

use crate::wrappers::{
    mesh::{BoundStaticMesh, Mesh},
    shader::Shader,
    textures::Texture2D,
};

use self::components::{Components, Renderer, Transform};

pub mod components;

#[derive(Debug)]
pub struct Object {
    components: Components,
}

impl Object {
    pub fn new() -> ObjectConstructor {
        ObjectConstructor {
            transform: Transform {
                enabled: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.components.transform.enabled
    }

    pub fn draw(
        &self,
        meshes: &HashMap<String, BoundStaticMesh>,
        textures: &HashMap<String, Texture2D>,
    ) {
        if !self.is_enabled() {
            return;
        }

        match &self.components.renderer {
            Some(r) => {
                r.draw(meshes, textures);
            }
            None => (),
        }
    }
}

#[derive(Debug, Default)]
pub struct ObjectConstructor {
    transform: Transform,
    mesh_name: Option<String>,
    texture_name: Option<String>,
}

impl ObjectConstructor {
    pub fn set_mesh_name<N: Into<String>>(mut self, mesh_name: N) -> Self {
        self.mesh_name = Some(mesh_name.into());
        self
    }

    pub fn set_texture_name<N: Into<String>>(mut self, texture_name: N) -> Self {
        self.texture_name = Some(texture_name.into());
        self
    }

    pub fn set_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn construct(self, _: &Shader) -> Object {
        let transform = self.transform;

        let renderer = if let Some(mesh_name) = self.mesh_name {
            let mut textures = vec![];
            if let Some(t) = self.texture_name {
                textures.push(t);
            }
            Some(Renderer::new(mesh_name, textures))
        } else {
            None
        };

        Object {
            components: Components {
                transform,
                renderer,
            },
        }
    }
}
