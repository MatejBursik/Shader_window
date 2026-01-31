#version 330 core

uniform sampler2D font_texture;
uniform ivec2 font_grid;
uniform int glyph_count;

uniform sampler2D img_texture;
uniform vec2 resolution;
uniform vec2 cell_size;

uniform float edge_threshold;

in vec2 TexPos;

out vec4 FragColor;

float luminance(vec3 c) {
    return dot(c, vec3(0.299, 0.587, 0.114));
}

float sample_cell_luminance(vec2 base_cell, vec2 offset) {
    vec2 c = base_cell + offset;
    vec2 uv = (c * cell_size + cell_size * 0.5) / resolution;

    return luminance(texture(img_texture, uv).rgb);
}

void main() {
    vec2 pixel = TexPos * resolution;
    vec2 cell = floor(pixel / cell_size);

    // Read average color from a cell in the image
    vec2 cell_uv = (cell * cell_size + cell_size * 0.5) / resolution;
    vec3 avg_col = texture(img_texture, cell_uv).rgb;

    // Sobel on 3x3 kernel
    float tl = sample_cell_luminance(cell, vec2(-1, 1));
    float t = sample_cell_luminance(cell, vec2(0, 1));
    float tr = sample_cell_luminance(cell, vec2(1, 1));

    float l = sample_cell_luminance(cell, vec2(-1, 0));
    float c = sample_cell_luminance(cell, vec2(0, 0));
    float r = sample_cell_luminance(cell, vec2(1, 0));

    float bl = sample_cell_luminance(cell, vec2(-1, -1));
    float b = sample_cell_luminance(cell, vec2(0, -1));
    float br = sample_cell_luminance(cell, vec2(1, -1));

    // Sobel X
    float sx = -tl - 2.0*l - bl +
                tr + 2.0*r + br;

    // Sobel Y
    float sy = -tl - 2.0*t - tr +
                bl + 2.0*b + br;

    vec2 grad = vec2(sx, sy);
    float mag = length(grad); // Strength of the edge (0 = no edge, higher = stronger edge)

    // Choose correct glyph (fill or edge)
    int glyph_index;

    if (mag < edge_threshold) {
        // Map brightness to glyph index (non-edge characters)
        // -4 because last 4 glyphs area reserved for edge characters (— / \ |)
        glyph_index = int(c * float(glyph_count - 1 - 4));
        glyph_index = clamp(glyph_index, 0, glyph_count - 1 - 4);

    } else {
        // 4-direction ASCII edges based normalized vector comparison
        // Sobel returns the GRADIENT direction which is PERPENDICULAR to the actual edge
        // Normalize gradient so its length = 1
        //   |g.x| - how horizontal the gradient is
        //   |g.y| - how vertical the gradient is
        vec2 g = normalize(grad);
        float agx = abs(g.x);
        float agy = abs(g.y);

        if (agx > agy * 1.5) {
            glyph_index = glyph_count - 1; // |
        } else if (agy > agx * 1.5) {
            glyph_index = glyph_count - 4; // —
        } else if (g.x * g.y > 0.0) {
            glyph_index = glyph_count - 2; // \
        } else {
            glyph_index = glyph_count - 3; // /
        }
    }

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