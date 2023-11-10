use std::sync::mpsc::Receiver;

use crate::wrappers::types::Vec4;
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

use crate::wrappers::{
    shader::{Shader, ShaderSource},
    types::{EventType, InnerEvent},
};

pub struct UnsafeEngine {
    shaders: Vec<Shader>,
    window: Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
}

pub struct UnsafeEngineBuilder<'a> {
    raw_shaders: Vec<ShaderSource<'a>>,
}

impl<'a> UnsafeEngineBuilder<'a> {
    pub fn add_shader(mut self, shader_source: ShaderSource<'a>) -> Self {
        self.raw_shaders.push(shader_source);
        self
    }

    pub fn build(self) -> UnsafeEngine {
        let UnsafeEngineBuilder { raw_shaders } = self;

        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, reciever) = glfw
            .create_window(1280, 720, "Я илюша обухов", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Self::gl_init(&mut window);

        let mut engine = UnsafeEngine {
            shaders: vec![],
            window,
            reciever,
            glfw,
        };

        engine.shaders = raw_shaders
            .into_iter()
            .map(|s| s.compile(&engine))
            .collect();

        engine
    }

    fn gl_init(window: &mut Window) {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
    }
}

impl<'a> UnsafeEngine {
    pub fn create() -> UnsafeEngineBuilder<'a> {
        UnsafeEngineBuilder {
            raw_shaders: vec![],
        }
    }

    pub fn set_background_color(&self, color: Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn clear_background(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_all(&self) {
        for s in &self.shaders {
            s.draw_associated(self);
        }
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

    pub fn access_shader(&self, idx: usize) -> Option<&Shader> {
        self.shaders.get(idx)
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
