// Cool Retro CRT for Ghostty. Original implementation inspired by
// https://github.com/dragontpe/cool-retro-term (not a source-code port).
// Ghostty supplies iChannel0, iResolution and iTime. No #version needed.
// Edit these constants, save, then reload Ghostty's configuration.
const float CURVATURE = 0.005;      // 0 = flat; keep below 0.25
const float SCREEN_INSET = 0.00; // margin per edge, as a fraction of screen size
const float BLOOM = 0.2;          // 0 = off
const float SCANLINES = 0.4;     // 0 = off
const float SCANLINE_PX = 4.0;    // physical pixels per scanline (Retina friendly)
const float NOISE = 0.008;
const float FLICKER = 0.009;
const float JITTER_PX = 0.13;
const float ROLLING_GLOW = 0.035;
const float VIGNETTE = 0.22;
const float COLOR_MODE = 1.5;     // 0 = original colors, 1 = amber, 2 = green
const float BRIGHTNESS = 1.10;
const vec3 AMBER = vec3(1.0, 0.64, 0.20);
const vec3 GREEN = vec3(0.34, 1.0, 0.42);
const float PI = 3.14159265359;

float hashCRT(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}
vec3 screenCRT(vec2 uv) {
    // Extend the background padding through the curved edges. Clamp to texel
    // centers so filtering never blends with pixels outside the texture.
    vec2 halfPixel = 0.5 / max(iResolution.xy, vec2(1.0));
    return texture(iChannel0, clamp(uv, halfPixel, vec2(1.0) - halfPixel)).rgb;
}
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 res = max(iResolution.xy, vec2(1.0));
    vec2 uv = fragCoord / res;
    vec2 p = uv * 2.0 - 1.0;
    // Output-to-source mapping: sample farther out to shrink the image inward.
    // Dividing by (1 + 2 * curvature) instead zoomed in and hid the first rows.
    vec2 q = p * (1.0 + CURVATURE * dot(p, p)) / (1.0 - 2.0 * SCREEN_INSET);
    vec2 sampleUV = q * 0.5 + 0.5;
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
    if (COLOR_MODE > 0.5) {
        // Max component keeps saturated ANSI colors visible in monochrome.
        float signal = max(color.r, max(color.g, color.b));
        vec3 phosphor = COLOR_MODE > 1.5 ? GREEN : AMBER;
        color *= phosphor;
        color += phosphor * 0.001; // faintly illuminated glass
    }
    float scan = 0.5 + 0.5 * cos(2.0 * PI * sampleUV.y * res.y / SCANLINE_PX);
    color *= 1.0 - SCANLINES * scan;
    float band = exp(-pow((fract(uv.y + iTime * 0.12) - 0.5) / 0.045, 2.0));
    color *= 1.0 + ROLLING_GLOW * band;
    color *= 1.0 + FLICKER * (sin(iTime * 71.0) + 0.5 * sin(iTime * 113.0));
    color += (hashCRT(fragCoord + vec2(frame, frame * 0.71)) - 0.5) * NOISE;
    color *= 1.0 - VIGNETTE * smoothstep(0.15, 1.6, dot(p, p));
    fragColor = vec4(clamp(color * BRIGHTNESS, 0.0, 1.0), 1.0);
}
