#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec4 Color;
} vs_out;

void main()
{
    gl_Position = vec4(Position, 1.0);
    vs_out.Color = Color;
}
