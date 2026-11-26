cbuffer ShadowObjectConstants : register(b0)
{
    float4x4 model_to_light_clip;
};

float4 vs_main(float3 position : POSITION) : SV_POSITION
{
    return mul(model_to_light_clip, float4(position, 1.0));
}
