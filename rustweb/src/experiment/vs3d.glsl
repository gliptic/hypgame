attribute vec3 pos;
attribute vec2 texcoord;
attribute vec4 color;

uniform mat4 modelView;
uniform vec3 viewPos;
uniform float invAr;

varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec4 fsSkyPos;

mat4 perspective3d() {
    /*
    float near = 1.0;
    float far = 30000.0;
    float fov = 1.0;
    float f = 1.0 / tan(fov / 2.0);
    float rangeInv = 1.0 / (near - far);
    return mat4(
        f / ar, 0, 0,                          0,
        0,      f, 0,                          0,
        0,      0, (near + far) * rangeInv,   -1,
        0,      0, near * far * rangeInv * 2.0,  0
    );
    */

    // float f = 1.83;
    // float rangeinv = -.00003333;
    // (near + far) * rangeInv = -1.000066668888963
    // near * far * rangeInv * 2.0 = -2.000066668888963
    return mat4(
        1.83 /* * invAr*/, 0, 0,  0,
        0,      1.83, 0,  0,
        0,      0,   -1, -1,
        0,      0,   -2,  0
    );
}

void main() {
    fsTexcoord = texcoord;
    fsPos = vec4(pos, 1);
    fsSkyPos = vec4(pos + viewPos, 1);
    //gl_Position = projection * modelView * vec4(pos, 1);

    /*
    mat4 something = mat4(
        invAr, 0, 0, 0,
        0,     1, 0, 0,
        0,     0, 1, 0,
        0,     0, 0, 1
    );

    // -> invAr * x, y, z, w
    // 
    */

    vec4 v = perspective3d() * modelView * vec4(pos, 1);
    v.x *= invAr;

    gl_Position = v;
    
    //gl_Position = modelView * vec4(pos, 1);
    //gl_Position = vec4(pos, 1);
}