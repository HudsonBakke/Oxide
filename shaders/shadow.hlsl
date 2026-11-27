cbuffer ShadowObjectConstants : register(b0)
{
    float4x4 model_to_light_clip;
    float4 transmission_parameters; // x=transparency, y=first layer
};

float4 vs_main(float3 position : POSITION) : SV_POSITION
{
    return mul(model_to_light_clip, float4(position, 1.0));
}

Texture2DArray<float> previous_depth : register(t0);

float ps_main(float4 position : SV_POSITION) : SV_TARGET
{
    if (transmission_parameters.y < 0.5)
    {
        float previous = previous_depth.Load(int4(int2(position.xy), 0, 0));
        if (position.z <= previous + 0.000001)
            discard;
    }
    return transmission_parameters.x;
}
