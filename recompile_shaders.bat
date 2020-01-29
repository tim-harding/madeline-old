@pushd shaders

@pushd node_rasterize
glslc.exe -o frag.spv -fshader-stage=frag frag.glsl
glslc.exe -o vert.spv -fshader-stage=vert vert.glsl
@popd

@pushd node_resolution
glslc.exe -o frag.spv -fshader-stage=frag frag.glsl
glslc.exe -o vert.spv -fshader-stage=vert vert.glsl
@popd

@popd