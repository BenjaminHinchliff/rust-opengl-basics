#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 Texcoord;

out VS_OUTPUT {
    vec4 Color;
    vec2 Texcoord;
} vs_out;

void main()
{
    gl_Position = vec4(Position, 1.0);
    vs_out.Color = Color;
    vs_out.Texcoord = Texcoord;
}
