#version 450

layout(location = 0) in vec2 a_Pos;

layout(location = 0) out vec2 v_TexCoord;

layout (set = 0, binding = 2) uniform Globals {
    vec2 screen_size;
} globals;

void main() {
    v_TexCoord = a_Pos;
    gl_Position = vec4(a_Pos * vec2(182, 56) / globals.screen_size, 0.0, 1.0);
}
