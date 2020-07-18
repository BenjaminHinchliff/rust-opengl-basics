#version 450 core

in VS_OUTPUT {
    vec4 Color;
} vs_out;

out vec4 Color;

void main()
{
    Color = vs_out.Color;
}
