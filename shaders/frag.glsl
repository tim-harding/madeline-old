#version 450

layout (location = 0) out vec4 color;

layout (set = 1, binding = 1) uniform Pass {
    vec3 offset;
} pass;

void main() {
    color = vec4(1.0);
}