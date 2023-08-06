#version 300 es
precision highp float;
precision highp int;

layout (location = 0) in vec2 i_position;
layout (location = 1) in vec2 i_uv;

out vec2 v_uv;

uniform mat4 u_mvp;

void main()
{
    gl_Position = u_mvp * vec4(i_position, 0.0, 1.0);
    v_uv = i_uv;
}
