cbuffer BackgroundConstants : register(b0)
{
    float4 camera_right;
    float4 camera_up;
    float4 camera_forward;
    float4 projection_params; // x=tan(fov/2), y=aspect
};

Texture2D background_texture : register(t0);
SamplerState background_sampler : register(s0);

struct VsOut
{
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut output;
    float2 positions[3] = {float2(-1.0, -1.0), float2(-1.0, 3.0), float2(3.0, -1.0)};
    float2 p = positions[vertex_id];
    output.position = float4(p, 1.0, 1.0);
    output.uv = p * float2(0.5, -0.5) + 0.5;
    return output;
}

float2 direction_to_uv(float3 direction)
{
    direction = normalize(direction);
    return float2(atan2(direction.z, direction.x) / (2.0 * 3.14159265) + 0.5,
                  acos(clamp(direction.y, -1.0, 1.0)) / 3.14159265);
}

float4 ps_main(VsOut input) : SV_TARGET
{
    float2 ndc = input.uv * 2.0 - 1.0;
    ndc.y = -ndc.y;
    float3 direction = normalize(
        camera_forward.xyz + camera_right.xyz * ndc.x * projection_params.x * projection_params.y +
        camera_up.xyz * ndc.y * projection_params.x);
    float3 hdr = background_texture.Sample(background_sampler, direction_to_uv(direction)).rgb;

    // Display transform only. IBL samples remain unclamped linear radiance.
    float3 mapped = hdr / (hdr + 1.0);
    mapped = pow(mapped, 1.0 / 2.2);
    return float4(mapped, 1.0);
}
