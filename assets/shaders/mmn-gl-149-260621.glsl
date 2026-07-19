#version 100
precision mediump float;

varying vec2 v_uv;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_cv1;
uniform float u_cv2;
uniform float u_cv3;
uniform float u_cv4;
uniform float u_gate1;
uniform float u_gate2;
uniform float u_gate3;
uniform float u_gate4;

float sdCircle(vec2 p, float r) {
    return length(p) - r;
}

float sdBox(vec2 p, vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float sdRoundedBoxUniform(vec2 p, vec2 b, float r) {
    vec2 q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r;
}

float smin(float a, float b, float k4, float inv) {
    float h = max(k4 - abs(a - b), 0.0);
    return min(a, b) - h * h * inv;
}

float column(vec2 p, float blendK4, float blendInv) {
    float d = sdRoundedBoxUniform(p, vec2(0.1, 0.25), 0.04);
    d = smin(d, sdBox(p - vec2(0.2,  0.45), vec2(0.1, 0.2)), blendK4, blendInv);
    d = smin(d, sdBox(p - vec2(0.2, -0.45), vec2(0.1, 0.2)), blendK4, blendInv);
    d = smin(d, sdBox(p - vec2(0.4,  0.0),  vec2(0.3, 0.1)), 0.04, 6.25);
    d = smin(d, sdCircle(p - vec2(0.4, -0.75), 0.1), blendK4, blendInv);
    d = smin(d, sdCircle(p - vec2(0.4,  0.75), 0.1), blendK4, blendInv);
    return d;
}

void main() {
    vec2 p = v_uv - 0.5;
    p.x *= u_resolution.x / u_resolution.y;

    p.x = mix(p.x, abs(p.x), u_gate2);
    p.y = mix(p.y, abs(p.y), u_gate3);

    p *= mix(8.0, 32.0, u_gate4);

    float rotScale = mix(-0.2, 0.2, u_cv2);
    float blend    = mix(0.03, 0.1, u_cv4);
    float blendK4  = blend * 4.0;
    float blendInv = 0.25 / blendK4;

    const int STEPS = 6;

    float d = 1.0;

    for (int i = 0; i < STEPS; i++) {
        float a = p.y * rotScale;
        float s = sin(a);
        float c = cos(a);
        p *= mat2(c, -s, s, c);

        p.x *= mix(1.0, p.y, u_cv3);

        vec2 off = vec2(float(i) * 0.4 + 0.4, 0.0);
        d = min(d, min(column(p + off, blendK4, blendInv),
                       column(off - p, blendK4, blendInv)));
    }

    float fill    = step(d, 0.0);
    float outline = step(d, 0.01) - fill;
    float v = mix(fill, outline, u_cv1);

    v = mix(v, 1.0 - v, u_gate1);

    gl_FragColor = vec4(vec3(v), 1.0);
}
