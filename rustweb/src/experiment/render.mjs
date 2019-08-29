import * as mat from './mat.mjs';

export let gl
export let canvas_w
export let canvas_h
export let viewTrans
export let viewTrans3d
export let viewPos3d

export let GL_VERTEX_SHADER = 0x8B31
export let GL_FRAGMENT_SHADER = 0x8B30
export let GL_ELEMENT_ARRAY_BUFFER = 0x8893
export let GL_ARRAY_BUFFER = 0x8892
export let GL_TEXTURE0 = 0x84C0

export let GL_SRC_ALPHA =           0x0302
export let GL_ONE_MINUS_SRC_ALPHA = 0x0303
export let GL_DST_ALPHA =           0x0304
export let GL_ONE_MINUS_DST_ALPHA = 0x0305
export let GL_DST_COLOR =           0x0306
export let GL_ONE_MINUS_DST_COLOR = 0x0307
export let GL_BLEND = 0x0BE2
export let GL_RGBA = 0x1908
export let GL_LUMINANCE = 0x1909

export let GL_TRIANGLES = 0x0004
export let GL_TRIANGLE_STRIP = 0x0005
export let GL_UNSIGNED_BYTE = 0x1401
export let GL_UNSIGNED_SHORT = 0x1403
export let GL_FLOAT = 0x1406
export let GL_STATIC_DRAW = 0x88E4
export let GL_DYNAMIC_DRAW = 0x88E8
export let GL_COMPILE_STATUS = 0x8B81
export let GL_LINK_STATUS = 0x8B82
export let GL_TEXTURE_2D = 0x0DE1
export let GL_TEXTURE_WRAP_S = 0x2802
export let GL_TEXTURE_WRAP_T = 0x2803
export let GL_TEXTURE_MAG_FILTER = 0x2800
export let GL_TEXTURE_MIN_FILTER = 0x2801
export let GL_NEAREST = 0x2600
export let GL_LINEAR = 0x2601
export let GL_CLAMP_TO_EDGE = 0x812F
export let GL_COLOR_BUFFER_BIT = 0x00004000

export let basicVs = `
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

export let vs3d = `
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
    
}`;

let VERTEX_SIZE = (4 * 2) + (4 * 2) + (4)
let VERTEX_SIZE3D = (4 * 3) + (4 * 2) + (4 * 4)
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

let arrPositionData3d = [];
let locPos = 0
let locUV = 1
let locColor = 2
let col = 0xffffffff

export function initGl(canvas) {
    gl = canvas.getContext("webgl")
    //gl.clearColor(0.0, 0.2, 0.0, 1.0)
    gl.clearColor(0.2, 0.25, 0.4, 1.0)
}

export function updateWindow(canvas) {
    canvas_w = canvas.width = window.innerWidth;
    canvas_h = canvas.height = window.innerHeight;
}

export function createBuffer(bufferType, size, usage) {
    let buffer = gl.createBuffer()
    gl.bindBuffer(bufferType, buffer)
    gl.bufferData(bufferType, size, usage)
    return buffer
}

export function createTexture(image, side, ty) {
    let texture = gl.createTexture()
    checkErr(gl.bindTexture(GL_TEXTURE_2D, texture))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE))
    checkErr(gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR))
    checkErr(gl.texImage2D(GL_TEXTURE_2D, 0, ty, side, side, 0, ty, GL_UNSIGNED_BYTE, new Uint8Array(image.buffer)))
    console.log("texImage")
    return texture
}

export function updateTexture(texture, image, side, ty) {
    gl.bindTexture(GL_TEXTURE_2D, texture)
    gl.texSubImage2D(GL_TEXTURE_2D, 0, 0, 0, side, side, ty, GL_UNSIGNED_BYTE, new Uint8Array(image.buffer))
    return texture
}

export function setViewTransform(shader) {
    gl.uniformMatrix3fv(gl.getUniformLocation(shader, "m"), 0, viewTrans)
}

export function setViewTransform3d(shader) {
    gl.uniformMatrix4fv(gl.getUniformLocation(shader, "modelView"), 0, viewTrans3d)

    //var p = perspective3d(1.58, canvas_w / canvas_h, 1, 3000);
    var p = mat.perspective3d(1, canvas_w / canvas_h, 1, 30000);
    gl.uniformMatrix4fv(gl.getUniformLocation(shader, "projection"), 0, p)

    //gl.uniformMatrix3fv(gl.getUniformLocation(shader, "viewPos"), 0, viewPos)
    gl.uniform3fv(gl.getUniformLocation(shader, "viewPos"), viewPos3d)
}

export function color(c) {
    col = c
}

export function setView(x, y, rotx, roty, zoom) {
    let ratio = canvas_h / canvas_w

    viewTrans = [
        1, 0, 0,
        0, 1, 0,
        0, 0, 1
    ]

    mat.scale(viewTrans, 2 / (1024 * zoom), 2 / (1024 * zoom * ratio))
    mat.rotvec(viewTrans, rotx, roty)
    mat.trans(viewTrans, -x, -y)
}

export function setView3d(roty, rotx, tx, ty, zoom) {

    /*
    viewTrans3d = [
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    ]*/

    /*
                1 0 0 0,
                0 1 0 0,
                0 0 1 0,
                x y z 1
    a b c 0  a b c 0
    e f g 0  e f g 0
    i j k 0  i j k 0
    m n o p  m+x n+y o+z p
    */

    var a = mat.mulmat3d(mat.rotx3d(rotx), mat.roty3d(roty));
    viewTrans3d = mat.mulmat3d(a, mat.trans3d(tx, 0, ty));
    //console.log(a)
    //console.log(viewTrans3d)
    //viewTrans3d = mulmat3d(rotx3d(rotx), roty3d(roty));
    //viewTrans3d[12] += tx;
    //viewTrans3d[14] += ty;
    viewPos3d = [tx, 0, ty]

    mat.scale3d(viewTrans3d, 1 / zoom, 1 / zoom, 1 / zoom);
}

export function flush() {
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

export function flush3d() {
    if (arrPositionData3d.length) {
        gl.bindBuffer(GL_ARRAY_BUFFER, VBO3D)
        checkErr(gl.vertexAttribPointer(locPos, 3, GL_FLOAT, 0, VERTEX_SIZE3D, 0))
        checkErr(gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE3D, 12))
        checkErr(gl.vertexAttribPointer(locColor, 4, GL_FLOAT, 0, VERTEX_SIZE3D, 20))
        gl.bufferSubData(GL_ARRAY_BUFFER, 0, new Float32Array(arrPositionData3d));
        //console.log(arrPositionData3d.length, arrPositionData3d.length / (VERTEX_SIZE3D / 4));
        gl.drawArrays(GL_TRIANGLES, 0, arrPositionData3d.length / (VERTEX_SIZE3D / 4))
        arrPositionData3d.length = 0;
    }
    currentTexture = null
}

export function img3d(texture, x, y, z, w, h, u0, v0, u1, v1) {
    let x0 = x,     y0 = y,     z0 = z;
    let x1 = x + w, y1 = y + h, z1 = z;
    let x2 = x,     y2 = y + h, z2 = z;
    let x3 = x + w, y3 = y,     z3 = z;
    let abgr = col

    if (texture != currentTexture) {
        flush3d()
        if (currentTexture != texture) {
            currentTexture = texture
            gl.bindTexture(GL_TEXTURE_2D, currentTexture)
        }
    }

    var a = (abgr >>> 24) / 255;
    var b = ((abgr >> 16) & 0xff) / 255;
    var g = ((abgr >> 8) & 0xff) / 255;
    var r = (abgr & 0xff) / 255;

    if (arrPositionData3d.push(
        x0,y0,z0,u0,v0,r,g,b,a,
        x3,y3,z3,u1,v0,r,g,b,a,
        x1,y1,z1,u1,v1,r,g,b,a,
        x0,y0,z0,u0,v0,r,g,b,a,
        x1,y1,z1,u1,v1,r,g,b,a,
        x2,y2,z2,u0,v1,r,g,b,a) >= MAX_BATCH * QUAD_SIZE_IN_WORDS3D) {

        flush3d();
    }
}

export function wall3d(texture, fx0, fz0, fx1, fz1, fy, cy, u0, v0, u1, v1) {
    let x0 = fx0,     y0 = fy,     z0 = fz0;
    let x1 = fx1,     y1 = fy,     z1 = fz1;
    let x2 = fx1,     y2 = cy,     z2 = fz1;
    let x3 = fx0,     y3 = cy,     z3 = fz0;
    let abgr = col

    if (texture != currentTexture) {
        flush3d()
        if (currentTexture != texture) {
            currentTexture = texture
            gl.bindTexture(GL_TEXTURE_2D, currentTexture)
        }
    }

    var a = (abgr >>> 24) / 255;
    var b = ((abgr >> 16) & 0xff) / 255;
    var g = ((abgr >> 8) & 0xff) / 255;
    var r = (abgr & 0xff) / 255;

    if (arrPositionData3d.push(
        x0,y0,z0,u0,v1,r,g,b,a,
        x1,y1,z1,u1,v1,r,g,b,a,
        x3,y3,z3,u0,v0,r,g,b,a,
        x1,y1,z1,u1,v1,r,g,b,a,
        x2,y2,z2,u1,v0,r,g,b,a,
        x3,y3,z3,u0,v0,r,g,b,a) >= MAX_BATCH * QUAD_SIZE_IN_WORDS3D) {

        flush3d();
    }
}

export function img(texture, x, y, w_, h_, u0, v0, u1, v1) {
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

export function activateShader(shader) {
    gl.useProgram(shader)
    setViewTransform(shader)
}

export function activateShader3d(shader) {
    gl.useProgram(shader)
    setViewTransform3d(shader)
    gl.uniform1i(gl.getUniformLocation(shader, 's'), 0);
}

export function compileShader(source, ty) {
    let shader = gl.createShader(ty)
    gl.shaderSource(shader, "#extension GL_OES_standard_derivatives:enable\nprecision lowp float;" + source)
    //gl.shaderSource(shader, "precision lowp float;" + source)
    gl.compileShader(shader)

    if (!gl.getShaderParameter(shader, GL_COMPILE_STATUS)) {
        //console.log(`Error compiling ${ty === GL_VERTEX_SHADER ? "vertex" : "fragment"} shader:`)
        console.log(gl.getShaderInfoLog(shader))
    }
    return shader
}

export function bindAttribLocations(shader) {
    ["a", "b", "c"].map((name, i) => gl.bindAttribLocation(shader, i, name))
}

export function bindAttribLocations3d(shader) {
    ["pos", "texcoord", "color"].map((name, i) => gl.bindAttribLocation(shader, i, name))
}

export function createShaderProgram(vsSource, fsSource, is3d) {
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

export function checkErr(v) {
    /* TEMP
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
    */
    return v
}

export function init(canvas) {
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