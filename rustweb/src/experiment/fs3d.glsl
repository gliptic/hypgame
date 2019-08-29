varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec4 fsSkyPos;
uniform sampler2D s;

const vec4 skytop = vec4(0.0, 0.0, 0.5, 1.0);
const vec4 skyhorizon = vec4(0.3294, 0.92157, 1.0, 1.0);

float hash2(vec2 co) { return fract(sin(dot(co.xy, vec2(12.9898,78.233))) * 43758.5453); }

float starplane(vec3 dir) { 
    float screenscale = 1.0 / 700.0;

    // Project to a cube-map plane and scale with the resolution of the display
    vec2 basePos = dir.xy * (0.5 / screenscale) / max(1e-3, abs(dir.z));
         
	const float largeStarSizePixels = 20.0;
    
    // Probability that a pixel is NOT on a large star. Must change with largeStarSizePixels
	const float prob = 0.97;
    	
	float color = 0.0;
    /*
	vec2 pos = floor(basePos / largeStarSizePixels);
	float starValue = hash2(pos);
    
    
    // Big stars
	if (starValue > prob) {

        // Sphere blobs
		vec2 delta = basePos - largeStarSizePixels * (pos + vec2(0.5));
		color = max(1.0 - length(delta) / (0.5 * largeStarSizePixels), 0.0);
		
        // Star shapes
        color *= 1.0 / max(1e-3, abs(delta.x) * abs(delta.y));
        
        // Avoid triplanar seams where star distort and clump
        color *= pow(abs(dir.z), 12.0);
    } 
*/

    // Small stars

    // Stabilize stars under motion by locking to a grid
    basePos = floor(basePos);

    if (hash2(basePos.xy * screenscale) > 0.997) {
        float r = hash2(basePos.xy * 0.5);
        const float iTime = 0.0;
        color += r * (0.3 * sin(iTime * (r * 5.0) + r) + 0.7) * 1.5;
    }
	
    // Weight by the z-plane
    return color * abs(dir.z);
}

const float pi = 3.1415927;
const float deg = pi / 180.0;

mat3 rotation(float yaw, float pitch) { return mat3(cos(yaw), 0, -sin(yaw), 0, 1, 0, sin(yaw), 0, cos(yaw)) * mat3(1, 0, 0, 0, cos(pitch), sin(pitch), 0, -sin(pitch), cos(pitch)); }

float starbox(vec3 dir) {
	return starplane(dir.xyz) + starplane(dir.yzx) + starplane(dir.zxy);
}

float starfield(vec3 dir) {
    return starbox(dir) + starbox(rotation(45.0 * deg, 45.0 * deg) * dir);
}

vec3 sphereColor(vec3 dir) {
	return vec3(starfield(dir));
}

float noise3(vec2 p) {
	return sin(p.x)*sin(p.y);
}

const mat2 m = mat2(
    0.80, 0.60,
    -0.60, 0.80);

float fbm4(vec2 p) {
    float f = 0.0;
    f += 0.5000*noise3(p); p = m*p*2.02;
    f += 0.2500*noise3(p); p = m*p*2.03;
    f += 0.1250*noise3(p); p = m*p*2.01;
    f += 0.0625*noise3(p);
    return f/0.9375;
}

float planet_pattern(vec2 p) {
    //return fbm4(p.xz);

    vec2 q = vec2( fbm4( p + vec2(0.0,0.0) ),
                   fbm4( p + vec2(5.2,1.3) ) );

    vec2 r = vec2( fbm4( p + 4.0*q + vec2(1.7,9.2) ),
                   fbm4( p + 4.0*q + vec2(8.3,2.8) ) );

    return fbm4( p + 4.0*r );
}

void main() {

    vec3 rd = normalize(fsSkyPos.xyz);
    vec3 ro = vec3(0);
    vec3 center = vec3(0, 1000, 0);
    float radius = 900.0*900.0;

    float a = dot(rd,rd);
    float b = 2.0 * dot(rd, center);
    float c = dot(center, center) - radius;
    vec3 skycol;

    {
        float h = rd.y * -0.5 + 0.5;

        skycol = sphereColor(rd) +
            mix(mix(vec3(0), vec3(0.2, 0.1, 0.43), h + 0.3),
                vec3(0.08, 0.61, 0.83), h * 1.5 - 0.4);

        float d = b*b - 4.0*a*c;
        if (d > 0.0) {
            float q = -0.5 * (b + (b > 0.0 ? sqrt(d) : -sqrt(d)));
            float t = min(q / a, c / q);
            if (t > 0.0) {
                vec3 intersection = rd * t;
                skycol = vec3(0.1, 0.2, 0.4) + vec3(0.0, 0.1, 0.4) * planet_pattern(intersection.xz * 0.05);
            }
        }
    }

    //vec4 c = mix(skyhorizon, skytop, cos(sky.y * 50.0) * cos(sky.x * 50.0));
    //gl_FragColor = vec4(1.0 + 0.5 * cos(fsSkyPos.z * 10.0), c.gb, 1);
    gl_FragColor = vec4(skycol, 1);
    //gl_FragColor = vec4(1, 1, 1, 1);
    //gl_FragColor = texture2D(s, fsTexcoord)*fsColor;
    //gl_FragColor = texture2D(s, fsTexcoord);
}