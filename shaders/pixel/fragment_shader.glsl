#version 330 core

uniform sampler2D img_texture;
uniform vec2 resolution;
uniform vec2 cell_size;

in vec2 TexPos;

out vec4 FragColor;

void main() {
    vec2 pixel = TexPos * resolution;
    vec2 cell = floor(pixel / cell_size);

    // Read average color from a cell in the image
    vec2 cell_uv = (cell * cell_size + cell_size * 0.5) / resolution;
    vec3 avg_col = texture(img_texture, cell_uv).rgb;

    FragColor = vec4(avg_col, 1.0);
}