use std::f32::consts::PI;

use engine::UnsafeEngine;
use engine_math::{
    transform::homogeneous::{perspective3, rotate3x, rotate3y, rotate3z, translate3, translate3z},
    Matrix4, Vector, Vector2, Vector3, Vector4,
};
use object::Object;
use wrappers::{
    mesh::{Mesh, Vertex},
    shader::ShaderSource,
    textures::Texture2D,
    types::Uniform,
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
        0, 3, 2, 2, 0, 1, 1, 2, 5, 2, 4, 5, 0, 1, 6, 1, 5, 6, 3, 0, 7, 0, 6, 7, 2, 3, 4, 3, 7, 4,
        5, 4, 6, 4, 7, 6,
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
        Vector3::new(2., 5., -15.),
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
    engine.add_object(
        Object::new()
            .set_mesh_name("rectangle")
            .set_texture_name("texture1"),
    );

    engine.set_background_color(Vector4::new(0., 0.1, 0.2, 1.));

    let projection = perspective3(100., 0.1, 800. / 600., PI / 4.);

    let mut time: f32 = 0.;
    let r = 10.;
    engine.game_loop(|engine, _| {
        time += 0.001;

        let cam_x = time.sin() * r;
        let cam_z = time.cos() * r;
        let cam_y = time;
        let view = look_at(
            Vector3::new(cam_x, cam_y, cam_z),
            Vector3::new(0., -1., 0.),
            Vector3::new(0., 1., 0.),
        );

        engine.clear_background();
        let shader = engine.access_shader();
        shader.set_uniform("view", Uniform::Matrix4(view));
        shader.set_uniform("projection", Uniform::Matrix4(projection));

        for (i, t) in positions.iter().enumerate() {
            let model = translate3(*t);
            let model = model * rotate3x(radians((20 * i) as _));
            let model = model * rotate3y(radians((20 * i) as f32));
            let model = model * rotate3z(radians((20 * i) as f32));
            shader.set_uniform("model", Uniform::Matrix4(model));
            engine.draw_all();
        }
    });
}

fn look_at(camera_position: Vector3, target_pos: Vector3, up_vector: Vector3) -> Matrix4 {
    let camera_dir = (camera_position - target_pos).normalize();
    let right = up_vector.cross(camera_dir).normalize();
    let up = camera_dir.cross(right).normalize();
    let res = Matrix4::new([
        [right.x, right.y, right.z, 0.],
        [up.x, up.y, up.z, 0.],
        [
            camera_dir.x,
            camera_dir.y,
            camera_dir.z,
            0.,
        ],
        [0., 0., 0., 1.],
    ]);

    res * translate3(-camera_position)
}

fn radians(angle: f32) -> f32 {
    PI * angle / 180.
}
