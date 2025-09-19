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

#define rot(a) mat2(cos(a), -sin(a), sin(a), cos(a))

void main() {
    vec2 uv = v_uv;
    vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    // Parameters
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

    gl_FragColor = vec4(col, 1.0);
}