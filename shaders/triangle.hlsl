struct VertexOutput
{
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

VertexOutput vs_main(uint vertex_id : SV_VertexID)
{
    float2 positions[3] =
    {
        float2( 0.0,  0.5),
        float2( 0.5, -0.5),
        float2(-0.5, -0.5)
    };

    float3 colors[3] =
    {
        float3(1.0, 0.0, 0.0),
        float3(0.0, 1.0, 0.0),
        float3(0.0, 0.0, 1.0)
    };

    VertexOutput output;
    output.position = float4(positions[vertex_id], 0.0, 1.0);
    output.color = colors[vertex_id];

    return output;
}

float4 ps_main(VertexOutput input) : SV_TARGET
{
    return float4(input.color, 1.0);
}