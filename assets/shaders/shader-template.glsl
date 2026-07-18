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
    vec2 p = (2.0 * gl_FragCoord.xy - u_resolution) / u_resolution.y;

    float d = length(p);
    
    vec3 finalColor = vec3(d);
    
    gl_FragColor = vec4(finalColor, 1.0);
}