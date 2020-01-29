#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec2 v_TexCoord;

layout (set = 0, binding = 0) uniform Globals {
    vec2 node_size;
} globals;

layout (set = 1, binding = 0) uniform Pass {
    vec2 offset;
} pass;

void main() {
    v_TexCoord = position;
    vec2 pos = (position + pass.offset) / globals.node_size - 1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}