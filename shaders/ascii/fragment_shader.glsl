#version 330 core

uniform sampler2D font_texture;
uniform ivec2 font_grid;
uniform int glyph_count;

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

    // Luminance calculation for physiological perception by converting RGB values to YIQ values.
    // This brightness value gives a perceived brightness for a color by a human eye.
    float luminance = dot(avg_col, vec3(0.299, 0.587, 0.114));

    // Map brightness to glyph index
    int glyph_index = int(luminance * float(glyph_count - 1));
    glyph_index = clamp(glyph_index, 0, glyph_count - 1);

    // Size of one glyph in UV space
    vec2 glyph_size = 1.0 / vec2(font_grid); // width/height of one glyph in UVs
    int gx = glyph_index % font_grid.x;
    int gy = glyph_index / font_grid.x;

    // Sample font texture
    vec2 glyph_uv_origin = vec2(gx, gy) * glyph_size;
    vec2 local_uv = fract(pixel / cell_size); // 0..1 inside the cell
    vec2 font_uv = glyph_uv_origin + local_uv * glyph_size;
    float glyph_alpha = texture(font_texture, font_uv).a;

    FragColor = vec4(avg_col * glyph_alpha, 1.0);
}