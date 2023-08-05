#version 300 es
precision highp float;
precision highp int;

in vec2 v_uv;
out vec4 Color;

uniform sampler2D u_texture;

void main()
{
    Color = texture(u_texture, v_uv);
    //Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
