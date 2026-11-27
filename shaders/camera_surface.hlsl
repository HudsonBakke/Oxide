// Camera feeds stay in linear HDR. The ordinary final display pass tone maps once.
cbuffer ViewConstants : register(b0)
{
    float4x4 view;
    float4x4 projection;
    float4 cameraPosition;
};
cbuffer SurfaceConstants : register(b1)
{
    float4 corners[4];
};
Texture2D<float4> cameraFeed : register(t0);
SamplerState feedSampler : register(s0);
struct VertexOutput
{
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
};
VertexOutput vs_main(uint vertex : SV_VertexID)
{
    static const uint indices[6] = {0, 1, 2, 0, 2, 3};
    static const float2 uvs[4] = {float2(0,0), float2(1,0), float2(1,1), float2(0,1)};
    uint corner = indices[vertex];
    VertexOutput output;
    output.position = mul(projection, mul(view, corners[corner]));
    output.uv = uvs[corner];
    return output;
}
float4 ps_main(VertexOutput input) : SV_TARGET
{
    return float4(cameraFeed.Sample(feedSampler, input.uv).rgb, 1.0);
}
