#version 100
precision mediump float;

varying vec2 v_uv;

uniform float u_time;
uniform vec2 u_resolution;
uniform sampler2D u_texture;

uniform float u_cv1;
uniform float u_cv2;
uniform float u_cv3;
uniform float u_cv4;
uniform float u_gate1;
uniform float u_gate2;
uniform float u_gate3;
uniform float u_gate4;

#define rot(a) mat2(cos(a), -sin(a), sin(a), cos(a))


float getOledGridMask(vec2 uv, vec2 resolution, float lineWidth) {
    vec2 gridUV = uv * resolution;
    vec2 pixelUV = fract(gridUV);
    vec2 lines = smoothstep(lineWidth, 0.0, pixelUV) + smoothstep(1.0 - lineWidth, 1.0, pixelUV);
    return 1.0 - (lines.x * lines.y);
}

vec3 applyCornerOverlay(vec3 baseColor, vec2 uv, vec2 u_resolution, vec2 cornerPos, float overlayWidth, float blendAmount) {
    vec2 oledResolution = vec2(128.0, 64.0);
    float textureAspectRatio = oledResolution.x / oledResolution.y;
    float screenAspectRatio = u_resolution.x / u_resolution.y;

    vec2 overlaySize;
    overlaySize.x = overlayWidth;
    overlaySize.y = (overlayWidth / textureAspectRatio) * screenAspectRatio;

    vec2 overlayMin = cornerPos * (1.0 - overlaySize);
    vec2 overlayMax = overlayMin + overlaySize;

    vec2 inside = step(overlayMin, uv) * (1.0 - step(overlayMax, uv));
    float overlayMask = inside.x * inside.y;

    if (overlayMask == 0.0) {
        return baseColor;
    }

    vec2 overlayUV = (uv - overlayMin) / overlaySize;
    overlayUV.y = 1.0 - overlayUV.y;
    vec3 texColor = texture2D(u_texture, overlayUV).rgb;

    float lineWidth = uv.x * 0.6;
    float gridMask = getOledGridMask(overlayUV, oledResolution, lineWidth);
    vec3 oledTexColor = texColor * gridMask;

    return mix(baseColor, oledTexColor, blendAmount * overlayMask);
}

void main() {
    vec2 uv = v_uv;
    vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    float freqx = u_cv1 * 40.0;
    float freqy = u_cv2 * 40.0;
    float brightness = mix(0.2, 1.4, u_cv3);
    float rotation = u_cv4 * 3.14;

    uv -= 0.5;
    uv = uv * rot(rotation);
    uv *= mix(1.0, 4.0, u_cv4);
    uv = mix(uv, uv * uv, u_cv4);
    uv += 0.5;

    float d = sin(uv.x * freqx + cos(uv.y * freqy) + u_time);

    float circle = step(length(p), 0.8);

    d = d * circle * brightness;
    d = mix(d, 1.0 - d, u_gate1);

    vec3 col = vec3(d);

    // Call the updated function with the screen resolution and a desired width for the overlay.
    col = applyCornerOverlay(col, v_uv, u_resolution, vec2(0.95, 0.05), 0.25, 1.0);

    gl_FragColor = vec4(col, 1.0);
}