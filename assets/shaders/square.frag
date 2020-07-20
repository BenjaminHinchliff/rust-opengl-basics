#version 450 core

in VS_OUTPUT {
    vec4 Color;
    vec2 Texcoord;
} vs_out;

uniform sampler2D container;
uniform sampler2D face;

out vec4 Color;

void main()
{
    Color = mix(texture(container, vs_out.Texcoord), texture(face, vs_out.Texcoord), 0.2) * vs_out.Color;
}
