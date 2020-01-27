#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec2 v_TexCoord;

void main() {
    v_TexCoord = position;
    gl_Position = vec4(position.x - 1.0, 1.0 - position.y, 0.0, 1.0);
}