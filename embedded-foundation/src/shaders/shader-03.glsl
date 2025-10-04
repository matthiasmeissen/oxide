#version 100
precision lowp float;  // Changed from mediump for better performance

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

// Reduced iterations for Pi performance
const int MAX_MARCHING_STEPS = 48;  // Was 128
const float MIN_DIST = 0.01;  // Increased for faster convergence
const float MAX_DIST = 50.0;  // Reduced for early exit

// Refraction constants
const float IOR_RATIO = 0.733;  // Pre-calculated IOR_AIR/IOR_MATERIAL (1.1/1.5)
const vec3 ABSORPTION_COLOR = vec3(0.08);  // Simplified
const float ABSORPTION_STRENGTH = 0.4;

// Fast rotation matrices (2D where possible)
mat2 rot2(float a) {
    float s = sin(a);
    float c = cos(a);
    return mat2(c, -s, s, c);
}

mat3 rotY(float a) {
    float s = sin(a);
    float c = cos(a);
    return mat3(c, 0, s, 0, 1, 0, -s, 0, c);
}

mat3 rotZ(float a) {
    float s = sin(a);
    float c = cos(a);
    return mat3(c, -s, 0, s, c, 0, 0, 0, 1);
}

// Simplified SDF - single smooth shape instead of 3 boxes
float sceneSDF(vec3 p) {
    // Pre-calculate rotation angles outside loop when possible
    vec3 p1 = p;
    
    float t1 = u_time * mix(0.5, 1.0, u_cv1);
    float t2 = u_time * mix(-0.8, -1.5, u_cv2);
    
    p1 *= rotY(t1);
    p1 *= rotZ(t2);
    
    // Single rounded box instead of 3 with unions
    vec3 q = abs(p1) - vec3(2.0 + u_cv4, 0.5, 0.5);
    float box = length(max(q, vec3(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0) - 0.2;
    
    // Optional: Add one more simple shape for interest
    vec3 p2 = p * rotZ(u_time * mix(0.3, 0.8, u_cv3));
    vec3 q2 = abs(p2) - vec3(0.4, 2.0 + u_cv4 * 0.5, 0.4);
    float box2 = length(max(q2, vec3(0.0))) + min(max(q2.x, max(q2.y, q2.z)), 0.0) - 0.15;
    
    // Simple min instead of smooth union (cheaper)
    return min(box, box2);
}

// Cheaper normal calculation - only 4 samples instead of 6
vec3 calcNormal(vec3 p) {
    const float eps = 0.01;  // Larger epsilon, fewer artifacts on Pi
    vec2 e = vec2(eps, 0);
    float d = sceneSDF(p);
    return normalize(vec3(
        sceneSDF(p + e.xyy) - d,
        sceneSDF(p + e.yxy) - d,
        sceneSDF(p + e.yyx) - d
    ));
}

// Single ray march - removed exit marching
float rayMarch(vec3 ro, vec3 rd) {
    float t = 0.0;
    for (int i = 0; i < MAX_MARCHING_STEPS; i++) {
        float d = sceneSDF(ro + t * rd);
        
        if (d < MIN_DIST) return t;
        if (t > MAX_DIST) break;
        
        t += d * 0.9;  // Slight undershoot for safety, still faster
    }
    return MAX_DIST;
}

// Simplified background
vec3 getBackgroundColor(vec3 rd) {
    float t = rd.y * 0.5 + 0.5;
    vec3 c1 = mix(vec3(1.0, 0.31, 0.12), vec3(0.2, 0.5, 0.9), u_gate1);
    vec3 c2 = mix(vec3(0.53, 0.65, 0.71), vec3(0.9, 0.3, 0.6), u_gate2);
    return mix(c1, c2, t);
}

void main() {
    // Simplified UV calculation
    vec2 uv = (gl_FragCoord.xy * 2.0 - u_resolution.xy) / u_resolution.y;
    
    // Camera setup
    float camDist = mix(6.0, 10.0, u_gate3);
    vec3 ro = vec3(0, 0, -camDist);
    vec3 rd = normalize(vec3(uv, 1.0));  // Simplified - no camera rotation needed
    
    // Ray marching
    float t = rayMarch(ro, rd);
    
    vec3 finalColor;
    
    if (t < MAX_DIST) {
        vec3 p = ro + rd * t;
        vec3 n = calcNormal(p);
        
        // Simplified Fresnel (Schlick's approximation)
        float fresnel = pow(1.0 - abs(dot(n, rd)), 3.0);  // Cheaper power
        
        // Reflection
        vec3 reflectDir = reflect(rd, n);
        vec3 reflectedColor = getBackgroundColor(reflectDir);
        
        // Simplified refraction - fake the exit
        vec3 refractDir = refract(rd, n, IOR_RATIO);
        
        vec3 refractedColor = vec3(0.0);
        if (dot(refractDir, refractDir) > 0.0) {
            // Instead of marching again, approximate the exit
            // Assume ray travels ~4 units through object
            float approxDist = 4.0 * (1.0 + u_cv4);
            
            // Fake exit refraction by bending again
            vec3 exitDir = refractDir;  // Simplified - skip second refract
            refractedColor = getBackgroundColor(exitDir);
            
            // Absorption based on approximate distance
            float absorption = exp(-0.3 * approxDist * mix(0.5, 1.0, u_gate4));
            refractedColor *= absorption;
        }
        
        // Blend reflection and refraction
        finalColor = mix(refractedColor, reflectedColor, fresnel * 0.3 + 0.1);
        
    } else {
        finalColor = getBackgroundColor(rd);
    }
    
    gl_FragColor = vec4(finalColor, 1.0);
}