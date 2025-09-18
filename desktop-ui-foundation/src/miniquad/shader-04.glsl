#version 100
precision mediump float;

varying vec2 v_uv;

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_param1;
uniform float u_param2;
uniform float u_param3;
uniform float u_param4;

void main() {
    vec2 uv = v_uv;
    vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    // Parameters
    float freq1 = u_param1 * 20.0;
    float freq2 = u_param1 * 20.0;

    float d = sin(uv.x * freq1) * sin(uv.y * freq2);

    vec3 col = vec3(d);

    gl_FragColor = vec4(col, 1.0);
}