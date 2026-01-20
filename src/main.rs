use std::ptr;
use std::time::Instant;

mod graphics;

use graphics::*;

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
        -1.1,  1.0,    0.0, 0.0  // top left
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
    let texture = texture::Texture::load("shaders/test/test_image.jpg").expect("Failed to load texture");

    // Load shaders
    let mut shader = shader_reader::ShaderReader::new("shaders/test/vertex_shader.glsl", "shaders/test/fragment_shader.glsl");
    shader.bind();
    shader.create_uniform("img_texture");
    shader.set_int_uniform("img_texture", 0);
    shader.create_uniform("time");
    shader.set_float_uniform("time", 0.0);

    let mut last_frame = Instant::now();
    let mut time: f32 = 0.0;

    while !window.close() {
        // Time difference
        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        time += dt;

        shader.bind();
        shader.set_float_uniform("time", time);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }
        texture.bind(gl::TEXTURE0);

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }

        window.update();
    }
}
