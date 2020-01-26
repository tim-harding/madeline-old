@pushd shaders
glslc.exe -o frag.spv -fshader-stage=frag frag.glsl
glslc.exe -o vert.spv -fshader-stage=vert vert.glsl
@popd