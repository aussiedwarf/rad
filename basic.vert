#version 330 core

layout (location = 0) in vec2 i_position;
layout (location = 1) in vec2 i_indices;

out vec2 v_indices;

void main()
{
    gl_Position = vec4(i_position, 0.0, 1.0);
    v_indices = i_indices;
}
