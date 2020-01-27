#version 450

layout (location = 0) in vec2 v_TexCoord;
layout (location = 1) uniform texture2D t_Color;
layout (location = 2) uniform sampler s_Color;

layout (location = 0) out vec4 color;

void main() {
    color = texture(sampler2D(t_Color, s_Color), v_TexCoord);
    // color = vec4(v_TexCoord, 1.0, 1.0);
}