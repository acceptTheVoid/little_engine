use std::{
    collections::HashMap,
    sync::mpsc::Receiver,
    time::{Duration, SystemTime},
};

use crate::{
    object::{Object, ObjectConstructor},
    wrappers::{
        gl::GL,
        mesh::{BoundStaticMesh, Mesh},
        textures::{BuilderTexture2D, Texture2D},
        types::Vec4,
    },
};
use egui_glfw::EguiBackend;
use engine_math::{transform::homogeneous::perspective3, Matrix4};
use glfw::{Action, Context, Glfw, Key, SwapInterval, Window, WindowEvent};

use crate::wrappers::{
    shader::{Shader, ShaderSource},
    types::{EventType, InnerEvent},
};

pub type Meshes = HashMap<String, BoundStaticMesh>;
pub type Textures = HashMap<String, Texture2D>;

pub struct UnsafeEngine {
    shader: Shader,
    objects: Vec<Object>,
    commands: Vec<Command>,
    meshes: Meshes,
    textures: Textures,
    _gl: GL,
    window: Window,
    reciever: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    time_diff: Duration,
    projection: Matrix4,
    egui: EguiBackend,
}

impl UnsafeEngine {
    pub fn new(shader: ShaderSource) -> UnsafeEngine {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, reciever) = glfw
            .create_window(800, 600, "Я илюша обухов", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_scroll_polling(true);
        window.set_char_polling(true);
        window.make_current();
        // window.set_cursor_mode(CursorMode::Disabled);

        let _gl = GL::init(&mut window);

        glfw.set_swap_interval(SwapInterval::None);

        let egui = EguiBackend::new(&mut window, &mut glfw);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self {
            shader: shader.compile(&_gl),
            _gl,
            window,
            reciever,
            glfw,
            objects: vec![],
            commands: vec![],
            meshes: HashMap::new(),
            textures: HashMap::new(),
            time_diff: Duration::from_secs(0),
            projection: perspective3(10000., 0.01, 800. / 600., 45.),
            egui,
        }
    }

    pub fn command(&mut self, command: Command) {
        self.commands.push(command)
    }

    pub fn add_mesh<Name: Into<String>>(&mut self, name: Name, mesh: Mesh) {
        self.meshes
            .insert(name.into(), mesh.create_static(&self.shader));
    }

    pub fn add_texture<Name: Into<String>>(&mut self, name: Name, texture: BuilderTexture2D) {
        self.textures
            .insert(name.into(), texture.process(&self.shader));
    }

    pub fn set_background_color(&self, color: Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    fn clear_background(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn add_object(&mut self, obj: ObjectConstructor) {
        let obj = obj.construct(&self.shader);
        self.objects.push(obj)
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }

    fn change_object(&mut self, id: usize, object: ObjectConstructor) {
        self.objects[id] = object.construct(&self.shader);
    }

    pub fn delta_time(&self) -> f32 {
        self.time_diff.as_secs_f32()
    }

    pub fn draw_loop<F>(&mut self, mut closure: F)
    where
        F: FnMut(&mut UnsafeEngine, Vec<EventType>),
    {
        while !self.window.should_close() {
            let time = SystemTime::now();

            let events: Vec<_> = self
                .handle_events()
                .into_iter()
                .filter_map(|e| {
                    match e {
                        InnerEvent::IngameEvent(e) => return Some(e),
                        InnerEvent::Close => self.window.set_should_close(true),
                        InnerEvent::Resize(w, h) => unsafe {
                            self.projection = perspective3(10000.0, 0.01, w as f32 / h as f32, 45.);
                            gl::Viewport(0, 0, w, h)
                        },
                        _ => (),
                    }

                    None
                })
                .collect();

            self.egui.begin_frame(&self.window, &mut self.glfw);
            closure(self, events);

            self.clear_background();

            let commands = std::mem::take(&mut self.commands);
            commands
                .into_iter()
                .for_each(|command| command.interpret(self));

            self.update();

            egui::SidePanel::left("bebra").resizable(true).show(self.egui.get_egui_ctx(), |ui| {
                self.get_objects().iter().enumerate().for_each(|(idx, _)| {
                    ui.label(format!("{idx}"));
                })
            });

            let (width, height) = self.window.get_framebuffer_size();
            self.egui.end_frame((width as _, height as _));

            self.window.swap_buffers();
            self.glfw.poll_events();

            let delta = time.elapsed().unwrap();
            self.time_diff = delta;
        }
    }

    pub fn access_shader(&self) -> &Shader {
        &self.shader
    }

    fn update(&self) {
        self.objects
            .iter()
            .filter(|obj| obj.is_enabled())
            .for_each(|obj| {
                let transform = obj.transform();
                if let Some(renderer) = obj.renderer() {
                    let (mesh, texture) = renderer.request();
                    let mesh = self.meshes.get(mesh).unwrap();
                    let texture = texture.map(|name| self.textures.get(name)).flatten();
                    self.shader.draw(transform, mesh, texture, self.projection);
                }
            });
    }

    fn handle_events(&mut self) -> Vec<InnerEvent> {
        glfw::flush_messages(&self.reciever)
            .into_iter()
            .map(|(_, event)| {
                self.egui.handle_event(&event, &self.window);
                handle_window_event(event)
            })
            .collect()
    }
}

fn handle_window_event(event: glfw::WindowEvent) -> InnerEvent {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => InnerEvent::Close,
        glfw::WindowEvent::Key(key, _, Action::Press, _) => {
            InnerEvent::IngameEvent(EventType::KeyPressed(key))
        }
        glfw::WindowEvent::Key(key, _, Action::Release, _) => {
            InnerEvent::IngameEvent(EventType::KeyReleased(key))
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            InnerEvent::IngameEvent(EventType::CursorMoved(xpos, ypos))
        }
        glfw::WindowEvent::FramebufferSize(width, height) => InnerEvent::Resize(width, height),
        _ => InnerEvent::EventsClear,
    }
}

#[derive(Debug)]
pub enum Command {
    AddObject(ObjectConstructor),
    ChangeObject(usize, ObjectConstructor),
}

impl Command {
    fn interpret(self, engine: &mut UnsafeEngine) {
        match self {
            Self::AddObject(obj) => engine.add_object(obj),
            Self::ChangeObject(idx, obj) => engine.change_object(idx, obj),
        }
    }
}
