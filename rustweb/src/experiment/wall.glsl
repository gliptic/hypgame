varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec4 fsSkyPos;
uniform sampler2D s;
uniform float time;

void main() {

    //vec4 c = mix(skyhorizon, skytop, cos(sky.y * 50.0) * cos(sky.x * 50.0));
    //gl_FragColor = vec4(1.0 + 0.5 * cos(fsSkyPos.z * 10.0), c.gb, 1);
    gl_FragColor = vec4(0, sin((fsPos.x + fsPos.y) * 0.03 + time) * 0.4 + 0.4, 0, 1);
    //gl_FragColor = vec4(1, 1, 1, 1);
    //gl_FragColor = texture2D(s, fsTexcoord)*fsColor;
    //gl_FragColor = texture2D(s, fsTexcoord);
}