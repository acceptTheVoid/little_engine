use std::sync::mpsc::Receiver;

use engine_math::Vec4;
use glfw::{Context, Glfw, Window, WindowEvent};

use crate::wrappers::shader::{Shader, ShaderSource};

pub struct UnsafeEngine {
    shaders: Vec<Shader>,
    window: Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    event_handler: Box<dyn FnMut(&mut Window, WindowEvent)>,
}

pub struct UnsafeEngineBuilder {
    raw_shaders: Vec<ShaderSource>,
    event_handler: Box<dyn FnMut(&mut Window, WindowEvent)>,
}

impl UnsafeEngineBuilder {
    pub fn add_shader(mut self, shader_source: ShaderSource) -> Self {
        self.raw_shaders.push(shader_source);
        self
    }

    pub fn add_event_handler<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Window, WindowEvent) + 'static,
    {
        self.event_handler = Box::new(callback);
        self
    }

    pub fn build(self) -> UnsafeEngine {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(1280, 720, "Я илюша обухов", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Self::gl_init(&mut window);

        let mut e = UnsafeEngine {
            shaders: vec![],
            window,
            reciever: events,
            glfw,
            event_handler: self.event_handler,
        };
        e.shaders = self
            .raw_shaders
            .into_iter()
            .map(|s| s.compile(&e))
            .collect();

        e
    }

    fn gl_init(window: &mut Window) {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
    }
}

impl UnsafeEngine {
    pub fn create() -> UnsafeEngineBuilder {
        UnsafeEngineBuilder {
            raw_shaders: vec![],
            event_handler: Box::new(|_, _| {}),
        }
    }

    pub fn set_background_color(&mut self, color: Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn clear_background(&mut self) {
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
        F: FnMut(&mut UnsafeEngine),
    {
        while !self.window.should_close() {
            self.handle_event();
            closure(self);
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn access_shader(&self, idx: usize) -> Option<&Shader> {
        self.shaders.get(idx)
    }

    pub fn access_shader_mut(&mut self, idx: usize) -> Option<&mut Shader> {
        self.shaders.get_mut(idx)
    }

    fn handle_event(&mut self) {
        for (_, event) in glfw::flush_messages(&self.reciever) {
            (self.event_handler)(&mut self.window, event);
        }
    }
}
