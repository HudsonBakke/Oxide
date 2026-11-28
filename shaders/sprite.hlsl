// Straight-alpha linear colors. The sRGB output view handles final encoding and
// blending in linear space; UI is composed after the 3D exposure/tone-map pass.
cbuffer SpriteConstants : register(b0)
{
    float4 target;       // x,y,width,height in full-window physical pixels
    float4 source;       // normalized u,v,width,height
    float4 tint;
    float4 transform;   // origin x/y, cos, sin
    float4 viewport;    // window width/height, flip horizontal/vertical
    float4 uvClamp;     // min u/v, max u/v: avoids atlas-edge bleed
};
Texture2D<float4> sprite : register(t0);
SamplerState spriteSampler : register(s0);
struct VertexOutput { float4 position : SV_POSITION; float2 uv : TEXCOORD0; };
VertexOutput vs_main(uint vertex : SV_VertexID)
{
    static const uint indices[6]={0,1,2,0,2,3};
    static const float2 points[4]={float2(0,0),float2(1,0),float2(1,1),float2(0,1)};
    float2 point=points[indices[vertex]];
    float2 local=point*target.zw-transform.xy;
    float2 rotated=float2(local.x*transform.z-local.y*transform.w,local.x*transform.w+local.y*transform.z);
    float2 pixel=target.xy+transform.xy+rotated;
    VertexOutput output;
    output.position=float4(pixel.x/viewport.x*2-1,1-pixel.y/viewport.y*2,0,1);
    point=float2(viewport.z!=0?1-point.x:point.x,viewport.w!=0?1-point.y:point.y);
    output.uv=source.xy+point*source.zw;
    return output;
}
float4 ps_main(VertexOutput input) : SV_TARGET
{
    float4 color=sprite.Sample(spriteSampler,clamp(input.uv,uvClamp.xy,uvClamp.zw));
    return saturate(color*tint);
}
