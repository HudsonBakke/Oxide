cbuffer ObjectConstants : register(b0)
{
    float4x4 model;
};

struct VertexInput
{
    float3 position  : POSITION;
    float3 normal    : NORMAL;
    float2 tex_coord : TEXCOORD;
};

struct VertexOutput
{
    float4 position  : SV_POSITION;
    float3 normal    : NORMAL;
    float2 tex_coord : TEXCOORD;
};

VertexOutput vs_main(VertexInput input)
{
    VertexOutput output;

    output.position = mul(
        model,
        float4(input.position, 1.0)
    );
    output.normal = input.normal;
    output.tex_coord = input.tex_coord;

    return output;
}

float4 ps_main(VertexOutput input) : SV_TARGET
{
    return float4(
        input.tex_coord.x,
        input.tex_coord.y,
        0.2,
        1.0
    );
}