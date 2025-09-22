#version 100
precision mediump float;

varying mediump vec2 v_uv;

void main() {
    vec2 uv = v_uv;
    gl_FragColor = vec4(uv.x, uv.y, 1.0, 1.0);
}