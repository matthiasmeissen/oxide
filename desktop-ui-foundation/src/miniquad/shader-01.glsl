#version 100
precision mediump float;

varying mediump vec2 v_uv;

uniform float u_time;

void main() {
    vec2 uv = v_uv;
    vec2 p = uv - 0.5;
    float d = length(p);
    d = step(d, abs(sin(u_time)));

    vec3 col = vec3(d);

    gl_FragColor = vec4(col, 1.0);
}