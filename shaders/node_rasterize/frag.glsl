#version 450

layout (location = 0) out vec4 color;

layout (set = 0, binding = 1) uniform Pass {
    vec3 color;
} pass;

void main() {
    color = vec4(pass.color, 1.0);
}