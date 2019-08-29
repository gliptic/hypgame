
/*
0 1 2
3 4 5
6 7 8
*/
export function trans(mat, x, y) {
    mat[6] += mat[0] * x + mat[3] * y
    mat[7] += mat[1] * x + mat[4] * y
}

/*
2 0 0
0 2 0
0 0 1
*/
export function scale(mat, x, y) {
    mat[0] *= x
    mat[1] *= x
    mat[3] *= y
    mat[4] *= y
}

export function rotvec(mat, x, y) {
    let a = mat[0]
    let b = mat[1]
    let c = mat[3]
    let d = mat[4]

    mat[0] = a * x + c * -y
    mat[1] = b * x + d * -y
    mat[3] = a * y + c * x
    mat[4] = b * y + d * x
}

/* mat4:
    0  1  2  3
    4  5  6  7
    8  9 10 11
12 13 14 15
*/
export function scale3d(mat, x, y, z) {
    mat[0] *= x;
    mat[1] *= x;
    mat[2] *= x;

    mat[4] *= y;
    mat[5] *= y;
    mat[6] *= y;

    mat[8] *= z;
    mat[9] *= z;
    mat[10] *= z;
}

export function rotx3d(a) {
    
    var cos = Math.cos;
    var sin = Math.sin;
    
    return [
            1,       0,        0,     0,
            0,  cos(a),  -sin(a),     0,
            0,  sin(a),   cos(a),     0,
            0,       0,        0,     1
    ];
}

export function roty3d(a) {

    var cos = Math.cos;
    var sin = Math.sin;
    
    return [
        cos(a),   0, sin(a),   0,
            0,   1,      0,   0,
        -sin(a),   0, cos(a),   0,
            0,   0,      0,   1
    ];
}

export function trans3d(x, y, z) {
    return [
        1,    0,    0,   0,
        0,    1,    0,   0,
        0,    0,    1,   0,
        x,    y,    z,   1
    ];
}

export function mulmat3d(a, b) {

    // TODO - Simplify for explanation
    // currently taken from https://github.com/toji/gl-matrix/blob/master/src/gl-matrix/mat4.js#L306-L337

    var result = [];

    var a00 = a[0], a01 = a[1], a02 = a[2], a03 = a[3],
        a10 = a[4], a11 = a[5], a12 = a[6], a13 = a[7],
        a20 = a[8], a21 = a[9], a22 = a[10], a23 = a[11],
        a30 = a[12], a31 = a[13], a32 = a[14], a33 = a[15];

    // Cache only the current line of the second matrix
    var b0  = b[0], b1 = b[1], b2 = b[2], b3 = b[3];  
    result[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    result[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    result[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    result[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
    result[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    result[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    result[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    result[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
    result[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    result[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    result[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    result[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
    result[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    result[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    result[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    result[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    return result;
}

export function perspective3d(fov, ar, near, far) {
    var f = 1 / Math.tan(fov / 2);
    var rangeInv = 1 / (near - far);
    return [
        f / ar, 0, 0,                          0,
        0,      f, 0,                          0,
        0,      0, (near + far) * rangeInv,   -1,
        0,      0, near * far * rangeInv * 2,  0
    ];
}

export function vadd2([x0,y0],[x1,y1]) {
    return [x0+x1,y0+y1];
}


export function vadd3([x0,y0,z0],[x1,y1,z1]) {
    return [x0+x1,y0+y1,z0+z1];
}

export function vangle2(ang) {
    return [Math.cos(ang),Math.sin(ang)];
}

export function vscale2([x0,y0],s) {
    return [x0*s,y0*s];
}

export function vround2([x0,y0]) {
    return [Math.round(x0),Math.round(y0)];
}

export function vfloor2([x0,y0]) {
    return [Math.floor(x0),Math.floor(y0)];
}