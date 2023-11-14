use crate::wrappers::shader::Shader;

use self::components::{Components, Renderer, Transform};

pub mod components;

#[derive(Debug, Clone)]
pub struct Object {
    transform: Transform,
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

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn renderer(&self) -> Option<&Renderer> {
        self.components.renderer.as_ref()
    }

    pub fn is_enabled(&self) -> bool {
        self.transform.enabled
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
            transform,
            components: Components { renderer },
        }
    }
}

impl From<Object> for ObjectConstructor {
    fn from(value: Object) -> Self {
        let req = value.renderer().map(|r| r.request());
        if let Some((mesh_name, texture_name)) = req {
            let mesh_name = Some(mesh_name.to_string());
            let texture_name = texture_name.map(|s| s.to_string());
            ObjectConstructor {
                transform: value.transform().clone(),
                mesh_name,
                texture_name,
            }
        } else {
            ObjectConstructor {
                transform: value.transform().clone(),
                mesh_name: None,
                texture_name: None,
            }
        }
    }
}
