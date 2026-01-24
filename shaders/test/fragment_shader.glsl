#version 330 core

uniform sampler2D img_texture;
uniform float time;

in vec2 TexPos;

out vec4 FragColor;

void main() {
    // Read color from image
    vec4 col = texture(img_texture, TexPos);

    // Find average to make greyscale
    float avg = (col.r + col.g + col.b) / 3.0;
    vec3 background = vec3(avg);

    // Wave animation
    float wave = sin(TexPos.x * 32.0 + time * 2.0);
    float line = smoothstep(0.0, 1.5, wave);
    vec3 color = mix(background, vec3(1.0, 0.0, 0.0), line);

    FragColor = vec4(color, 1.0);
}