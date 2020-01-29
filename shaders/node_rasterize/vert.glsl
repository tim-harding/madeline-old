#version 450

layout (location = 0) in vec2 position;

layout (set = 0, binding = 0) uniform Pass {
    vec2 offset;
} pass;

void main() {
    vec2 pos = (position + pass.offset) / vec2(92, 29) * vec2(2.0) - 1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}