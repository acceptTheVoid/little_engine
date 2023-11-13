use std::{collections::HashMap, sync::mpsc::Receiver};

use crate::{
    object::{Object, ObjectConstructor},
    wrappers::{
        gl::GL,
        mesh::{BoundStaticMesh, Mesh},
        textures::{BuilderTexture2D, Texture2D},
        types::Vec4,
    },
};
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

use crate::wrappers::{
    shader::{Shader, ShaderSource},
    types::{EventType, InnerEvent},
};

pub type Meshes = HashMap<String, BoundStaticMesh>;
pub type Textures = HashMap<String, Texture2D>;

pub struct UnsafeEngine {
    shader: Shader,
    objects: Vec<Object>,
    meshes: Meshes,
    textures: Textures,
    gl: GL,
    window: Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
}

// pub struct UnsafeEngineBuilder {
//     raw_shaders: ShaderSource,
// }

// impl UnsafeEngineBuilder {
//     pub fn build(self) -> UnsafeEngine {
//         let UnsafeEngineBuilder { raw_shaders } = self;

//         let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

//         let (mut window, reciever) = glfw
//             .create_window(1280, 720, "Я илюша обухов", glfw::WindowMode::Windowed)
//             .expect("Failed to create GLFW window.");

//         window.set_key_polling(true);
//         window.make_current();

//         Self::gl_init(&mut window);

//         let mut engine = UnsafeEngine {
//             shader: ,
//             window,
//             reciever,
//             glfw,
//         };

//         engine
//     }

//     fn gl_init(window: &mut Window) {
//         gl::load_with(|s| window.get_proc_address(s) as *const _);
//     }
// }

impl UnsafeEngine {
    pub fn new(shader: ShaderSource) -> UnsafeEngine {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, reciever) = glfw
            .create_window(800, 600, "Я илюша обухов", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        let gl = GL::init(&mut window);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self {
            shader: shader.compile(&gl),
            gl,
            window,
            reciever,
            glfw,
            objects: vec![],
            meshes: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn add_mesh<N: Into<String>>(&mut self, name: N, mesh: Mesh) {
        self.meshes
            .insert(name.into(), mesh.create_static(&self.shader));
    }

    pub fn add_texture<N: Into<String>>(&mut self, name: N, texture: BuilderTexture2D) {
        self.textures
            .insert(name.into(), texture.process(&self.shader));
    }

    pub fn set_background_color(&self, color: Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn clear_background(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn add_object(&mut self, obj: ObjectConstructor) {
        let obj = obj.construct(&self.shader);
        self.objects.push(obj)
    }

    pub fn draw_all(&self) {
        self.shader
            .draw_associated(&self.objects, &self.meshes, &self.textures);
    }

    pub fn game_loop<F>(&mut self, mut closure: F)
    where
        F: FnMut(&UnsafeEngine, EventType),
    {
        while !self.window.should_close() {
            for e in self.handle_events() {
                match e {
                    InnerEvent::IngameEvent(e) => closure(self, e),
                    InnerEvent::Close => self.window.set_should_close(true),
                    InnerEvent::Resize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                    _ => (),
                }
            }

            closure(self, EventType::None);
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn access_shader(&self) -> &Shader {
        &self.shader
    }

    fn handle_events(&mut self) -> Vec<InnerEvent> {
        glfw::flush_messages(&self.reciever)
            .into_iter()
            .map(|(_, event)| handle_window_event(event))
            .collect()
    }
}

fn handle_window_event(event: glfw::WindowEvent) -> InnerEvent {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => InnerEvent::Close,
        glfw::WindowEvent::Key(key, _, Action::Press, _) => {
            InnerEvent::IngameEvent(EventType::KeyPress(key))
        }
        glfw::WindowEvent::FramebufferSize(width, height) => InnerEvent::Resize(width, height),
        _ => InnerEvent::EventsClear,
    }
}
