varying vec2 d;
varying vec4 e;
uniform sampler2D s;

void main() {
    gl_FragColor = vec4(texture2D(s, d).rgb, 1) * e;
}