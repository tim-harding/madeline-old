#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec2 v_TexCoord;

layout (set = 0, binding = 0) uniform Locals {
    vec2 screen_size;
} locals;

void main() {
    v_TexCoord = position;
    gl_Position = vec4(
        position.x /  locals.screen_size.x, 
        -position.y / locals.screen_size.y, 
        0.0, 
        1.0
    );
}