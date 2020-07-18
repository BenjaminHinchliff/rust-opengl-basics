#version 450 core

in VS_OUTPUT {
    vec3 Color;
} vs_out;

out vec4 Color;

void main()
{
    Color = vec4(vs_out.Color, 1.0);
}
