use std::f32::consts::PI;

use engine::UnsafeEngine;
use engine_math::{Vector2, Vector3, Vector4, Matrix4, transform::homogeneous::{rotate3x, translate3z, perspective3, translate3, rotate3y, rotate3z}};
use object::{Object, components::Transform};
use wrappers::{
    mesh::{Mesh, Vertex},
    shader::ShaderSource,
    textures::Texture2D, types::Uniform,
};

mod engine;
mod object;
mod wrappers;

fn main() {
    let vertices = vec![
        Vertex {
            pos: Vector3::new(0.5, -0.5, -0.5),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.5),
            col: Vector3::new(0., 1., 0.),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.5),
            col: Vector3::new(0., 0., 1.),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.5),
            col: Vector3::new(0., 0., 1.),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            col: Vector3::new(0., 1., 0.),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, -0.5),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, -0.5),
            col: Vector3::new(1., 0., 0.),
            tex: Vector2::new(0., 1.),
        },
    ];

    let indices = vec![
        0, 3, 2,
        2, 0, 1,
        1, 2, 5,
        2, 4, 5, 
        0, 1, 6,
        1, 5, 6,
        3, 0, 7,
        0, 6, 7,
        2, 3, 4,
        3, 7, 4,
        5, 4, 6,
        4, 7, 6
    ];
    let mesh = Mesh::new(
        vertices,
        indices,
        // vec![
        //     Texture2D::load("textures/container.jpg", "texture1"),
        //     Texture2D::load("textures/crack.png", "texture2"),
        // ],
    );

    let positions = [
        Vector3::new(0., 0., 0.),
        Vector3::new(2.,5., -15.),
        Vector3::new(-1.5, -2.2, -2.5),
        Vector3::new(-3.8, -2.0, -12.3),
        Vector3::new(2.4, -0.4, -3.5),
        Vector3::new(-1.7, 3.0, -7.5),
        Vector3::new(1.3, -2.0, -2.5),
        Vector3::new(1.5, 2.0, -2.5),
        Vector3::new(1.5, 0.2, -1.5),
        Vector3::new(-1.3, 1., -1.5),
    ];

    let texture = Texture2D::load("textures/container.jpg", "texture1");

    let shader =
        ShaderSource::from_files("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl")
            .unwrap();

    let mut engine = UnsafeEngine::new(shader);
    engine.add_mesh("rectangle", mesh);
    engine.add_texture("texture1", texture);
    for pos in positions {
        engine.add_object(Object::new().set_mesh_name("rectangle").set_texture_name("texture1").set_transform(Transform { enabled: true, pos, ..Default::default()}));
    }
    // engine.add_object(
    //     Object::new()
    //         .set_mesh_name("rectangle")
    //         .set_texture_name("texture1"),
    // );

    engine.set_background_color(Vector4::new(0., 0.1, 0.2, 1.));

    let model = rotate3x(PI /  3.);
    let view = translate3z(-3.);
    let projection = perspective3(100., 0.1, 800. / 600., PI / 4.);

    engine.game_loop(|engine, _| {
        engine.clear_background();
        let shader = engine.access_shader();
        shader.set_uniform("model", Uniform::Matrix4(model));
        shader.set_uniform("view", Uniform::Matrix4(view));
        shader.set_uniform("projection", Uniform::Matrix4(projection));
        // time += 0.0001;
        // let trans = engine_math::matrices::transform_matrix::rotation_matrix_in_homogeneous_3d_Oz(time);
        // engine
        //     .access_shader()
        //     .set_uniform("transform", Uniform::Matrix4(trans));
        engine.draw_all();
    });
}
