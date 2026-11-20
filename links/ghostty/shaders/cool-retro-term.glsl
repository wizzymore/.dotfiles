// Cool Retro CRT for Ghostty. Original implementation inspired by
// https://github.com/dragontpe/cool-retro-term (not a source-code port),
// with fine horizontal scanlines and black corners in the style of Dispatch.
// Ghostty supplies iChannel0, iResolution and iTime. No #version needed.
// Edit these constants, save, then reload Ghostty's configuration.
// Measured from a 1600x900 Dispatch capture: the sides sit 24px in and bow
// 12px at the corners; the top and bottom touch the frame and bow 22px.
const vec2 CURVATURE = vec2(0.015, 0.05); // bend of the sides, top/bottom; 0 = flat
const vec2 SCREEN_INSET = vec2(0.015, 0.0); // black margin per edge: sides, top/bottom
const float CORNER_RADIUS = 0.0;  // 0 = sharp corners; fraction of screen
const float BLOOM = 0.2;          // 0 = off
const float SCANLINES = 0.40;     // darkness of the dark pixel row; 0 = off
const float SCANLINE_PX = 3.0;    // physical pixels per scanline (lit rows + 1 dark row)
const float NOISE = 0.02;         // animated grain
const float STATIC_NOISE = 0.015; // fixed grain baked into the glass
const float FLICKER = 0.003;      // whole-screen brightness pulse
const float JITTER_PX = 0.13;
const float ROLLING_GLOW = 0.06;  // bright band drifting down the screen; 0 = off
const float ROLLING_SPEED = 0.08; // screen heights per second
const float VIGNETTE = 0.22;
const float BRIGHTNESS = 1.15;    // compensates for light lost to scanlines
const float PI = 3.14159265359;

float hashCRT(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}
vec3 screenCRT(vec2 uv) {
    // Clamp to texel centers so filtering never blends with pixels outside
    // the texture; the bezel mask below hides everything past the edge.
    vec2 halfPixel = 0.5 / max(iResolution.xy, vec2(1.0));
    return texture(iChannel0, clamp(uv, halfPixel, vec2(1.0) - halfPixel)).rgb;
}
// 1 inside the rounded screen rectangle, 0 outside, antialiased over ~1px.
float screenMask(vec2 uv, vec2 res) {
    vec2 aspect = vec2(res.x / res.y, 1.0);
    vec2 d = abs(uv - 0.5) * aspect;
    vec2 halfSize = 0.5 * aspect;
    vec2 corner = d - halfSize + CORNER_RADIUS;
    float dist = length(max(corner, 0.0)) + min(max(corner.x, corner.y), 0.0)
        - CORNER_RADIUS;
    float aa = 0.5 / res.y; // just enough to avoid stair-stepping
    return 1.0 - smoothstep(-aa, 0.0, dist);
}
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 res = max(iResolution.xy, vec2(1.0));
    vec2 uv = fragCoord / res;
    vec2 p = uv * 2.0 - 1.0;
    // Each axis bends by the square of the other one, so the middle of every
    // edge bulges outward and the corners curl in. The inset keeps a black
    // margin on the sides; the middle of the top and bottom edges touches the
    // window, as in Dispatch.
    vec2 q = p * (1.0 + CURVATURE * p.yx * p.yx) / (1.0 - 2.0 * SCREEN_INSET);
    vec2 sampleUV = q * 0.5 + 0.5;
    float mask = screenMask(sampleUV, res);
    if (mask <= 0.0) {
        // Real black outside the tube: no glow, grain or tint.
        fragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    float frame = floor(iTime * 30.0);
    sampleUV.x += (hashCRT(vec2(floor(fragCoord.y / 4.0), frame)) - 0.5)
        * JITTER_PX / res.x;
    vec2 px = 1.0 / res;
    vec3 color = screenCRT(sampleUV);
    // Two rings approximate a soft phosphor halo without feedback buffers.
    vec3 nearGlow = vec3(0.0);
    vec3 farGlow = vec3(0.0);
    for (int j = 0; j < 8; j++) {
        float angle = float(j) * PI * 0.25;
        vec2 dir = vec2(cos(angle), sin(angle));
        nearGlow += screenCRT(sampleUV + dir * px * 2.0);
        farGlow += screenCRT(sampleUV + dir * px * 6.0);
    }
    color += BLOOM * (nearGlow * 0.085 + farGlow * 0.040);
    // Fine horizontal scanlines: the last physical pixel row of each line is
    // dimmed. Screen-space rows stay crisp and avoid moire from the warp.
    float row = mod(floor(fragCoord.y), SCANLINE_PX);
    color *= 1.0 - SCANLINES * step(SCANLINE_PX - 1.0, row);
    // Soft band: brightens text and also lifts the dark background slightly,
    // so it stays visible on a near-black terminal without flashing.
    float band = exp(-pow((fract(uv.y + iTime * ROLLING_SPEED) - 0.5) / 0.07, 2.0));
    color *= 1.0 + ROLLING_GLOW * band;
    color += ROLLING_GLOW * 0.25 * band;
    color *= 1.0 + FLICKER * (sin(iTime * 71.0) + 0.5 * sin(iTime * 113.0));
    // Grain: a moving layer plus a fixed one, slightly stronger in highlights.
    float luma = dot(color, vec3(0.299, 0.587, 0.114));
    float grain = (hashCRT(fragCoord + vec2(frame, frame * 0.71)) - 0.5) * NOISE
        + (hashCRT(floor(fragCoord) * 0.37 + 11.0) - 0.5) * STATIC_NOISE;
    color += grain * (0.4 + 0.6 * sqrt(clamp(luma, 0.0, 1.0)));
    color *= 1.0 - VIGNETTE * smoothstep(0.15, 1.6, dot(p, p));
    fragColor = vec4(clamp(color * BRIGHTNESS, 0.0, 1.0) * mask, 1.0);
}
