(function(window) {

/*
0 1 2
3 4 5
6 7 8
*/
function trans(mat, x, y) {
    mat[6] += mat[0] * x + mat[3] * y
    mat[7] += mat[1] * x + mat[4] * y
}

/*
2 0 0
0 2 0
0 0 1
*/
function scale(mat, x, y) {
    mat[0] *= x
    mat[1] *= x
    mat[3] *= y
    mat[4] *= y
}

function rotvec(mat, x, y) {
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
function scale3d(mat, x, y, z) {
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

function rotx3d(a) {
  
    var cos = Math.cos;
    var sin = Math.sin;
    
    return [
         1,       0,        0,     0,
         0,  cos(a),  -sin(a),     0,
         0,  sin(a),   cos(a),     0,
         0,       0,        0,     1
    ];
}

function roty3d(a) {

    var cos = Math.cos;
    var sin = Math.sin;
    
    return [
       cos(a),   0, sin(a),   0,
            0,   1,      0,   0,
      -sin(a),   0, cos(a),   0,
            0,   0,      0,   1
    ];
}

function trans3d(x, y, z) {
	return [
	    1,    0,    0,   0,
	    0,    1,    0,   0,
	    0,    0,    1,   0,
	    x,    y,    z,   1
	];
}

function mulmat3d(a, b) {

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

function perspective3d(fov, ar, near, far) {
    var f = 1 / Math.tan(fov / 2);
    var rangeInv = 1 / (near - far);
    return [
        f / ar, 0, 0,                          0,
        0,      f, 0,                          0,
        0,      0, (near + far) * rangeInv,   -1,
        0,      0, near * far * rangeInv * 2,  0
    ];
}

let gl
let render_w
let render_h
let viewTrans
let viewTrans3d

let GL_VERTEX_SHADER = 0x8B31
let GL_FRAGMENT_SHADER = 0x8B30
let GL_ELEMENT_ARRAY_BUFFER = 0x8893
let GL_ARRAY_BUFFER = 0x8892
let GL_TEXTURE0 = 0x84C0

let GL_SRC_ALPHA =           0x0302
let GL_ONE_MINUS_SRC_ALPHA = 0x0303
let GL_DST_ALPHA =           0x0304
let GL_ONE_MINUS_DST_ALPHA = 0x0305
let GL_DST_COLOR =           0x0306
let GL_ONE_MINUS_DST_COLOR = 0x0307
let GL_BLEND = 0x0BE2
let GL_RGBA = 0x1908
let GL_LUMINANCE = 0x1909

let GL_TRIANGLES = 0x0004
let GL_TRIANGLE_STRIP = 0x0005
let GL_UNSIGNED_BYTE = 0x1401
let GL_UNSIGNED_SHORT = 0x1403
let GL_FLOAT = 0x1406
let GL_STATIC_DRAW = 0x88E4
let GL_DYNAMIC_DRAW = 0x88E8
let GL_COMPILE_STATUS = 0x8B81
let GL_LINK_STATUS = 0x8B82
let GL_TEXTURE_2D = 0x0DE1
let GL_TEXTURE_WRAP_S = 0x2802
let GL_TEXTURE_WRAP_T = 0x2803
let GL_TEXTURE_MAG_FILTER = 0x2800
let GL_TEXTURE_MIN_FILTER = 0x2801
let GL_NEAREST = 0x2600
let GL_LINEAR = 0x2601
let GL_CLAMP_TO_EDGE = 0x812F
let GL_COLOR_BUFFER_BIT = 0x00004000

let basicVs = `
attribute vec2 a, b;
attribute vec4 c;
varying vec2 d,f;
varying vec4 e;
uniform mat3 m;
void main() {
    d=b;
    e=c;
    f=a;
    gl_Position=vec4(m*vec3(a,1.),1.);
}`;

let vs3d = `
attribute vec3 pos;
attribute vec2 texcoord;
attribute vec4 color;

uniform mat4 projection;
uniform mat4 modelView;

varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec3 fsSkyPos;

void main() {
    fsTexcoord = texcoord;
    fsPos = vec4(pos, 1);
    fsSkyPos = normalize(pos);
    gl_Position = projection * modelView * vec4(pos, 1);
    //gl_Position = modelView * vec4(pos, 1);
    //gl_Position = vec4(pos, 1);
    
}`;

let VERTEX_SIZE = (4 * 2) + (4 * 2) + (4)
let VERTEX_SIZE3D = (4 * 3) + (4 * 2) + (4)
let MAX_BATCH = 10922
let MAX_STACK = 100
let MAT_SIZE = 6
let VERTICES_PER_QUAD = 6
let QUAD_SIZE_IN_WORDS = (VERTEX_SIZE * VERTICES_PER_QUAD) / 4
let QUAD_SIZE_IN_WORDS3D = (VERTEX_SIZE3D * VERTICES_PER_QUAD) / 4
let VERTEX_DATA_SIZE = VERTEX_SIZE * MAX_BATCH * 6
let VERTEX_DATA_SIZE3D = VERTEX_SIZE3D * MAX_BATCH * 6

let VBO, VBO3D
let count = 0
let currentTexture
let vertexData = new ArrayBuffer(VERTEX_DATA_SIZE)
let vPositionData = new Float32Array(vertexData)
let vColorData = new Uint32Array(vertexData)

let vertexData3d = new ArrayBuffer(VERTEX_DATA_SIZE3D)
let vPositionData3d = new Float32Array(vertexData3d)
let vColorData3d = new Uint32Array(vertexData3d)
let mat0 = 1
let mat1 = 0
let mat2 = 0
let mat3 = 1
let mat4 = 0
let mat5 = 0
let stack = []
let locPos = 0
let locUV = 1
let locColor = 2
let col = 0xffffffff

function initGl(canvas) {
    gl = canvas.getContext("webgl")
    //gl.clearColor(0.0, 0.2, 0.0, 1.0)
    gl.clearColor(0.2, 0.25, 0.4, 1.0)
}

function createBuffer(bufferType, size, usage) {
    let buffer = gl.createBuffer()
    gl.bindBuffer(bufferType, buffer)
    gl.bufferData(bufferType, size, usage)
    return buffer
}

function createTexture(image, side, ty) {
    let texture = gl.createTexture()
    checkErr(gl.bindTexture(GL_TEXTURE_2D, texture))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR))
    checkErr(gl.texImage2D(GL_TEXTURE_2D, 0, ty, side, side, 0, ty, GL_UNSIGNED_BYTE, new Uint8Array(image.buffer)))
    console.log("texImage")
    return texture
}

function updateTexture(texture, image, side, ty) {
    gl.bindTexture(GL_TEXTURE_2D, texture)
    gl.texSubImage2D(GL_TEXTURE_2D, 0, 0, 0, side, side, ty, GL_UNSIGNED_BYTE, new Uint8Array(image.buffer))
    return texture
}

function setViewTransform(shader) {
    gl.uniformMatrix3fv(gl.getUniformLocation(shader, "m"), 0, viewTrans)
}

function setViewTransform3d(shader) {
    gl.uniformMatrix4fv(gl.getUniformLocation(shader, "modelView"), 0, viewTrans3d)

    //var p = perspective3d(1.58, render_w / render_h, 1, 3000);
    var p = perspective3d(1, render_w / render_h, 1, 30000);
    gl.uniformMatrix4fv(gl.getUniformLocation(shader, "projection"), 0, p)
    
}

function color(c) {
    col = c
}

function setView(x, y, rotx, roty, zoom) {
    let ratio = render_h / render_w

    viewTrans = [
        1, 0, 0,
        0, 1, 0,
        0, 0, 1
    ]

    scale(viewTrans, 2 / (1024 * zoom), 2 / (1024 * zoom * ratio))
    rotvec(viewTrans, rotx, roty)
    trans(viewTrans, -x, -y)
}

function setView3d(roty, rotx, tx, ty, zoom) {

    /*
    viewTrans3d = [
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    ]*/

    viewTrans3d = mulmat3d(mulmat3d(rotx3d(rotx), roty3d(roty)), trans3d(tx, 0, ty));

    scale3d(viewTrans3d, 1 / zoom, 1 / zoom, 1 / zoom);
}

function flush() {
    if (count) {
        //console.log(vPositionData.subarray(0, count * QUAD_SIZE_IN_WORDS));
        checkErr(gl.bindBuffer(GL_ARRAY_BUFFER, VBO))
        checkErr(gl.vertexAttribPointer(locPos, 2, GL_FLOAT, 0, VERTEX_SIZE, 0))
        checkErr(gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE, 8))
        checkErr(gl.vertexAttribPointer(locColor, 4, GL_UNSIGNED_BYTE, 1, VERTEX_SIZE, 16))
        checkErr(gl.bufferSubData(GL_ARRAY_BUFFER, 0, vPositionData.subarray(0, count * QUAD_SIZE_IN_WORDS)))
        checkErr(gl.drawArrays(GL_TRIANGLES, 0, count * VERTICES_PER_QUAD))
        count = 0
    }
    currentTexture = null
}

function flush3d() {
    if (count) {
        gl.bindBuffer(GL_ARRAY_BUFFER, VBO3D)
        checkErr(gl.vertexAttribPointer(locPos, 3, GL_FLOAT, 0, VERTEX_SIZE3D, 0))
        checkErr(gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE3D, 12))
        checkErr(gl.vertexAttribPointer(locColor, 4, GL_UNSIGNED_BYTE, 1, VERTEX_SIZE3D, 20))
        gl.bufferSubData(GL_ARRAY_BUFFER, 0, vPositionData3d.subarray(0, count * QUAD_SIZE_IN_WORDS3D))
        gl.drawArrays(GL_TRIANGLES, 0, count * VERTICES_PER_QUAD)
        count = 0
    }
    currentTexture = null
}

function img3d(texture, x, y, z, w_, h_, u0, v0, u1, v1) {
    let x0 = x
    let y0 = y
    let z0 = z
    let x1 = x + w_
    let y1 = y + h_
    let z1 = z
    let x2 = x
    let y2 = y + h_
    let z2 = z;
    let x3 = x + w_
    let y3 = y
    let z3 = z
    let offset = 0
    let argb = col

    if (texture != currentTexture || count + 1 >= MAX_BATCH) {
        flush3d()
        if (currentTexture != texture) {
            currentTexture = texture
            gl.bindTexture(GL_TEXTURE_2D, currentTexture)
        }
    }

    offset = count * QUAD_SIZE_IN_WORDS3D - 1
    // Vertex Order
    // Vertex Position | UV | ARGB
    // Vertex 1
    vPositionData3d[++offset] = x0
    vPositionData3d[++offset] = y0
    vPositionData3d[++offset] = z0;
    vPositionData3d[++offset] = u0
    vPositionData3d[++offset] = v0
    vColorData3d[++offset] = argb

    // Vertex 4
    vPositionData3d[++offset] = x3
    vPositionData3d[++offset] = y3
    vPositionData3d[++offset] = z3;
    vPositionData3d[++offset] = u1
    vPositionData3d[++offset] = v0
    vColorData3d[++offset] = argb
    
    // Vertex 2
    vPositionData3d[++offset] = x1
    vPositionData3d[++offset] = y1
    vPositionData3d[++offset] = z1;
    vPositionData3d[++offset] = u1
    vPositionData3d[++offset] = v1
    vColorData3d[++offset] = argb

    // Vertex 1
    vPositionData3d[++offset] = x0
    vPositionData3d[++offset] = y0
    vPositionData3d[++offset] = z0;
    vPositionData3d[++offset] = u0
    vPositionData3d[++offset] = v0
    vColorData3d[++offset] = argb

    // Vertex 2
    vPositionData3d[++offset] = x1
    vPositionData3d[++offset] = y1
    vPositionData3d[++offset] = z1;
    vPositionData3d[++offset] = u1
    vPositionData3d[++offset] = v1
    vColorData3d[++offset] = argb
    
    // Vertex 3
    vPositionData3d[++offset] = x2
    vPositionData3d[++offset] = y2
    vPositionData3d[++offset] = z2;
    vPositionData3d[++offset] = u0
    vPositionData3d[++offset] = v1
    vColorData3d[++offset] = argb
    
    if (++count >= MAX_BATCH) {
        flush3d()
    }
}

function img(texture, x, y, w_, h_, u0, v0, u1, v1) {
    let x0 = x
    let y0 = y
    let x1 = x + w_
    let y1 = y + h_
    let x2 = x
    let y2 = y + h_
    let x3 = x + w_
    let y3 = y
    let offset = 0
    let argb = col

    if (texture != currentTexture || count + 1 >= MAX_BATCH) {
        flush()
        if (currentTexture != texture) {
            currentTexture = texture
            gl.bindTexture(GL_TEXTURE_2D, currentTexture)
        }
    }

    offset = count * QUAD_SIZE_IN_WORDS - 1
    // Vertex Order
    // Vertex Position | UV | ARGB
    // Vertex 1
    vPositionData[++offset] = x0
    vPositionData[++offset] = y0
    vPositionData[++offset] = u0
    vPositionData[++offset] = v0
    vColorData[++offset] = argb

    // Vertex 4
    vPositionData[++offset] = x3
    vPositionData[++offset] = y3
    vPositionData[++offset] = u1
    vPositionData[++offset] = v0
    vColorData[++offset] = argb
    
    // Vertex 2
    vPositionData[++offset] = x1
    vPositionData[++offset] = y1
    vPositionData[++offset] = u1
    vPositionData[++offset] = v1
    vColorData[++offset] = argb

    // Vertex 1
    vPositionData[++offset] = x0
    vPositionData[++offset] = y0
    vPositionData[++offset] = u0
    vPositionData[++offset] = v0
    vColorData[++offset] = argb

    // Vertex 2
    vPositionData[++offset] = x1
    vPositionData[++offset] = y1
    vPositionData[++offset] = u1
    vPositionData[++offset] = v1
    vColorData[++offset] = argb
    
    // Vertex 3
    vPositionData[++offset] = x2
    vPositionData[++offset] = y2
    vPositionData[++offset] = u0
    vPositionData[++offset] = v1
    vColorData[++offset] = argb
    
    if (++count >= MAX_BATCH) {
        flush()
    }
}

function activateShader(shader) {
    gl.useProgram(shader)
    setViewTransform(shader)
}

function activateShader3d(shader) {
    gl.useProgram(shader)
    setViewTransform3d(shader)
    gl.uniform1i(gl.getUniformLocation(shader, 's'), 0);
}

function compileShader(source, ty) {
    let shader = gl.createShader(ty)
    gl.shaderSource(shader, "#extension GL_OES_standard_derivatives:enable\nprecision lowp float;" + source)
    gl.compileShader(shader)

    if (!gl.getShaderParameter(shader, GL_COMPILE_STATUS)) {
        //console.log(`Error compiling ${ty === GL_VERTEX_SHADER ? "vertex" : "fragment"} shader:`)
        console.log(gl.getShaderInfoLog(shader))
    }
    return shader
}

function bindAttribLocations(shader) {
    ["a", "b", "c"].map((name, i) => gl.bindAttribLocation(shader, i, name))
}

function bindAttribLocations3d(shader) {
    ["pos", "texcoord", "color"].map((name, i) => gl.bindAttribLocation(shader, i, name))
}

function createShaderProgram(vsSource, fsSource, is3d) {
    let program = gl.createProgram()
    let vShader = compileShader(vsSource, GL_VERTEX_SHADER)
    let fShader = compileShader(fsSource, GL_FRAGMENT_SHADER)
    gl.attachShader(program, vShader)
    gl.attachShader(program, fShader)
    gl.linkProgram(program)

    if (!gl.getProgramParameter(program, GL_LINK_STATUS)) {
        console.log("Error linking shader program:")
        console.log(gl.getProgramInfoLog(program))
    }

    if (is3d) {
        bindAttribLocations3d(program)
    } else {
        bindAttribLocations(program)
    }
    return program
}

function checkErr(v) {
    let err = gl.getError()
    if (err != 0) {
        for (var k in gl) {
            if (gl[k] == err) {
                console.log("error key:", k);
            }
        }
        console.log("error:", err)
        console.trace()
    }
    return v
}

function render_init(canvas) {
    initGl(canvas)
    canvas.onclick = function() {
        canvas.requestPointerLock();
    };
    //gl.blendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
    gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.enable(GL_BLEND)

    gl.getExtension("OES_standard_derivatives")
    
    VBO = createBuffer(GL_ARRAY_BUFFER, VERTEX_DATA_SIZE, GL_DYNAMIC_DRAW)
    //gl.bindBuffer(GL_ARRAY_BUFFER, VBO)
    
    checkErr(gl.enableVertexAttribArray(locPos))
    checkErr(gl.enableVertexAttribArray(locUV))
    checkErr(gl.enableVertexAttribArray(locColor))

    VBO3D = createBuffer(GL_ARRAY_BUFFER, VERTEX_DATA_SIZE3D, GL_DYNAMIC_DRAW)
    //gl.bindBuffer(GL_ARRAY_BUFFER, VBO3D)
    
    //checkErr(gl.enableVertexAttribArray(locPos))
    //checkErr(gl.enableVertexAttribArray(locUV))
    //checkErr(gl.enableVertexAttribArray(locColor))

    gl.activeTexture(GL_TEXTURE0)
}

let uniLocs;
let uniNames = ["t", "P", "R"];

let fs3d = `
varying vec2 fsTexcoord;
varying vec4 fsColor;
varying vec4 fsPos;
varying vec3 fsSkyPos;
uniform sampler2D s;

const vec4 skytop = vec4(0.0, 0.0, 0.5, 1.0);
const vec4 skyhorizon = vec4(0.3294, 0.92157, 1.0, 1.0);

void main() {
    vec4 c = mix(skyhorizon, skytop, fsSkyPos.y);
    gl_FragColor = vec4(1.0 + 0.5 * cos(fsSkyPos.z * 10.0), c.gb, 1);
    //gl_FragColor = vec4(1, 1, 1, 1);
    //gl_FragColor = texture2D(s, fsTexcoord)*fsColor;
    //gl_FragColor = texture2D(s, fsTexcoord);
}
`;

let fs = `
varying vec2 d;
varying vec4 e;
uniform sampler2D s;

/*
function len([x,y]) {
    return Math.hypot(x,y);
}
*/

float C(vec2 p, float r) {
    return length(p) - r;
}

/*
function vsub([x,y], [x2,y2]) {
    return [x-x2,y-y2];
}

function intersect(v1,v2) {
    return v1[0] > v2[0] ? v1 : v2;
}

function union_c(v1, v2) {
    return v1[0] < v2[0] ? v1 : v2;
}

function union(d1, d2) {
    return Math.min(d1, d2);
}

function subtract(d1, d2) {
    return Math.max(-d1, d2);
}
*/

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
`;

function genTex(pixels, ty, f) {
    let side = Math.sqrt(pixels.length);
    let w = side;
    let h = side;
    
    function regen() {
        let i = 0;
        let y = 0;
        while (y < h) {
            let x = 0;
            while (x < w) {
                pixels[i] = f(x, y);
                x += 1;
                i += 1;
            }
            y += 1;
        }
    }

    regen();

    let pixtex = createTexture(pixels, side, ty);
    pixtex.d = pixels;
    pixtex.reload = function() {
        regen();
        updateTexture(pixtex, pixels, side, ty);
    };
    return pixtex;
}

function clamp(x, min, max) {
    if (x <= min) {
        return min;
    } else if (x >= max) {
        return max;
    } else {
        return x;
    }
}

function startGame() {
    let imgShader;
    let shader3d;
    let canvas = document.getElementById("g");
    //let canvas = document.getElementsByTagName("canvas")[0];
    //canvas.width = window.innerWidth;
    //canvas.height = window.innerHeight;
    
    render_init(canvas);

    imgShader = createShaderProgram(basicVs, fs);
    shader3d = createShaderProgram(vs3d, fs3d, true);
    //console.log('uniform s:', gl.getUniformLocation(shader3d, 's'));
    //uniLocs = uniNames.map(|name| { return gl.getUniformLocation(imgShader, name) });
    //console.log();
    
    function getPointColor(x, y) {
        let alpha = ((60 - Math.hypot(x - 64, y - 64)) * 64.0) | 0;
        return 0x00ffffff + (clamp(alpha, 0, 255) << 24);
        //return 0x001010ff + (clamp(alpha, 0, 255) << 24);
        //return 0xff00ff00;
    }

    // 16x16
    let cell_size = 8;
    let grid_side = 128 / 8;
    let grid_size = grid_side * grid_side;
    let imgdata = Array(grid_size);

    function shift_sin(x) {
        return (1 + Math.sin(x)) / 2;
    }

    for (var i = 0; i < grid_size; ++i) {
        //imgdata[i] = [Math.random() * (128 - cell_size*2), Math.random() * (128 - cell_size*2), Math.random()];
        imgdata[i] = [shift_sin(i) * (128 - cell_size*2), shift_sin(i + 10) * (128 - cell_size*2), Math.random()];
    }

    function frac_(x, y, i) {
        let xi = x >>> 3;
        let yi = y >>> 3;
        let xf = x & 7;
        let yf = y & 7;

        //let b = ((60 - Math.hypot(x - 64, y - 64)) * 64.0) | 0;
        let b = Math.sin(x / 30.0) + Math.cos(y / 30.0);

        var [xpos, ypos] = imgdata[xi * grid_side + yi];
        if (i <= 1) {
            return b;
        }

        return frac_(xpos + xf * 2, ypos + yf * 2, i - 1) + b;
    }

    function frac(x, y) {
        var alpha = clamp(255 * frac_(x, y, 8) / 8, 0, 255);

        return 0xff000000 + (alpha << 0) + (alpha << 8) + (alpha << 16);
    }

    function len([x,y]) {
        return Math.hypot(x,y);
    }

    function circ(p, r) {
        return len(p) - r;
    }

    function vsub([x,y], [x2,y2]) {
        return [x-x2,y-y2];
    }

    function intersect(v1,v2) {
        return v1[0] > v2[0] ? v1 : v2;
    }

    function union_c(v1, v2) {
        return v1[0] < v2[0] ? v1 : v2;
    }

    function union(d1, d2) {
        return Math.min(d1, d2);
    }

    function subtract(d1, d2) {
        return Math.max(-d1, d2);
    }

    function subtract_c([d1,c1], v2) {
        return -d1 > v2[0] ? [-d1, c1] : v2;
    }

    function mix_color(c1, c2, f) {
        var r = mix((c1 >>> 16) & 0xff, (c2 >>> 16) & 0xff, f) << 16;
        var g = mix((c1 >>> 8) & 0xff, (c2 >>> 8) & 0xff, f) << 8;
        var b = mix(c1 & 0xff, c2 & 0xff, f) << 0;
        return r | g | b;
    }

    function mix(d1,d2,f) {
        return d2 * f + d1 * (1-f);
    }

    function mix_c([d1,c1],[d2,c2],f) {
        return [d2*f + d1*(1-f), mix_color(c1, c2, f)];
    }

    function sm_subtract_c([d1,c1], [d2,c2], k) {
        let h = clamp(0.5 - 0.5*(d2 + d1)/k, 0, 1);
        let r = mix_c([d2,c2], [-d1,c1], h);
        return [r[0] + k*h*(1-h), r[1]];
    }

    /*
    vec4 banana2(vec2 p){
    return vec4(1.,1.,1.,.5)-
    S(vec4(0.,0.,1.,C(p,40.)),vec4(.735,.8,.94,C(p-vec2(20.,20.),30.)),10.);
}*/

    function rkey(x, y) {
        var v = [x/4 - 64, y/4 - 64];
        //var c = subtract_c(
        var c = sm_subtract_c(
            [circ(vsub(v, [0, 0]), 40), 0x0000ffff],
            [circ(vsub(v, [20, 20]), 30), 0x00103344],
            10);

        var alpha = clamp(128 - c[0] * 256, 0, 255);
        return c[1] + (alpha << 24);
    }

    let pointTex = genTex(new Uint32Array(128*128), GL_RGBA, getPointColor);
    //let pointTex = genTex(new Uint32Array(512*512), GL_RGBA, rkey);

    var rotx = 0.0;
    var roty = 0.0;

    canvas.onmousemove = function(e) {
        if (document.pointerLockElement === canvas) {
            rotx += e.movementY;
            roty += e.movementX;
        }
        /*
        y += e.movementY;
        if (x > canvas.width + RADIUS) {
          x = -RADIUS;
        }
        if (y > canvas.height + RADIUS) {
          y = -RADIUS;
        }  
        if (x < -RADIUS) {
          x = canvas.width + RADIUS;
        }
        if (y < -RADIUS) {
          y = canvas.height + RADIUS;
        }
        tracker.textContent = "X position: " + x + ", Y position: " + y;
      
        if (!animation) {
          animation = requestAnimationFrame(function() {
            animation = null;
            canvasDraw();
          });
        }
        */
      }

    let t = 0.0;
    function update() {
        window.requestAnimationFrame(function(currentTime) {
            update();
            render_w = canvas.width = window.innerWidth;
            render_h = canvas.height = window.innerHeight;
            gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
            gl.clear(GL_COLOR_BUFFER_BIT);

            /*
            let p = Math.random() * grid_size >>> 0;
            imgdata[p][0] = clamp(imgdata[p][0] + Math.random(), 0, 128 - cell_size*2);
            imgdata[p][1] = clamp(imgdata[p][0] + Math.random(), 0, 128 - cell_size*2);
            pointTex.reload();
            */

            {
                t += 5;
                //setView3d(rotx / 10, 1.0);
                setView3d(roty / 500, rotx / 500, t, t, 1.0);
                activateShader3d(shader3d);

                color(0xff770077);
                for (var x = -10000; x < 10000; x += 512 * 2 * 1.5) {
                    img3d(pointTex, x, -256 * 4, -2500, 512 * 2, 512 * 4, 0, 0, 1, 1);
                }
                //img3d(null, -0.5, -0.5, 0.1, 1, 1, 0, 0, 1, 1);
                flush3d();
            }

            {
                setView(0, 0, 1, 0, 1.0);
                //t += 0.01;
                activateShader(imgShader);
                //img_simple(Math.random(), Math.random(), 0xffffffff);
                color(0xffffffff);
                //img(pointTex, -64.0, -64.0, 128, 128, 0, 0, 1, 1);
                //img(pointTex, -128.0, -128.0, 256, 256, 0, 0, 1, 1);
                img(pointTex, -256, -256, 512, 512, 0, 0, 1, 1);
                
                flush();
            }
        });
    }

    update();
}

window.onload = function() {
    startGame();

    window.onkeydown = function(ev) {
        //wasm_U8[wasm.KEYS + ev.keyCode] = 1;
    };

    window.onkeyup = function(ev) {
        //wasm_U8[wasm.KEYS + ev.keyCode] = 0;
    };
};

})(this)