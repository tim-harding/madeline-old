#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = vec4(position - vec2(0.5), 0.0, 1.0);
}