use engine::UnsafeEngine;
use engine_math::{Vector2, Vector3, Vector4};
use wrappers::{
    mesh::{Mesh, Vertex},
    shader::ShaderSource,
    textures::Texture2D,
    types::Uniform,
};

mod engine;
mod wrappers;

fn main() {
    let vertices = vec![
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.),
            col: Vector3::new(0., 1., 0.),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.),
            col: Vector3::new(0., 0., 1.),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(0., 1.),
        },
    ];
    let indices = vec![0, 1, 3, 1, 2, 3];
    let mesh = Mesh::new(
        vertices,
        indices,
        vec![
            Texture2D::load("textures/container.jpg", "texture1"),
            Texture2D::load("textures/crack.png", "texture2"),
        ],
    );

    let shader =
        ShaderSource::from_files("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl")
            .unwrap()
            .add_mesh(mesh);

    let mut engine = UnsafeEngine::create().add_shader(shader).build();

    engine.set_background_color(Vector4::new(0., 0.1, 0.2, 1.));

    let mut time = 0.;
    engine.game_loop(|engine, _| {
        engine.clear_background();
        time += 0.0001;
        let trans = engine_math::matrices::transform_matrix::rotation_matrix_in_homogeneous_3d_Oz(time);
        engine
            .access_shader(0)
            .unwrap()
            .set_uniform("transform", Uniform::Matrix4(trans));
        engine.draw_all();
    });
}
