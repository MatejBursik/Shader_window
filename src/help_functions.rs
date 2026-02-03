use crate::graphics::*;
use crate::select_shader::SelectShader;

pub fn load_test_image(index: &mut usize) -> texture::Texture{
    let test_images = vec![
        "assets/test_image_acerola_example.png",
        "assets/test_image_astroneer_gameplay.png",
        "assets/test_image_chrome_screenshot.png",
        "assets/test_image_destiny_2_poster.png",
        "assets/test_image_destiny_2_rasputin.png",
        "assets/test_image_destiny_2_witness.png",
        "assets/test_image_dont_starve_gameplay.png",
        "assets/test_image_dont_starve_poster.png",
        "assets/test_image_doom_demon.png",
        "assets/test_image_doom_slayer.png",
        "assets/test_image_elden_ring_gameplay.png",
        "assets/test_image_elden_ring_poster.png",
        "assets/test_image_hollow_knight_gameplay.png",
        "assets/test_image_hollow_knight_poster.png",
        "assets/test_image_titanfall_2_poster_1.png",
        "assets/test_image_titanfall_2_poster_2.png",
        "assets/test_image_train.jpg"
    ];

    if *index >= test_images.len() {
        *index = 0;
    }

    texture::Texture::load(test_images[*index]).expect("Failed to load texture")
}

pub fn load_shader(selected_shader: &SelectShader, size: (i32, i32)) -> shader_reader::ShaderReader {
    let mut shader: shader_reader::ShaderReader;

    match selected_shader {
        SelectShader::EdgeDetect => {
            shader = shader_reader::ShaderReader::new("shaders/edge_detect/vertex_shader.glsl", "shaders/edge_detect/fragment_shader.glsl");
            shader.bind();

            shader.create_uniform("resolution");
            shader.set_vec2_f32_uniform("resolution", size.0 as f32, size.1 as f32);
        }

        SelectShader::Ascii => {
            shader = shader_reader::ShaderReader::new("shaders/ascii/vertex_shader.glsl", "shaders/ascii/fragment_shader_with_edge.glsl");
            shader.bind();

            shader.create_uniform("img_texture");
            shader.set_int_uniform("img_texture", 0);

            shader.create_uniform("font_texture");
            shader.set_int_uniform("font_texture", 1);

            shader.create_uniform("resolution");
            shader.set_vec2_f32_uniform("resolution", size.0 as f32, size.1 as f32);

            shader.create_uniform("cell_size");
            shader.set_vec2_f32_uniform("cell_size", 8.0, 8.0);

            shader.create_uniform("font_grid");
            shader.set_vec2_i32_uniform("font_grid", 15, 1);

            shader.create_uniform("glyph_count");
            shader.set_int_uniform("glyph_count", 15);

            shader.create_uniform("edge_threshold");
            shader.set_float_uniform("edge_threshold", 0.8);
        }

        SelectShader::Pixel => {
            shader = shader_reader::ShaderReader::new("shaders/pixel/vertex_shader.glsl", "shaders/pixel/fragment_shader.glsl");
            shader.bind();

            shader.create_uniform("resolution");
            shader.set_vec2_f32_uniform("resolution", size.0 as f32, size.1 as f32);

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

    shader
}