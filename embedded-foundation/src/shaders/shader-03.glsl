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

// Ray marching constants
const int MAX_MARCHING_STEPS = 32;
const float MIN_DIST = 0.001;
const float MAX_DIST = 1000.0;

// Refraction constants
const float IOR_MATERIAL = 1.5;
const float IOR_AIR = 1.1;
const vec3 ABSORPTION_COLOR = vec3(0.08, 0.08, 0.08);
const float ABSORPTION_STRENGTH = 0.4;
const int MAX_MARCHING_STEPS_INTERNAL = 64;
const float MAX_DIST_INTERNAL = 50.0;

// SDF helper functions
float vmax(vec3 v) {
    return max(max(v.x, v.y), v.z);
}

mat3 rotationY(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return mat3(c, 0, s, 0, 1, 0, -s, 0, c);
}

mat3 rotationX(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return mat3(1, 0, 0, 0, c, -s, 0, s, c);
}

mat3 rotationZ(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return mat3(c, -s, 0, s, c, 0, 0, 0, 1);
}

float fBoxRound(vec3 p, vec3 b, float r) {
    vec3 q = abs(p) - b;
    return length(max(q, vec3(0.0))) + min(vmax(q), 0.0) - r;
}

float fOpUnionRound(float a, float b, float r) {
    vec2 u = max(vec2(r - a, r - b), vec2(0));
    return max(r, min(a, b)) - length(u);
}

// Scene definition - controlled by CV parameters
float sceneSDF(vec3 p) {
    float dist = MAX_DIST;
    
    vec3 p1 = p;
    
    // Rotation speeds controlled by CV1 and CV2
    float rotSpeed1 = mix(10.0, 30.0, u_cv1);
    float rotSpeed2 = mix(-20.0, -50.0, u_cv2);
    float rotSpeed3 = mix(15.0, 35.0, u_cv3);
    
    p1 *= rotationY(radians(u_time * rotSpeed1));
    p1 *= rotationZ(radians(u_time * rotSpeed2 * 0.5));
    
    // Box sizes influenced by CV4
    float boxScale = mix(2.0, 3.5, u_cv4);
    
    float box = fBoxRound(p1, vec3(boxScale, 0.4, abs(p.x) * 0.1), 0.2);
    dist = fOpUnionRound(dist, box, 0.2);
    
    p1 *= rotationZ(radians(u_time * rotSpeed2));
    float box2 = fBoxRound(p1, vec3(boxScale, abs(p.y) * 0.4, 0.4), 0.2);
    dist = fOpUnionRound(dist, box2, 0.1);
    
    p1 *= rotationY(radians(u_time * rotSpeed3));
    float box3 = fBoxRound(p1, vec3(abs(p.y) * 1.4, 0.8, 0.3), 0.2);
    dist = fOpUnionRound(dist, box3, 0.1);
    
    return dist;
}

vec3 calcNormal(vec3 p) {
    const float eps = 0.001;
    vec2 h = vec2(eps, 0);
    return normalize(vec3(
        sceneSDF(p + h.xyy) - sceneSDF(p - h.xyy),
        sceneSDF(p + h.yxy) - sceneSDF(p - h.yxy),
        sceneSDF(p + h.yyx) - sceneSDF(p - h.yyx)
    ));
}

float rayMarch(vec3 ro, vec3 rd, out vec3 pHit) {
    float t = 0.0;
    for (int i = 0; i < MAX_MARCHING_STEPS; i++) {
        vec3 p = ro + t * rd;
        float d = sceneSDF(p);
        
        if (abs(d) < MIN_DIST) {
            pHit = p;
            return t;
        }
        if (t > MAX_DIST || d > MAX_DIST) {
            break;
        }
        t += d;
    }
    pHit = ro + t * rd;
    return MAX_DIST;
}

float rayMarchExit(vec3 ro, vec3 rd, out vec3 pExit) {
    float t = MIN_DIST;
    for (int i = 0; i < MAX_MARCHING_STEPS_INTERNAL; ++i) {
        vec3 p = ro + t * rd;
        float sdfVal = sceneSDF(p);
        
        if (sdfVal > -MIN_DIST && t > MIN_DIST * 1.5) {
            pExit = p - rd * sdfVal;
            return t;
        }
        if (t > MAX_DIST_INTERNAL) {
            pExit = p;
            return MAX_DIST_INTERNAL;
        }
        t += max(MIN_DIST, abs(sdfVal));
    }
    pExit = ro + t * rd;
    return MAX_DIST_INTERNAL;
}

vec3 getBackgroundColor(vec3 rayDir) {
    float t = 0.5 * (normalize(rayDir).y + 1.0);
    
    // Background colors influenced by gates
    vec3 c1 = mix(vec3(1.0, 0.31, 0.12), vec3(0.2, 0.5, 0.9), u_gate1);
    vec3 c2 = mix(vec3(0.53, 0.65, 0.71), vec3(0.9, 0.3, 0.6), u_gate2);
    
    return mix(c1, c2, t);
}

void main() {
    vec2 uv = (2.0 * gl_FragCoord.xy - u_resolution.xy) / u_resolution.y;
    
    // Camera setup - distance controlled by gate3
    float camDistance = mix(-4.0, -8.0, u_gate3);
    vec3 ro = vec3(0.0, 0.0, camDistance);
    
    vec3 lookAt = vec3(0.0, 0.0, 0.0);
    float fov = 1.0;
    
    vec3 camForward = normalize(lookAt - ro);
    vec3 camRight = normalize(cross(vec3(0.0, 1.0, 0.0), camForward));
    vec3 camUp = normalize(cross(camForward, camRight));
    
    vec3 rd = normalize(uv.x * camRight + uv.y * camUp + fov * camForward);
    
    // Ray marching & refraction
    vec3 pHit_entry;
    float t_entry = rayMarch(ro, rd, pHit_entry);
    
    vec3 finalColor;
    
    if (t_entry < MAX_DIST) {
        vec3 normal_entry = calcNormal(pHit_entry);
        
        // Fresnel calculation
        float NdotV = abs(dot(normal_entry, rd));
        float R0 = pow((IOR_AIR - IOR_MATERIAL) / (IOR_AIR + IOR_MATERIAL), 2.0);
        float fresnel = R0 + (1.0 - R0) * pow(1.0 - NdotV, 5.0);
        
        // Reflected component
        vec3 reflectedColor = vec3(0.0);
        if (fresnel > 0.001) {
            vec3 reflectDir = reflect(rd, normal_entry);
            reflectedColor = getBackgroundColor(reflectDir);
        }
        
        // Refracted component
        vec3 refractedColor = vec3(0.0);
        float eta_enter = IOR_AIR / IOR_MATERIAL;
        vec3 refractDir_enter = refract(rd, normal_entry, eta_enter);
        
        if (dot(refractDir_enter, refractDir_enter) > 0.0) {
            vec3 ro_internal = pHit_entry + refractDir_enter * MIN_DIST * 2.0;
            
            vec3 pHit_exit;
            float t_internal = rayMarchExit(ro_internal, refractDir_enter, pHit_exit);
            
            if (t_internal < MAX_DIST_INTERNAL) {
                vec3 normal_exit = calcNormal(pHit_exit);
                float eta_exit = IOR_MATERIAL / IOR_AIR;
                vec3 refractDir_exit = refract(refractDir_enter, normal_exit, eta_exit);
                
                if (dot(refractDir_exit, refractDir_exit) > 0.0) {
                    refractedColor = getBackgroundColor(refractDir_exit);
                    
                    // Beer's Law absorption - strength controlled by gate4
                    float absorptionMod = mix(0.2, 0.8, u_gate4);
                    float dist_travelled_inside = length(pHit_exit - pHit_entry);
                    vec3 absorption = exp(-ABSORPTION_COLOR * ABSORPTION_STRENGTH * absorptionMod * dist_travelled_inside);
                    refractedColor *= absorption;
                    
                } else {
                    vec3 reflectDir_internal = reflect(refractDir_enter, normal_exit);
                    refractedColor = getBackgroundColor(reflectDir_internal) * 0.3;
                }
            } else {
                refractedColor = getBackgroundColor(refractDir_enter) * 0.1;
            }
        }
        
        finalColor = mix(refractedColor, reflectedColor, fresnel);
        
    } else {
        finalColor = getBackgroundColor(rd);
    }
    
    gl_FragColor = vec4(finalColor, 1.0);
}