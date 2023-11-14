use engine_math::{Vector2, Vector3, Vector4};
use glfw::Key;
use unsafe_engine::engine::UnsafeEngine;
use unsafe_engine::object::{components::Transform, Object, ObjectConstructor};
use unsafe_engine::wrappers::{
    mesh::{Mesh, Vertex},
    shader::ShaderSource,
    textures::Texture2D,
    types::EventType,
};
use unsafe_engine::Command;

fn main() {
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
    engine.add_mesh("cube", cube());
    engine.add_texture("texture1", texture);

    engine.set_background_color(Vector4::new(0., 0.1, 0.2, 1.));

    let mut i = 0;
    engine.game_loop(|engine, events| {
        engine.clear_background();

        for e in events {
            match e {
                EventType::KeyPressed(Key::Space) => {
                    engine.command(Command::AddObject(
                        Object::new()
                            .set_mesh_name("cube")
                            .set_texture_name("texture1")
                            .set_transform(Transform {
                                pos: positions[i % positions.len()],
                                ..Default::default()
                            }),
                    ));
                    i += 1;
                }
                _ => (),
            }
        }

        let len = engine.get_objects().len();
        for i in 0..len {
            let time = engine.delta_time();
            let obj = engine.get_objects()[i].clone();
            let mut pos = obj.transform().pos;
            let mut rotation = obj.transform().rotation;
            rotation.x += time / 4.;
            rotation.y += time;
            rotation.z += time / 8.;
            pos.y += time / 2.;
            let transform = Transform {
                pos,
                rotation,
                ..obj.transform().clone()
            };

            engine.command(Command::ChangeObject(
                i,
                ObjectConstructor::from(obj.clone()).set_transform(transform),
            ));
        }
    });
}

fn cube() -> Mesh {
    let vertices = vec![
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, -0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 0.),
        },
        // ------
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        // ------
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        // ------
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        // ------
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, -0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, -0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        // ------
        Vertex {
            pos: Vector3::new(-0.5, 0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, -0.5),
            tex: Vector2::new(1., 1.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(0.5, 0.5, 0.5),
            tex: Vector2::new(1., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, 0.5),
            tex: Vector2::new(0., 0.),
        },
        Vertex {
            pos: Vector3::new(-0.5, 0.5, -0.5),
            tex: Vector2::new(0., 1.),
        },
    ];

    let indices = (0..36).collect();
    Mesh::new(vertices, indices)
}
