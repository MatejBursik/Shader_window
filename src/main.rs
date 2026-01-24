use std::ptr;
use std::time::Instant;

mod graphics;
mod select_shader;

use graphics::*;
use select_shader::SelectShader;

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
    let texture = texture::Texture::load("assets/test_image_train.jpg").expect("Failed to load texture");
    let glyph_texture = texture::Texture::load("shaders/ascii/glyph_texture_v2_(16x16x11).png").expect("Failed to load texture"); // FIX: load only for ascii

    // Load selected shaders
    let selected_shader: SelectShader = SelectShader::Ascii;
    let mut shader: shader_reader::ShaderReader;

    match selected_shader {
        SelectShader::Ascii => {
            shader = shader_reader::ShaderReader::new("shaders/ascii/vertex_shader.glsl", "shaders/ascii/fragment_shader.glsl");
            shader.bind();

            shader.create_uniform("img_texture");
            shader.set_int_uniform("img_texture", 0);

            shader.create_uniform("font_texture");
            shader.set_int_uniform("font_texture", 1);

            let (w, h) = window.get_window_size();
            shader.create_uniform("resolution");
            shader.set_vec2_f32_uniform("resolution", w, h);

            shader.create_uniform("cell_size");
            shader.set_vec2_f32_uniform("cell_size", 8.0, 8.0);

            shader.create_uniform("font_grid");
            shader.set_vec2_i32_uniform("font_grid", 11, 1);

            shader.create_uniform("glyph_count");
            shader.set_int_uniform("glyph_count", 11);
        }

        SelectShader::Pixel => {
            shader = shader_reader::ShaderReader::new("shaders/pixel/vertex_shader.glsl", "shaders/pixel/fragment_shader.glsl");
            shader.bind();

            let (w, h) = window.get_window_size();
            shader.create_uniform("resolution");
            shader.set_vec2_f32_uniform("resolution", w, h);

            shader.create_uniform("cell_size");
            shader.set_vec2_f32_uniform("cell_size", 8.0, 8.0);
        }

        SelectShader::Test => {
            shader = shader_reader::ShaderReader::new("shaders/test/vertex_shader.glsl", "shaders/test/fragment_shader.glsl");
            shader.bind();

            shader.create_uniform("time");
            shader.set_float_uniform("time", 0.0);
        }

        _ => {
            shader = shader_reader::ShaderReader::new("shaders/none/vertex_shader.glsl", "shaders/none/fragment_shader.glsl");
            shader.bind();
        }
    }
    
    let mut last_frame = Instant::now();
    let mut time: f32 = 0.0;

    while !window.close() {
        // Time difference
        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        time += dt;

        // Update shaders and textures
        match selected_shader {
            SelectShader::Ascii => {
                shader.bind();
                glyph_texture.bind(gl::TEXTURE1);
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
