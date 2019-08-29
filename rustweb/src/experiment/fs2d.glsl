varying vec2 d;
varying vec4 e;
uniform sampler2D s;

float C(vec2 p, float r) {
    return length(p) - r;
}

vec4 union_c(vec4 v1, vec4 v2) {
    //return v1[0] < v2[0] ? v1 : v2;
    return v1.w < v2.w ? v1 : v2;
}

vec4 subtract_c(vec4 v1, vec4 v2) {
    return -v1.w > v2.w ? vec4(v1.xyz, -v1.w) : v2;
}

vec4 U(vec4 v1, vec4 v2, float k) {
    float h = clamp(0.5 + 0.5*(v2.w - v1.w)/k, 0.0, 1.0);
    vec4 r = mix(v2, v1, h);
    return vec4(r.xyz, r.w - k*h*(1.0-h));
}

vec4 S(vec4 v1, vec4 v2, float k) {
    float h = clamp(0.5 - 0.5*(v2.w + v1.w)/k, 0.0, 1.0);
    vec4 r = mix(v2, vec4(v1.xyz, -v1.w), h);
    return vec4(r.xyz, r.w+k*h*(1.0-h));
}

vec4 rounded(vec4 p, float r) {
    return p - vec4(0., 0., 0., r);
}

vec4 annular(vec4 p, float r) {
    return vec4(p.xyz, abs(p.w) - r);
}

float box(vec2 p, vec2 b) {
    vec2 d;
    d = abs(p) - b;
    return length(max(d, vec2(0))) + min(max(d.x,d.y), 0.0);
}

float curvy(vec2 p) {
    return sin(2.0*p.x) * sin(2.0*p.y);
}

vec4 banana(vec2 p) {
    vec4 c = U(
        annular(
            S(
                vec4(1, 1, 0, C(p, 40.0)/* + curvy(p)*/),
                vec4(.265, .2, .06, C(p - vec2(20, 20), 30.0)),
                1.0
            ),
            2.0
        ),
        vec4(0, 0, 1, C(p - vec2(30, 30), 10.0)),
        1.0
    );
    //float alpha = clamp(0.5 - c.a, 0.0, 1.0);
    float alpha = clamp(0.5 - c.a / fwidth(c.a), 0.0, 1.0);
    return vec4(c.xyz, alpha);
}

//#define ban(p)(vec4(1.,1.,1.,.5)-S(vec4(0.,0.,1.,C(p,40.)),vec4(.735,.8,.94,C(p-vec2(20.,20.),30.)),10.))

//vec4 ban(vec2 p){return vec4(1.,1.,1.,.5)-S(vec4(0.,0.,1.,C(p,40.)),vec4(.735,.8,.94,C(p-vec2(20.,20.),30.)),10.);}

void main() {
    //gl_FragColor = vec4(texture2D(s, d).rgb, 1);
    vec2 p = (d - vec2(0.5, 0.5)) * 128.0;
    gl_FragColor = banana(p);
    //gl_FragColor = vec4(1, 1, 1, 1);
}