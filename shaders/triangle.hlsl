cbuffer ObjectConstants : register(b0)
{
    float4x4 model;
    float4x4 normal_matrix;
    float4 base_color_factor;
    float4 material_factors; // x=metallic, y=roughness, z=emissive strength
    float4 emissive_color;
};

cbuffer ViewConstants : register(b1)
{
    float4x4 view;
    float4x4 projection;
    float4 camera_position;
};

Texture2D base_texture : register(t0);
Texture2D normal_texture : register(t1);
Texture2D environment_texture : register(t2);
Texture2D irradiance_texture : register(t3);
Texture2D background_texture : register(t4);
SamplerState base_sampler : register(s0);
Texture2D<float> directional_shadow_map : register(t5);
Texture2DArray<float> transmission_depth : register(t6);
Texture2DArray<float> transmission_values : register(t7);
SamplerComparisonState shadow_sampler : register(s1);

cbuffer ShadowConstants : register(b3)
{
    float4x4 world_to_shadow;
    float4 shadow_parameters; // x=depth bias, y=normal bias, z=texel size, w=enabled
    float4 transmission_parameters; // x=texel size, y=layer count, z=enabled
};

struct PointLight
{
    float4 position;
    float4 color;
};

struct SpotLight
{
    float4 position;
    float4 direction;
    float4 color;
};

struct DirectionalLight
{
    float4 direction;
    float4 color;
};

cbuffer LightConstants : register(b2)
{
    PointLight point_lights[8];
    SpotLight spot_lights[8];
    DirectionalLight directional_lights[8];

    uint point_light_count;
    uint spot_light_count;
    uint directional_light_count;
    uint light_padding;

    float4 environment_ambient; // rgb=color, a=intensity
    float4
        environment_params; // x=IBL intensity, y=IBL texture enabled, z=background texture enabled
};

struct VertexInput
{
    float3 position : POSITION;
    float3 normal : NORMAL;
    float2 tex_coord : TEXCOORD;
    float4 tangent : TANGENT;
};

struct VertexOutput
{
    float4 position : SV_POSITION;
    float3 world_position : POSITION0;
    float3 normal : NORMAL;
    float2 tex_coord : TEXCOORD;
    float3 tangent : TANGENT0;
    float3 bitangent : TANGENT1;
};

VertexOutput vs_main(VertexInput input)
{
    VertexOutput output;

    float4 world_position = mul(model, float4(input.position, 1.0));

    output.world_position = world_position.xyz;

    output.normal = normalize(mul(normal_matrix, float4(input.normal, 0.0)).xyz);

    float4 view_position = mul(view, world_position);

    output.position = mul(projection, view_position);

    output.tex_coord = input.tex_coord;

    float3 world_tangent = normalize(mul(normal_matrix, float4(input.tangent.xyz, 0.0)).xyz);
    world_tangent = normalize(world_tangent - output.normal * dot(output.normal, world_tangent));
    output.tangent = world_tangent;
    output.bitangent = normalize(cross(output.normal, world_tangent)) * input.tangent.w;

    return output;
}

float3 evaluate_brdf(float3 normal, float3 view_direction, float3 light_direction, float3 radiance,
                     float3 albedo, float metallic, float roughness)
{
    float n_dot_l = max(dot(normal, light_direction), 0.0);
    float n_dot_v = max(dot(normal, view_direction), 0.0);

    if (n_dot_l <= 0.0 || n_dot_v <= 0.0)
        return float3(0.0, 0.0, 0.0);

    float3 half_vector = normalize(view_direction + light_direction);
    float n_dot_h = max(dot(normal, half_vector), 0.0);
    float v_dot_h = max(dot(view_direction, half_vector), 0.0);

    float alpha = max(roughness * roughness, 0.0025);
    float alpha2 = alpha * alpha;
    float denom = n_dot_h * n_dot_h * (alpha2 - 1.0) + 1.0;
    float distribution = alpha2 / max(3.14159265 * denom * denom, 0.0001);

    float k = ((roughness + 1.0) * (roughness + 1.0)) / 8.0;
    float g_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
    float g_l = n_dot_l / (n_dot_l * (1.0 - k) + k);

    float3 f0 = lerp(float3(0.04, 0.04, 0.04), albedo, metallic);
    float3 fresnel = f0 + (1.0 - f0) * pow(1.0 - v_dot_h, 5.0);

    float3 specular = distribution * g_v * g_l * fresnel / max(4.0 * n_dot_v * n_dot_l, 0.0001);

    float3 diffuse = (1.0 - fresnel) * (1.0 - metallic) * albedo / 3.14159265;

    return (diffuse + specular) * radiance * n_dot_l;
}

float2 direction_to_equirectangular_uv(float3 direction)
{
    direction = normalize(direction);
    float u = atan2(direction.z, direction.x) / (2.0 * 3.14159265) + 0.5;
    float v = acos(clamp(direction.y, -1.0, 1.0)) / 3.14159265;
    return float2(u, v);
}

float3 sample_environment_texture(float3 direction)
{
    return environment_texture.Sample(base_sampler, direction_to_equirectangular_uv(direction)).rgb;
}

float3 filtered_environment(float3 reflection_direction, float roughness)
{
    // Small environments may have fewer than the usual five roughness levels.
    uint width, height, levels;
    environment_texture.GetDimensions(0, width, height, levels);
    return environment_texture
        .SampleLevel(base_sampler, direction_to_equirectangular_uv(reflection_direction),
                     roughness * float(levels - 1))
        .rgb;
}

float3 evaluate_environment(float3 normal, float3 view_direction, float3 albedo, float metallic,
                            float roughness)
{
    float n_dot_v = max(dot(normal, view_direction), 0.0);
    float3 f0 = lerp(float3(0.04, 0.04, 0.04), albedo, metallic);
    float3 fresnel = f0 + (1.0 - f0) * pow(1.0 - n_dot_v, 5.0);

    float3 ambient = environment_ambient.rgb * environment_ambient.a * albedo * (1.0 - metallic) *
                     (1.0 - fresnel);

    if (environment_params.y < 0.5)
        return ambient;

    float3 reflection_direction = reflect(-view_direction, normal);
    float3 specular_environment = filtered_environment(reflection_direction, roughness);

    float3 diffuse_environment =
        irradiance_texture.Sample(base_sampler, direction_to_equirectangular_uv(normal)).rgb;

    float intensity = environment_params.x;
    // The precomputed map stores irradiance; Lambert's BRDF contributes 1/pi.
    float3 diffuse = diffuse_environment * intensity * albedo * (1.0 - metallic) *
                     (1.0 - fresnel) / 3.14159265;

    float specular_energy = 1.0 - 0.45 * roughness;

    return ambient + diffuse + specular_environment * intensity * fresnel * specular_energy;
}

float directional_visibility(float3 world_position, float3 geometric_normal)
{
    if (shadow_parameters.w < 0.5)
        return 1.0;
    float4 clip = mul(world_to_shadow,
                      float4(world_position + geometric_normal * shadow_parameters.y, 1.0));
    float3 projected = clip.xyz / clip.w;
    float2 uv = projected.xy * float2(0.5, -0.5) + 0.5;
    if (projected.z <= 0.0 || projected.z >= 1.0 || any(uv < 0.0) || any(uv > 1.0))
        return 1.0;

    float visibility = 0.0;
    [unroll]
    for (int y = -1; y <= 1; ++y)
    {
        [unroll]
        for (int x = -1; x <= 1; ++x)
        {
            float sample_visibility = directional_shadow_map.SampleCmpLevelZero(
                shadow_sampler, uv + float2(x, y) * shadow_parameters.z,
                projected.z - shadow_parameters.x);
            if (sample_visibility > 0.0 && transmission_parameters.z > 0.5)
            {
                int resolution = int(round(1.0 / transmission_parameters.x));
                int2 pixel = int2(floor(uv * resolution)) + int2(x, y);
                if (all(pixel >= 0) && all(pixel < resolution))
                {
                    // Depth peeling records successive transparent surfaces.
                    // Stop before the receiver, or at the first empty layer.
                    [loop]
                    for (int layer = 0; layer < int(transmission_parameters.y); ++layer)
                    {
                        float depth = transmission_depth.Load(int4(pixel, layer, 0));
                        if (depth >= projected.z - shadow_parameters.x)
                            break;
                        sample_visibility *= transmission_values.Load(int4(pixel, layer, 0));
                    }
                }
            }
            visibility += sample_visibility;
        }
    }
    return visibility / 9.0;
}

float4 ps_main(VertexOutput input) : SV_TARGET
{
    float3 geometric_normal = normalize(input.normal);
    float3 tangent = normalize(input.tangent);
    float3 bitangent = normalize(input.bitangent);

    float3 sampled_normal = normal_texture.Sample(base_sampler, input.tex_coord).xyz * 2.0 - 1.0;
    sampled_normal.xy *= material_factors.w;
    sampled_normal = normalize(sampled_normal);

    float3 normal = normalize(tangent * sampled_normal.x + bitangent * sampled_normal.y +
                              geometric_normal * sampled_normal.z);

    float3 view_direction = normalize(camera_position.xyz - input.world_position);

    float4 sampled = base_texture.Sample(base_sampler, input.tex_coord);

    float3 albedo = sampled.rgb * base_color_factor.rgb;

    float metallic = saturate(material_factors.x);
    float roughness = saturate(material_factors.y);

    float3 lighting = float3(0.0, 0.0, 0.0);

    for (uint i = 0; i < point_light_count; ++i)
    {
        float3 to_light = point_lights[i].position.xyz - input.world_position;
        float distance_squared = max(dot(to_light, to_light), 0.0001);
        float3 light_direction = normalize(to_light);
        float3 radiance = point_lights[i].color.rgb * point_lights[i].position.w / distance_squared;

        lighting += evaluate_brdf(normal, view_direction, light_direction, radiance, albedo,
                                  metallic, roughness);
    }

    for (uint i = 0; i < directional_light_count; ++i)
    {
        float3 light_direction = -normalize(directional_lights[i].direction.xyz);
        float3 radiance = directional_lights[i].color.rgb * directional_lights[i].color.a;
        if (directional_lights[i].direction.w > 0.5)
            radiance *= directional_visibility(input.world_position, geometric_normal);

        lighting += evaluate_brdf(normal, view_direction, light_direction, radiance, albedo,
                                  metallic, roughness);
    }

    for (uint i = 0; i < spot_light_count; ++i)
    {
        float3 from_light = input.world_position - spot_lights[i].position.xyz;
        float distance_squared = max(dot(from_light, from_light), 0.0001);
        float3 light_to_fragment = normalize(from_light);
        float cone_alignment = dot(normalize(spot_lights[i].direction.xyz), light_to_fragment);

        if (cone_alignment >= spot_lights[i].direction.w)
        {
            float3 radiance =
                spot_lights[i].color.rgb * spot_lights[i].position.w / distance_squared;

            lighting += evaluate_brdf(normal, view_direction, -light_to_fragment, radiance, albedo,
                                      metallic, roughness);
        }
    }

    lighting += evaluate_environment(normal, view_direction, albedo, metallic, roughness);

    float3 emissive = emissive_color.rgb * material_factors.z;

    return float4(lighting + emissive, sampled.a * base_color_factor.a);
}
