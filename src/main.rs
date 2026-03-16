use std::ptr;
use std::time::Instant;
use std::sync::mpsc;
use glfw::Key;
use glfw::ffi::glfwShowWindow;
use tokio::sync::broadcast;
use mozjpeg::{ Compress, ColorSpace };

mod graphics;
mod capture;
mod select_shader;
mod select_mode;
mod web_socket;
mod help_functions;

use graphics::*;
use capture::*;
use select_shader::SelectShader;
use select_mode::SelectMode;
use web_socket::start_server;
use help_functions::*;

fn main() {
    let mut window = window::Window::new(1280, 720, "Window");
    window.init_gl();
    window.set_fps(1);

    unsafe {
        glfwShowWindow(window.get_window_ptr());
    }

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

    // Setup web socket for window sharing
    let mut share_window = false;
    let (tx, _rx) = broadcast::channel::<Vec<u8>>(4); // FIX: connection breaks when app window is minimised
    start_server(tx.clone());
    let mut frame: Vec<u8>;

    // Setup encoder thread
    let (frame_tx, frame_rx) = mpsc::sync_channel::<(Vec<u8>, i32, i32)>(1); // bound of 1 = drop if busy
    let tx_clone = tx.clone();

    std::thread::spawn(move || {
        while let Ok((frame, w, h)) = frame_rx.recv() {
            let start = Instant::now();

            // Reduce resolution before encoding
            let (frame, w, h) = rgb_downscale_by_factor(&frame, w as usize, h as usize, 2);

            // Encode RGB frame to jpeg
            let mut comp = Compress::new(ColorSpace::JCS_RGB);
            comp.set_fastest_defaults();
            comp.set_size(w as usize, h as usize);
            comp.set_quality(100.0); // 60 is recommended but with downscale, text becomes completely unreadable and helped with maybe 2-5ms
            //comp.set_chroma_sampling_pixel_sizes((2,2), (2,2)); // 4:2:0 chroma subsampling helped with maybe 2-5ms but worse quality
            let mut comp = comp.start_compress(Vec::new()).unwrap();
            comp.write_scanlines(&frame).unwrap();
            let compressed = comp.finish().unwrap();

            let _ = tx_clone.send(compressed);
            println!("encode: {:?}", start.elapsed());
        }
    });
    
    // Setup screen capture
    let mut select_mode = SelectMode::Image;
    capture_settings(window.get_window_ptr());
    let mut capture = ScreenCapture::new();

    // Load image as a texture
    let mut texture_idx: usize = 0;
    let mut texture = load_test_image(&mut texture_idx);
    window.set_window_size(texture.get_texture_size());

    let glyph_texture = texture::Texture::load_file("shaders/ascii/glyph_texture_v2-edge_(16x16x15).png").expect("Failed to load texture"); // FIX: load only for ascii

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
                if window.is_key_released(Key::C) {
                    // Toggle capture and image modes
                    select_mode = select_mode.next();

                    if select_mode == SelectMode::Image {
                        texture = load_test_image(&mut texture_idx);
                        window.set_window_size(texture.get_texture_size());
                    }
                }

                if window.is_key_released(Key::O) {
                    // Toggle overlay mode on/off
                    window.toggle_overlay_mode();
                }

                if window.is_key_released(Key::S) {
                    // Toggle share window on/off
                    share_window = !share_window;
                    println!("Toggle share server: {}", share_window)
                }
                
                if window.is_key_pressed(Key::N) {
                    if window.is_key_released(Key::I) {
                        // Next test image
                        if select_mode == SelectMode::Image {
                            texture_idx += 1;
                            texture = load_test_image(&mut texture_idx);
                            window.set_window_size(texture.get_texture_size());
                        }
                    }

                    if window.is_key_released(Key::E) {
                        // Next shader/effect
                        selected_shader = selected_shader.next();
                        shader = load_shader(&selected_shader, window.get_window_size());
                    }
                }
            }
        }

        if select_mode == SelectMode::ScreenCapture {
            let (w, h) = window.get_window_size();
            let (x, y) = window.get_window_pos();
            
            texture = capture.get_frame((w as usize, h as usize), (x as usize, y as usize)).clone();
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

        // Draw to window
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }

        if share_window && tx.receiver_count() > 0 {
            // Load pixels from the window into a variable
            let (w, h) = window.get_window_size();
            frame = window.read_pixels(); // Vec<u8> RGB

            // Try_send - if the encoder thread is still busy, drop the frame
            let _ = frame_tx.try_send((frame, w, h));
        }
        
        window.update();
    }
}
