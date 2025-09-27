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

#define PI 3.14159265359
#define rot(a) mat2(cos(a), -sin(a), sin(a), cos(a))

void main() {
    vec2 uv = v_uv;
    vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    float size = mix(0.4, 2.0, u_cv1);
    float s1 = mix(2.0, 10.0, u_cv2);
    float s2 = mix(4.0, 20.0, u_cv3);

    vec2 p1 = p + size;
    p1 = rot(u_time * 0.4) * p1;

    vec2 p2 = rot(p1.x) * p;

    float d = distance(p, vec2(cos(p2.x * s1), p.y)) * distance(p1, vec2(p.x, sin(p.y * s2)));

    d = step(u_cv4, d);

    d = mix(d, 1.0 - d, u_gate1);

    vec3 col = vec3(d);

    gl_FragColor = vec4(col, 1.0);
}