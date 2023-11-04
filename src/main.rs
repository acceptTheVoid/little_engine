use engine::UnsafeEngine;
use engine_math::{Vec3, Vec4};
use glfw::{ffi::glfwGetTime, Action, Key};
use wrappers::{
    mesh::{Mesh, Vertex},
    shader::ShaderSource, types::Uniform,
};

mod engine;
mod wrappers;

fn main() {
    let vertices = vec![
        Vertex {
            pos: Vec3::new(-0.5, -0.5, 0.),
            col: Vec3::new(1., 0., 0.),
        },
        Vertex {
            pos: Vec3::new(0.5, -0.5, 0.),
            col: Vec3::new(0., 1., 0.),
        },
        Vertex {
            pos: Vec3::new(0., 0.5, 0.),
            col: Vec3::new(0., 0., 1.),
        },
    ];
    let indices = vec![0, 1, 2];
    let mesh = Mesh::new(vertices, indices);

    // let vertices = vec![
    //     Vertex {
    //         pos: Vec3::new(-0.75, -0.75, 0.),
    //     },
    //     Vertex {
    //         pos: Vec3::new(-0.8, -0.3, 0.),
    //     },
    //     Vertex {
    //         pos: Vec3::new(-0.6, -0.5, 0.),
    //     },
    // ];
    // let indices = vec![0, 1, 2];
    // let mesh_boga = Mesh::new(vertices, indices);

    let shader =
        ShaderSource::from_files("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl")
            .unwrap()
            .add_mesh(mesh);

    // let shader_2 = ShaderSource::from_files(
    //     "shaders/vertex_shader.glsl",
    //     "shaders/fragment_shader_2.glsl",
    // )
    // .unwrap()
    // .add_mesh(mesh_boga);

    let mut engine = UnsafeEngine::create()
        .add_shader(shader)
        // .add_shader(shader_2)
        .add_event_handler(handle_window_event)
        .build();

    engine.set_background_color(Vec4::new(0., 0.1, 0.2, 1.));

    engine.game_loop(|e| {
        e.clear_background();
        e.draw_all();
    });
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        // glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => (),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // Make sure the viewport matches the new window dimensions.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        _ => {}
    }
}
