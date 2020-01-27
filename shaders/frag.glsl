#version 450

// Input and output locations can overlap
layout (location = 0) in vec2 v_TexCoord;

// Set bind group's first parameter corresponds to `set`.
// This allows, for example, a separate bind group for 
// globals and for individual entities. 
layout (set = 0, binding = 0) uniform texture2D t_Color;
layout (set = 0, binding = 1) uniform sampler s_Color;

// Vulkan requires that uniform blocks be used
// to define uniforms. Location layout is invalid.
layout (set = 0, binding = 2) uniform Locals {
    float brightness;

// Optional namespacing of members under `local`
} locals;

layout (location = 0) out vec4 color;

void main() {
    vec4 tx = texture(sampler2D(t_Color, s_Color), v_TexCoord);
    color = tx * vec4(vec3(locals.brightness), 1.0);
}