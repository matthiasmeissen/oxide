#version 100
precision mediump float;

varying vec2 v_uv;

//uniform vec2 u_resolution;
uniform float u_time;
uniform float u_param1;
uniform float u_param2;
uniform float u_param3;
uniform float u_param4;

void main() {
    vec2 uv = v_uv;
    //vec2 p = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / min(u_resolution.x, u_resolution.y);

    #define rot(a) mat2(cos(a), -sin(a), sin(a), cos(a))

    // Parameters
    float freqx = u_param1 * 20.0;
    float freqy = u_param2 * 20.0;
    float speed = u_param3 * 4.0;
    float rotation = u_param4 * 3.14;

    uv = uv * rot(rotation);

    float d = sin(uv.x * freqx + cos(uv.y * freqy) + u_time * speed);

    vec3 col = vec3(d);

    gl_FragColor = vec4(col, 1.0);
}