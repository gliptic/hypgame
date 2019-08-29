varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec4 fsSkyPos;
uniform sampler2D s;


float sdRoundedCylinder( vec3 p, float ra, float rb, float h ) {
    vec2 d = vec2( length(p.xz)-2.0*ra+rb, abs(p.y) - h );
    return min(max(d.x,d.y),0.0) + length(max(d,0.0)) - rb;
}

float sdSphere( vec3 p, float s )
{
    return length(p) - s;
}

float map(vec3 pos) {
    return sdRoundedCylinder(pos.xzy + vec3(0, 5, 0), 0.1, 0.1, 0.1);
}

void main() {
    vec3 rd = normalize(fsSkyPos.xyz);

    vec3 ro = vec3(0);
    float t = 1.0;
    const float tmax = 20.0;
    float res = -1.0;

    for (int i = 0; i < 20; i++) {
        float h = map(ro + rd * t);
        if (abs(h) < 0.0001 * t) {
            res = t;
            break;
        }
        t += h;
        if (t >= tmax) {
            break;
        }
    }

    gl_FragColor = res >= 0.0 ? vec4(1, 0, 0, 1.0) : vec4(0, 0, 0, 1);
}