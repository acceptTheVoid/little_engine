use gl::types::{GLfloat, GLint, GLsizeiptr};
use glfw::{Action, Context, Key};
use std::{ffi::CString, mem, ptr};

fn sizeof<T>(_: T) -> i32 {
    mem::size_of::<T>() as _
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(1280, 720, "Я илюша обухов", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Генерируем буферы
    let vertices: [GLfloat; 9] = [-0.5, -0.5, 0., 0.5, -0.5, 0., 0.5, 0.5, 0.];
    let mut vbo: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    }

    let vertex_shader_source = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

    let fragment_shader_source = r#"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
"#;

    unsafe {
        let vertex_shader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_source = CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success: GLint = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut info = vec![0i8; 512];
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info.as_mut_ptr());
            eprintln!(
                "Compilation failed: {}",
                CString::from_vec_unchecked(mem::transmute(info))
                    .to_str()
                    .unwrap()
            );

            return;
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_source = CString::new(fragment_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info = vec![0i8; 512];
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info.as_mut_ptr());
            eprintln!(
                "Compilation failed: {}",
                CString::from_vec_unchecked(mem::transmute(info))
                    .to_str()
                    .unwrap()
            );

            return;
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // TODO: Обработка ошибок для линковки шейдеров

        gl::UseProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * sizeof(0_f32), ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.1, 0.2, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // Make sure the viewport matches the new window dimensions.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        _ => {}
    }
}
