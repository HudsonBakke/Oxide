// Scene color remains linear HDR until this single, full-surface display pass.
cbuffer DisplayConstants : register(b0)
{
    float exposure;
    float3 padding;
};

Texture2D<float4> scene_color : register(t0);

float4 vs_main(uint vertex_id : SV_VertexID) : SV_POSITION
{
    float2 positions[3] = {
        float2(-1.0, -1.0), float2(-1.0, 3.0), float2(3.0, -1.0)
    };
    return float4(positions[vertex_id], 0.0, 1.0);
}

float4 ps_main(float4 position : SV_POSITION) : SV_TARGET
{
    float4 scene = scene_color.Load(int3(int2(position.xy), 0));
    // Keep the exposure multiplication bounded even for extreme HDR inputs.
    float3 exposed = exposure == 0.0 ? float3(0.0, 0.0, 0.0)
                                    : min(max(scene.rgb, 0.0) * exposure, 1.0e20);
    float3 mapped = exposed / (1.0 + exposed);
    // The sRGB render-target view encodes after linear-space blending.
    return float4(mapped, saturate(scene.a));
}
