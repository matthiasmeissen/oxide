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
    float rotSpeed1 = mix(0.1, 0.5, u_cv1);
    p1 = rot(u_time * rotSpeed1) * p1;
    
    // Tile size modulation controlled by CV2 and CV3
    float tileModX = mix(0.2, 1.5, u_cv2);
    float tileModY = mix(0.2, 0.8, u_cv3);
    float timeOscillation = sin(u_time * 0.1) + 0.4;
    
    vec2 tileSize = vec2(
        fract(p.y / sin(p.x + p.y)) * tileModX,
        timeOscillation * tileModY
    );
    
    p1 = repeat(p1 * p1, tileSize);
    
    // Second rotation speed controlled by CV4
    float rotSpeed2 = mix(0.5, 2.0, u_cv4);
    p1 = rot(u_time * rotSpeed2) * p1;
    
    // Distance threshold controlled by gate3
    float d1 = length(p1);
    float threshold1 = mix(0.6, 0.95, u_gate3);
    d1 = step(threshold1, d1);
    
    // Pattern calculation with gate2 influence
    float d2 = p1.x * p1.y + p1.y;
    float threshold2 = mix(0.1, 0.4, u_gate2) + sin(p.y);
    d2 = step(threshold2, d2);
    
    // Combine patterns
    vec3 col = vec3(d1 * d2);
    
    return col;
}

void main() {
    vec2 p = (2.0 * gl_FragCoord.xy - u_resolution) / u_resolution.y;
    
    // Base pattern
    vec3 col = col1(p);
    
    // Invert colors based on gate1
    vec3 invertedCol = 1.0 - col;
    vec3 finalColor = mix(col, invertedCol, u_gate1);
    
    // Contrast adjustment with gate4
    float contrast = mix(1.0, 0.2, u_gate4);
    finalColor = (finalColor - 0.5) * contrast + 0.5;
    finalColor = clamp(finalColor, 0.0, 1.0);
    
    gl_FragColor = vec4(finalColor, 1.0);
}