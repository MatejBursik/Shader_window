use std::ptr;
use std::time::Instant;
use glfw::Key;

mod graphics;
mod select_shader;
mod help_functions;

use graphics::*;
use select_shader::SelectShader;
use help_functions::*;

fn main() {
    let mut window = window::Window::new(1280, 720, "Window");
    window.init_gl();
    window.set_fps(1);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ibo = 0;

    let vertices: Vec<f32> = vec![
        // positions   // texture coords
        1.0,  1.0,     1.0, 0.0,  // top right
        1.0, -1.0,     1.0, 1.0,  // bottom right
        -1.0, -1.0,    0.0, 1.0,  // bottom left
        -1.0,  1.0,    0.0, 0.0   // top left
    ];

    let indices: Vec<u32> = vec![
        0, 1, 3,  // first triangle
        1, 2, 3   // second triangle
    ];
    
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ibo);
        
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);
        
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(0);
        
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Load image as a texture
    let mut texture_idx: usize = 0;
    let mut texture = load_test_image(&mut texture_idx);
    window.set_window_size(texture.get_texture_size());

    let glyph_texture = texture::Texture::load("shaders/ascii/glyph_texture_v2-edge_(16x16x15).png").expect("Failed to load texture"); // FIX: load only for ascii

    // Load selected shaders
    let mut selected_shader: SelectShader = SelectShader::None;
    let mut shader: shader_reader::ShaderReader = load_shader(&selected_shader, window.get_window_size());
    
    // Time difference setup
    let mut last_frame = Instant::now();
    let mut time: f32 = 0.0;

    while !window.close() {
        // Time difference
        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        time += dt;

        // User inputs
        if window.is_key_pressed(Key::LeftControl) {
            if window.is_key_pressed(Key::LeftAlt) {
                if window.is_key_released(Key::O) {
                    // Toggle overlay mode
                    window.toggle_overlay_mode();
                }
                
                if window.is_key_pressed(Key::N) {
                    if window.is_key_released(Key::I) {
                        // Next test image
                        texture_idx += 1;
                        texture = load_test_image(&mut texture_idx);
                        window.set_window_size(texture.get_texture_size());
                    }

                    if window.is_key_released(Key::S) {
                        // Next shader
                        selected_shader = selected_shader.next();
                        shader = load_shader(&selected_shader, window.get_window_size());
                    }
                }
            }
        }

        // Update shaders and textures
        match selected_shader {
            SelectShader::EdgeDetect => {
                shader.bind();
                
                let (w, h) = window.get_window_size();
                shader.create_uniform("resolution");
                shader.set_vec2_f32_uniform("resolution", w as f32, h as f32);
            }

            SelectShader::Ascii => {
                shader.bind();

                let (w, h) = window.get_window_size();
                shader.create_uniform("resolution");
                shader.set_vec2_f32_uniform("resolution", w as f32, h as f32);

                glyph_texture.bind(gl::TEXTURE1);
            }

            SelectShader::Pixel => {
                shader.bind();

                let (w, h) = window.get_window_size();
                shader.create_uniform("resolution");
                shader.set_vec2_f32_uniform("resolution", w as f32, h as f32);
            }

            SelectShader::Test => {
                shader.bind();
                shader.set_float_uniform("time", time);
            }

            _ => {
                shader.bind();
            }
        }

        texture.bind(gl::TEXTURE0);

        // Draw
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }

        window.update();
    }
}
