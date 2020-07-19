#version 450 core

in VS_OUTPUT {
    vec4 Color;
    vec2 Texcoord;
} vs_out;

uniform sampler2D InTexture;

out vec4 Color;

void main()
{
    Color = texture(InTexture, vs_out.Texcoord) * vs_out.Color;
}
