attribute vec3 pos;
attribute vec2 texcoord;
attribute vec4 color;

uniform mat4 projection;
uniform mat4 modelView;
uniform vec3 viewPos;

varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec4 fsSkyPos;

void main() {
    fsTexcoord = texcoord;
    fsPos = vec4(pos, 1);
    fsSkyPos = vec4(pos + viewPos, 1);
    gl_Position = projection * modelView * vec4(pos, 1);
    //gl_Position = modelView * vec4(pos, 1);
    //gl_Position = vec4(pos, 1);
}