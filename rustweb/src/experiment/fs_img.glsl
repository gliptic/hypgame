varying vec2 d;
varying vec4 e;
uniform sampler2D s;

void main() {
    gl_FragColor = texture2D(s, d) * e;
}