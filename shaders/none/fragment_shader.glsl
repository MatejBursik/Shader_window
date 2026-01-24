#version 330 core

uniform sampler2D img_texture;

in vec2 TexPos;

out vec4 FragColor;

void main() {
    // Read color from image
    vec4 col = texture(img_texture, TexPos);

    FragColor = col;
}