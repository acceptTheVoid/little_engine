use std::{
    ffi::{c_void, CStr},
    ptr,
    sync::mpsc::Receiver,
};

use egui_glfw::EguiBackend;
use gl::{
    types::{GLchar, GLenum, GLsizei, GLuint},
    Viewport,
};
use glfw::{Context, SwapInterval, WindowEvent};

type Events = Receiver<(f64, WindowEvent)>;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    ui: EguiBackend,
    events: Events,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_scroll_polling(true);

        window.make_current();
        glfw.set_swap_interval(SwapInterval::Sync(1));

        gl::load_with(|symbol| window.get_proc_address(symbol) as _);
        Viewport::load_with(|symbol| window.get_proc_address(symbol) as _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(debug_callback), ptr::null());
        }

        let ui = EguiBackend::new(&mut window, &mut glfw);

        Self {
            glfw,
            window,
            ui,
            events,
        }
    }

    pub fn ui_handle(&self) -> &EguiBackend {
        &self.ui
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
    }

    fn process_events(&mut self) {}
}

extern "system" fn debug_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    println!(
        "OpenGL Debug Message: source={}, type={}, id={}, severity={}, message={}",
        source,
        gltype,
        id,
        severity,
        unsafe { CStr::from_ptr(message).to_str().unwrap() }
    );
}
