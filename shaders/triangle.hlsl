cbuffer ObjectConstants : register(b0)
{
    float4x4 model;
    float4x4 normal_matrix;
};

cbuffer ViewConstants : register(b1)
{
    float4x4 view;
    float4x4 projection;
};

Texture2D base_texture : register(t0);
SamplerState base_sampler : register(s0);

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
};

struct VertexInput
{
    float3 position  : POSITION;
    float3 normal    : NORMAL;
    float2 tex_coord : TEXCOORD;
};

struct VertexOutput
{
    float4 position       : SV_POSITION;
    float3 world_position : POSITION0;
    float3 normal         : NORMAL;
    float2 tex_coord      : TEXCOORD;
};

VertexOutput vs_main(VertexInput input)
{
    VertexOutput output;

    float4 world_position =
        mul(
            model,
            float4(input.position, 1.0)
        );

    output.world_position =
        world_position.xyz;

    output.normal =
        normalize(
            mul(
                normal_matrix,
                float4(input.normal, 0.0)
            ).xyz
        );

    float4 view_position =
        mul(
            view,
            world_position
        );

    output.position =
        mul(
            projection,
            view_position
        );

    output.tex_coord =
        input.tex_coord;

    return output;
}

float4 ps_main(VertexOutput input) : SV_TARGET
{
    float3 normal =
        normalize(input.normal);

    float3 base_color =
        base_texture.Sample(
            base_sampler,
            input.tex_coord
        ).rgb;

    float3 lighting =
        float3(0.0, 0.0, 0.0);

    for (
        uint i = 0;
        i < point_light_count;
        ++i
    )
    {
        float3 to_light =
            point_lights[i].position.xyz
            - input.world_position;

        float distance_squared =
            max(
                dot(to_light, to_light),
                0.0001
            );

        float3 light_direction =
            normalize(to_light);

        float diffuse =
            max(
                dot(
                    normal,
                    light_direction
                ),
                0.0
            );

        float intensity =
            point_lights[i].position.w
            / distance_squared;

        lighting +=
            point_lights[i].color.rgb
            * intensity
            * diffuse;
    }

    for (
        uint i = 0;
        i < directional_light_count;
        ++i
    )
    {
        // The stored direction describes the direction
        // the light rays travel, so the direction from
        // the surface toward the light is its opposite.
        float3 light_direction =
            -normalize(
                directional_lights[i]
                    .direction.xyz
            );

        float diffuse =
            max(
                dot(
                    normal,
                    light_direction
                ),
                0.0
            );

        float intensity =
            directional_lights[i].color.a;

        lighting +=
            directional_lights[i].color.rgb
            * intensity
            * diffuse;
    }

    for (
        uint i = 0;
        i < spot_light_count;
        ++i
    )
    {
        float3 from_light =
            input.world_position
            - spot_lights[i].position.xyz;

        float distance_squared =
            max(
                dot(
                    from_light,
                    from_light
                ),
                0.0001
            );

        float3 light_to_fragment =
            normalize(from_light);

        float3 spot_direction =
            normalize(
                spot_lights[i].direction.xyz
            );

        float cone_alignment =
            dot(
                spot_direction,
                light_to_fragment
            );

        float cone_cutoff =
            spot_lights[i].direction.w;

        if (cone_alignment >= cone_cutoff)
        {
            // Diffuse lighting needs the direction
            // FROM the surface TOWARD the light.
            float3 fragment_to_light =
                -light_to_fragment;

            float diffuse =
                max(
                    dot(
                        normal,
                        fragment_to_light
                    ),
                    0.0
                );

            float intensity =
                spot_lights[i].position.w
                / distance_squared;

            lighting +=
                spot_lights[i].color.rgb
                * intensity
                * diffuse;
        }
    }

    return float4(
        base_color * lighting,
        1.0
    );
}