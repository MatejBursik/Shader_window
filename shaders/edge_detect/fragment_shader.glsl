#version 330 core

uniform sampler2D img_texture;
uniform vec2 resolution;

in vec2 TexPos;

out vec4 FragColor;

float luminance(vec3 c) {
    return dot(c, vec3(0.299, 0.587, 0.114));
}

void main() {
    vec2 resolution_uv = 1.0 / resolution;

    // 3x3 kernel samples
    float tl = luminance(texture(img_texture, TexPos + resolution_uv * vec2(-1.0, 1.0)).rgb);
    float t = luminance(texture(img_texture, TexPos + resolution_uv * vec2(0.0, 1.0)).rgb);
    float tr = luminance(texture(img_texture, TexPos + resolution_uv * vec2(1.0, 1.0)).rgb);

    float l = luminance(texture(img_texture, TexPos + resolution_uv * vec2(-1.0, 0.0)).rgb);
    float r = luminance(texture(img_texture, TexPos + resolution_uv * vec2(1.0, 0.0)).rgb);

    float bl = luminance(texture(img_texture, TexPos + resolution_uv * vec2(-1.0, -1.0)).rgb);
    float b = luminance(texture(img_texture, TexPos + resolution_uv * vec2(0.0, -1.0)).rgb);
    float br = luminance(texture(img_texture, TexPos + resolution_uv * vec2(1.0, -1.0)).rgb);

    // Sobel X
    float gx = -tl - 2.0*l - bl +
                tr + 2.0*r + br;

    // Sobel Y
    float gy = -tl - 2.0*t - tr +
                bl + 2.0*b + br;

    // Scharr X
    //float gx = -3.0*tl - 10.0*l - 3.0*bl +
                  3.0*tr + 10.0*r + 3.0*br;

    // Scharr Y
    //float gy = -3.0*tl - 10.0*t - 3.0*tr +
    //            3.0*bl + 10.0*b + 3.0*br;

    float g = length(vec2(gx, gy));

    FragColor = vec4(vec3(g), 1.0);
}