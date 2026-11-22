#define PARTICLES 12

cbuffer C : register(b0)
{
    float4 rect;
    float4 misc;
    float4 bgc;
    float4 ringc;
    float4 glow[4];
    float4 ring[3];
    float4 part[PARTICLES];
};

static const float3 PINK   = float3(222.0, 163.0, 195.0) / 255.0;
static const float3 ORANGE = float3(214.0, 140.0, 110.0) / 255.0;
static const float3 VIOLET = float3(165.0, 135.0, 225.0) / 255.0;
static const float3 ROSE   = float3(240.0, 130.0, 170.0) / 255.0;

float falloff(float d, float rad)
{
    float f = saturate(1.0 - d / rad);
    return f * f;
}

float3 over(float3 dst, float3 src, float a)
{
    return lerp(dst, src, saturate(a));
}

float ign(float2 p)
{
    return frac(52.9829189 * frac(dot(p, float2(0.06711056, 0.00583715))));
}

float4 vs_main(uint id : SV_VertexID) : SV_Position
{
    float2 uv = float2((id << 1) & 2, id & 2);
    return float4(uv * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);
}

float4 ps_main(float4 sv : SV_Position) : SV_Target
{
    float ppp = misc.x;
    float aa  = 1.0 / ppp;
    float2 px = sv.xy;
    float2 p  = px / ppp; // координаты в points, как в egui

    float3 col = bgc.rgb;

    float3 gc[4] = { PINK, VIOLET, ROSE, ORANGE };
    [unroll]
    for (int g = 0; g < 4; g++)
    {
        float d = length(p - glow[g].xy);
        col = over(col, gc[g], glow[g].w * falloff(d, glow[g].z));
    }

    float dc = length(p - ringc.xy);
    [unroll]
    for (int r = 0; r < 3; r++)
    {
        float cov = saturate((0.6 - abs(dc - ring[r].x)) / aa + 0.5); // толщина 1.2
        col = over(col, PINK, ring[r].y * cov);
    }

    [unroll]
    for (int i = 0; i < PARTICLES; i++)
    {
        float2 pp = part[i].xy;
        float sz  = part[i].z;
        float a   = part[i].w;
        float3 c  = (i % 4 == 0) ? ORANGE : PINK;
        float d   = length(p - pp);

        if (i % 8 == 0)
            col = over(col, c, a * 0.28 * falloff(d, 10.0 + sz * 7.0));

        col = over(col, c, a * saturate((sz - d) / aa + 0.5));
    }

    float2 ctr = (rect.xy + rect.zw * 0.5) * ppp;
    float2 hs  = rect.zw * 0.5 * ppp;
    float rad  = misc.y * ppp;
    float2 q   = abs(px - ctr) - (hs - rad);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - rad;
    float mask = saturate(0.5 - dist);

    col += (ign(sv.xy) - 0.5) / 255.0;

    return float4(col * mask, mask);
}
