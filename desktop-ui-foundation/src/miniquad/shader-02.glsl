#version 100
precision mediump float;

varying vec2 v_uv;

uniform vec2 u_resolution;
uniform float u_time;
uniform vec4 u_params;

void main() {
    vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    p *= 0.02;

    float color = 0.0;
    color += sin(p.x * cos(u_time / 15.0) * 8.0 + u_params.x * 10.0) + cos(p.y * cos(u_time / 10.0) * 10.0 + u_params.y * 10.0);
    color += sin(p.y * sin(u_time / 5.0) * 4.0 + u_params.z * 10.0) + cos(p.x * sin(u_time / 25.0) * 4.0 + u_params.w * 10.0);
    color *= sin(u_time / 10.0) * 0.5 + 0.5;

    gl_FragColor = vec4(vec3(color, color * 0.5, sin(color + u_time / 3.0) * 0.75), 1.0);
}