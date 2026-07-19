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

vec2 repeat(vec2 p, vec2 tileSize) {
    // Creates a repeating pattern of tiles with size tileSize
    vec2 p1 = mod(p, tileSize) - tileSize * 0.5;
    return p1 / (tileSize * 0.5);
}

vec3 col1(vec2 p) {
    vec2 p1 = p;
    
    // Rotation speed controlled by CV1
    float s1 = mix(2.0, 0.4, u_cv1);
    p1 *= s1;
    p1.x -= u_cv1;
    p1 = rot(u_time * 0.2) * p1;
    p1.x += u_cv1;

    float tileX = mix(fract(p.y / sin(p.x + p.y)), sin(p1.y), u_cv2);
    float tileY = mix(p.x, abs(sin(p.y * p1.x)), u_cv3);
    
    p1 = repeat(p1 * p1, vec2(tileX, tileY));

    float offsetX = mix(-1.0, 1.0, u_cv4);
    p1.x += offsetX;
    
    // Second rotation speed controlled by CV4
    p1 = rot(u_time * 0.4) * p1;
    
    // Distance threshold controlled by gate2
    float d1 = length(p1);
    float threshold1 = mix(0.6, 0.95, u_gate2);
    d1 = step(threshold1, d1);
    
    // Combine patterns
    vec3 col = vec3(d1);
    
    return col;
}

void main() {
    vec2 p = (2.0 * gl_FragCoord.xy - u_resolution) / u_resolution.y;

    p.x = mix(p.x, abs(p.x), u_gate3);
    p.y = mix(p.y, abs(p.y), u_gate4);

    // Base pattern
    vec3 col = col1(p);
    
    // Invert colors based on gate1
    vec3 invertedCol = 1.0 - col;
    vec3 finalColor = mix(col, invertedCol, u_gate1);
    
    finalColor = clamp(finalColor, 0.0, 1.0);
    
    gl_FragColor = vec4(finalColor, 1.0);
}