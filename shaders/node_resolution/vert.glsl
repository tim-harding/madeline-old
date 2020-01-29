#version 450

layout(location = 0) in vec2 a_Pos;

layout(location = 0) out vec2 v_TexCoord;

void main() {
    v_TexCoord = a_Pos;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
