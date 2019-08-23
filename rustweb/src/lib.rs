#![no_std]
#![feature(asm)]
#![feature(ptr_offset_from)]
//#![feature(maybe_uninit_ref)]

use core::panic::PanicInfo;

extern crate wee_alloc;
extern crate rustweb_macro;
use rustweb_macro::{javascript, glsl, glsl2};
use unchecked_index::unchecked_index;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//use wasm_bindgen::prelude::*;
//use sillyvec::SillyVec;

mod game_js;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
html!{ main

    <html style="width:100%;height:100%;margin:0px;overflow:hidden">
    <head>
        <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    </head>
    <body style="width:100%;height:100%;margin:0px;overflow:hidden">
        <div style="position: relative">
        <canvas id="g"></canvas>
        </div>
        <!-- <script src="rustweb.js"></script> -->
        <script>{}</script>
    </body>
    </html>
}*/

javascript! { vecmath

    fn trans(mat: _, x: _, y: _) {
        mat[6] += mat[0] * x + mat[3] * y;
        mat[7] += mat[1] * x + mat[4] * y;
    }

    fn scale(mat: _, x: _, y: _) {
        mat[0] *= x;
        mat[1] *= x;
        mat[3] *= y;
        mat[4] *= y;
    }

    fn rotvec(mat: _, x: _, y: _) {
        let a = mat[0];
        let b = mat[1];
        let c = mat[3];
        let d = mat[4];

        mat[0] = a * x + c * -y;
        mat[1] = b * x + d * -y;
        mat[3] = a * y + c * x;
        mat[4] = b * y + d * x;
    }
}

glsl2!{ main
    #![version(120)]

    //#[attribute] static a: vec2 = UNDEF;

    #[varying] static d: vec2 = UNDEF;
    #[varying] static e: vec4 = UNDEF;
    #[uniform] static s: sampler2D = UNDEF;

    /*
        fn pie(p: vec2, angle: float) -> float {
            angle = radians(angle) / 2.0;
            let n: vec2 = vec2(cos(angle), sin(angle));
            return abs(p).x * n.x + p.y*n.y;
        }

        fn rotateCCW(p: vec2, a: float) -> vec2 {
            let m: mat2 = mat2(cos(a), sin(a), -sin(a), cos(a));
            return p * m;
        }
    */
    pub fn main() {
        gl_FragColor = texture2D(s, d)*e;
    }

}

glsl2!{ vertex
    // IN Vertex Position and
    // IN Texture Coordinates
    #[attribute] static a: vec2 = UNDEF;
    #[attribute] static b: vec2 = UNDEF;
    // IN Vertex Color
    #[attribute] static c: vec4 = UNDEF;
    // OUT Texture Coordinates
    #[varying] static d: vec2 = UNDEF;
    #[varying] static f: vec2 = UNDEF;
    // OUT Vertex Color
    #[varying] static e: vec4 = UNDEF;
    // CONST View Matrix
    #[uniform] static m: mat3 = UNDEF;

    fn main() {
        d = b;
        e = c;
        f = a;
        gl_Position = vec4(m * vec3(a, 1.0), 1.0);
    }
}

javascript!{ render

    use vecmath;
    
    static gl: _ = 0;
    static w: _ = 0;
    static h: _ = 0;
    static viewTrans: _ = 0;

    static GL_VERTEX_SHADER: _ = 0x8B31;
    static GL_FRAGMENT_SHADER: _ = 0x8B30;
    static GL_ELEMENT_ARRAY_BUFFER: _ = 0x8893;
    static GL_ARRAY_BUFFER: _ = 0x8892;
    static GL_TEXTURE0: _ = 0x84C0;

    static GL_SRC_ALPHA: _ =           0x0302;
    static GL_ONE_MINUS_SRC_ALPHA: _ = 0x0303;
    static GL_DST_ALPHA: _ =           0x0304;
    static GL_ONE_MINUS_DST_ALPHA: _ = 0x0305;
    static GL_DST_COLOR: _ =           0x0306;
    static GL_ONE_MINUS_DST_COLOR: _ = 0x0307;
    static GL_BLEND: _ = 0x0BE2;
    static GL_RGBA: _ = 0x1908;
    static GL_LUMINANCE: _ = 0x1909;

    static GL_TRIANGLES: _ = 0x0004;
    static GL_TRIANGLE_STRIP: _ = 0x0005;
    static GL_UNSIGNED_BYTE: _ = 0x1401;
    static GL_UNSIGNED_SHORT: _ = 0x1403;
    static GL_FLOAT: _ = 0x1406;
    static GL_STATIC_DRAW: _ = 0x88E4;
    static GL_DYNAMIC_DRAW: _ = 0x88E8;
    static GL_COMPILE_STATUS: _ = 0x8B81;
    static GL_LINK_STATUS: _ = 0x8B82;
    static GL_TEXTURE_2D: _ = 0x0DE1;
    static GL_TEXTURE_WRAP_S: _ = 0x2802;
    static GL_TEXTURE_WRAP_T: _ = 0x2803;
    static GL_TEXTURE_MAG_FILTER: _ = 0x2800;
    static GL_TEXTURE_MIN_FILTER: _ = 0x2801;
    static GL_NEAREST: _ = 0x2600;
    static GL_LINEAR: _ = 0x2601;
    static GL_CLAMP_TO_EDGE: _ = 0x812F;
    static GL_COLOR_BUFFER_BIT: _ = 0x00004000;

    static basicVs: _ = glsl.vertex;

    let wasm_memory = wasm.memory.buffer;
    let wasm_U8 = Uint8Array.new(wasm_memory);
    let wasm_U32 = Uint32Array.new(wasm_memory);
    let VERTEX_SIZE = (4 * 2) + (4 * 2) + (4);
    let MAX_BATCH = 10922; // floor((2 ^ 16) / 6)
    let MAX_STACK = 100;
    let MAT_SIZE = 6;
    let VERTICES_PER_QUAD = 6;
    let QUAD_SIZE_IN_WORDS = (VERTEX_SIZE * VERTICES_PER_QUAD) / 4;
    let VERTEX_DATA_SIZE = VERTEX_SIZE * MAX_BATCH * 6;

    let VBO;
    let count = 0;
    let currentTexture;
    let vertexData = ArrayBuffer.new(VERTEX_DATA_SIZE);
    let vPositionData = Float32Array.new(vertexData);
    let vColorData = Uint32Array.new(vertexData);
    let mat0 = 1;
    let mat1 = 0;
    let mat2 = 0;
    let mat3 = 1;
    let mat4 = 0;
    let mat5 = 0;
    let stack = [];
    let (locPos, locUV, locColor) = (0, 1, 2);
    let col = 0xffffffff;

    fn initGl(canvas: _) {
        gl = canvas.getContext("webgl");
        w = canvas.width;
        h = canvas.height;
        gl.clearColor(0.0, 0.2, 0.0, 1.0); // TODO: May not need this later
    }

    fn createBuffer(bufferType: _, size: _, usage: _) {
        let buffer = gl.createBuffer();
        gl.bindBuffer(bufferType, buffer);
        gl.bufferData(bufferType, size, usage);
        return buffer;
    }

    fn createTexture(image: _, side: _, ty: _) {
        let texture = gl.createTexture();
        gl.bindTexture(GL_TEXTURE_2D, texture);
        gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
        gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        // MAG_FILTER is GL_LINEAR by default
        gl.texImage2D(GL_TEXTURE_2D, 0, ty, side, side, 0, ty, GL_UNSIGNED_BYTE, Uint8Array.new(image.buffer));
        console.log("texImage");
        return texture;
    }

    fn setViewTransform(shader: _) {
        // TODO: glsl.main.m instead of "m"
        gl.uniformMatrix3fv(gl.getUniformLocation(shader, "m"), 0, viewTrans);
    }

    fn color(c: _) {
        col = c;
    }

    fn setView(x: _, y: _, rotx: _, roty: _, zoom: _) {
        let ratio = h / w;

        viewTrans = [
            1, 0, 0,
            0, 1, 0,
            0, 0, 1
        ];

        vecmath.scale(viewTrans, 2 / (1024 * zoom), 2 / (1024 * zoom * ratio));
        vecmath.rotvec(viewTrans, rotx, roty);
        vecmath.trans(viewTrans, -x, -y)
    }

    pub fn flush_buf(buf_p: *const u32, cou: u32) {
        gl.bufferSubData(GL_ARRAY_BUFFER, 0, wasm_U8.subarray(buf_p, buf_p + cou * 5 * 4));
        gl.drawArrays(GL_TRIANGLES, 0, cou);
    }

    fn flush() {
        if (count) {
            gl.bufferSubData(GL_ARRAY_BUFFER, 0, vPositionData.buffer.subarray(0, count * QUAD_SIZE_IN_WORDS));
            gl.drawArrays(GL_TRIANGLES, 0, count * VERTICES_PER_QUAD);
            count = 0;
        }
        currentTexture = null;
    }

    fn img(texture: _, x: _, y: _, w_: _, h_: _, u0: _, v0: _, u1: _, v1: _) {
        let x0 = x;
        let y0 = y;
        let x1 = x + w_;
        let y1 = y + h_;
        let x2 = x;
        let y2 = y + h_;
        let x3 = x + w_;
        let y3 = y;
        let offset = 0;
        let argb = col;

        if texture != currentTexture || count + 1 >= MAX_BATCH {
            flush();
            if currentTexture != texture {
                currentTexture = texture;
                gl.bindTexture(GL_TEXTURE_2D, currentTexture);
            }
        }

        offset = count * QUAD_SIZE_IN_WORDS - 1;
        // Vertex Order
        // Vertex Position | UV | ARGB
        // Vertex 1
        vPositionData[offset += 1] = x0 * mat0 + y0 * mat2 + mat4;
        vPositionData[offset += 1] = x0 * mat1 + y0 * mat3 + mat5;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;

        // Vertex 4
        vPositionData[offset += 1] = x3 * mat0 + y3 * mat2 + mat4;
        vPositionData[offset += 1] = x3 * mat1 + y3 * mat3 + mat5;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;
        
        // Vertex 2
        vPositionData[offset += 1] = x1 * mat0 + y1 * mat2 + mat4;
        vPositionData[offset += 1] = x1 * mat1 + y1 * mat3 + mat5;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;

        // Vertex 1
        vPositionData[offset += 1] = x0 * mat0 + y0 * mat2 + mat4;
        vPositionData[offset += 1] = x0 * mat1 + y0 * mat3 + mat5;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;

        // Vertex 2
        vPositionData[offset += 1] = x1 * mat0 + y1 * mat2 + mat4;
        vPositionData[offset += 1] = x1 * mat1 + y1 * mat3 + mat5;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;
        
        // Vertex 3
        vPositionData[offset += 1] = x2 * mat0 + y2 * mat2 + mat4;
        vPositionData[offset += 1] = x2 * mat1 + y2 * mat3 + mat5;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;
        
        if (count += 1) >= MAX_BATCH {
            flush();
        }
    }

    fn img_simple(x: _, y: _, argb: _) {
        let x0 = x;
        let y0 = y;
        let x1 = x + 25;
        let y1 = y + 25;
        let x2 = x;
        let y2 = y + 25;
        let x3 = x + 25;
        let y3 = y;
        let offset = 0;
        let u0 = 0;
        let v0 = 0;
        let u1 = 1;
        let v1 = 1;

        offset = count * QUAD_SIZE_IN_WORDS - 1;
        // Vertex Order
        // Vertex Position | UV | ARGB
        // Vertex 1
        vPositionData[offset += 1] = x0;
        vPositionData[offset += 1] = y0;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;

        // Vertex 4
        vPositionData[offset += 1] = x3;
        vPositionData[offset += 1] = y3;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;
        
        // Vertex 2
        vPositionData[offset += 1] = x1;
        vPositionData[offset += 1] = y1;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;

        // Vertex 1
        vPositionData[offset += 1] = x0;
        vPositionData[offset += 1] = y0;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v0;
        vColorData[offset += 1] = argb;

        // Vertex 2
        vPositionData[offset += 1] = x1;
        vPositionData[offset += 1] = y1;
        vPositionData[offset += 1] = u1;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;
        
        // Vertex 3
        vPositionData[offset += 1] = x2 * mat0 + y2 * mat2 + mat4;
        vPositionData[offset += 1] = x2 * mat1 + y2 * mat3 + mat5;
        vPositionData[offset += 1] = u0;
        vPositionData[offset += 1] = v1;
        vColorData[offset += 1] = argb;
        
        if (count += 1) >= MAX_BATCH {
            flush();
        }
    }

    fn activateShader(shader: _) {
        gl.useProgram(shader);
        setViewTransform(shader);
    }

    fn compileShader(source: _, ty: _) {
        let shader = gl.createShader(ty);
        gl.shaderSource(shader, "#extension GL_OES_standard_derivatives:enable\nprecision lowp float;" + source);
        gl.compileShader(shader);

        if (!gl.getShaderParameter(shader, GL_COMPILE_STATUS)) {
            //console.log(`Error compiling ${ty === GL_VERTEX_SHADER ? "vertex" : "fragment"} shader:`);
            console.log(gl.getShaderInfoLog(shader));
        }
        return shader;
    }

    fn bindAttribLocations(shader: _) {
        ["a", "b", "c"].map(|name, i| { gl.bindAttribLocation(shader, i, name) });
    }

    fn createShaderProgram(vsSource: _, fsSource: _) {
        let program = gl.createProgram();
        let vShader = compileShader(vsSource, GL_VERTEX_SHADER);
        let fShader = compileShader(fsSource, GL_FRAGMENT_SHADER);
        gl.attachShader(program, vShader);
        gl.attachShader(program, fShader);
        gl.linkProgram(program);

        if (!gl.getProgramParameter(program, GL_LINK_STATUS)) {
            console.log("Error linking shader program:");
            console.log(gl.getProgramInfoLog(program));
        }

        bindAttribLocations(program);
        return program;
    }

    fn checkErr(v: _) {
        let err = gl.getError();
        if err != 0 {
            console.log("error:", err);
            console.trace();
        }
        return v;
    }

    fn init(canvas: _) {
        initGl(canvas);
        gl.blendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        gl.enable(GL_BLEND);

        gl.getExtension("OES_standard_derivatives");
        
        VBO = createBuffer(GL_ARRAY_BUFFER, VERTEX_DATA_SIZE, GL_DYNAMIC_DRAW);
        gl.bindBuffer(GL_ARRAY_BUFFER, VBO);
        
        gl.enableVertexAttribArray(locPos);
        gl.enableVertexAttribArray(locUV);
        gl.enableVertexAttribArray(locColor);
        gl.vertexAttribPointer(locPos, 2, GL_FLOAT, 0, VERTEX_SIZE, 0);
        gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE, 8);
        gl.vertexAttribPointer(locColor, 4, GL_UNSIGNED_BYTE, 1, VERTEX_SIZE, 16);
        
        gl.activeTexture(GL_TEXTURE0);
    }
}

use core::mem::MaybeUninit;

const MAX_BATCH: usize = 10922;

static mut RENDER_BUF: MaybeUninit<[Vertex; MAX_BATCH * 6]> = MaybeUninit::uninit();
static mut RENDER_COUNT: usize = 0;
static mut STATE: State = State { entities: [Entity { x: 0, y: 0 }; 20], entity_count: 1, you: 0 };

#[export_name = "KEYS"]
static mut KEYS: [u8; 256] = [0u8; 256];
//static mut STATE: MaybeUninit<State> = MaybeUninit::zeroed(); //State { entities: [Entity { x: 0.0, y: 0.0 }; 20] };

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
    u: f32,
    v: f32,
    col: u32
}

#[derive(Clone, Copy, Default)]
struct Entity {
    x: u32,
    y: u32
}

const W: usize = 32;
const H: usize = 32;

enum CellKind {
    Floor = 0,
    Wall = 1,
}

struct Cell {
    kind: CellKind
}

struct Map {
    cells: [Cell; W * H]
}

struct State {
    entities: [Entity; 20],
    entity_count: usize,
    you: usize
}

struct Control {
    mov: (i8, i8),
    pick: bool,
    use_: bool
}

#[cfg(not(all(target_os = "nacl", target_arch = "le32")))]
pub fn black_box<T>(dummy: T) -> T {
    // we need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}

impl State {
    #[inline(never)]
    fn update(&mut self /*, control: Control*/) {
        let mut ent = unsafe { unchecked_index(&mut self.entities[..]) };


        let me = (&mut ent[self.you]);
        /*
        me.x = (me.x as i32 + control.mov.0 as i32) as u32;
        me.y = (me.y as i32 + control.mov.1 as i32) as u32;
        */
        unsafe {
            if KEYS[37] != 0 { me.x -= 1 }
            if KEYS[38] != 0 { me.y += 1 }
            if KEYS[39] != 0 { me.x += 1 }
            if KEYS[40] != 0 { me.y -= 1 }
        }
    }
}

pub fn rend2(x0: f32, y0: f32, argb: u32) {
    
    //let mut loc_buf: MaybeUninit<[Vertex; MAX_BATCH * 6]> = MaybeUninit::uninit();
    //let mut buf = unsafe { loc_buf.as_mut_ptr() as *mut Vertex };
    let mut buf = unsafe { RENDER_BUF.as_mut_ptr() as *mut Vertex };
    let x1 = x0 + 25.0;
    let y1 = y0 + 25.0;
    let x2 = x0;
    let y2 = y0 + 25.0;
    let x3 = x0 + 25.0;
    let y3 = y0;

    let u0 = 0.0;
    let v0 = 0.0;
    let u1 = 1.0;
    let v1 = 1.0;

    let offset = unsafe { RENDER_COUNT };

    unsafe {
        buf = buf.offset(offset as isize);

        buf.write(Vertex {
            x: x0, y: y0,
            u: u0, v: v0,
            col: argb
        });

        // Vertex 4
        buf.offset(1).write(Vertex {
            x: x3, y: y3,
            u: u1, v: v0,
            col: argb
        });

        // Vertex 2
        buf.offset(2).write(Vertex {
            x: x1, y: y1,
            u: u1, v: v1,
            col: argb
        });

        // Vertex 1
        buf.offset(3).write(Vertex {
            x: x0, y: y0,
            u: u0, v: v0,
            col: argb
        });

        // Vertex 2
        buf.offset(4).write(Vertex {
            x: x1, y: y1,
            u: u1, v: v1,
            col: argb
        });

        // Vertex 3
        buf.offset(5).write(Vertex {
            x: x2, y: y2,
            u: u0, v: v1,
            col: argb
        });
    }

    if unsafe { RENDER_COUNT += 6; RENDER_COUNT } >= MAX_BATCH * 6 {
        flush();
    }
}

fn flush() {
    unsafe {
        flush_buf(RENDER_BUF.as_ptr() as *const u32, RENDER_COUNT as u32);
        RENDER_COUNT = 0;
    }
}

#[export_name = "rend"]
pub extern "C" fn rend() {
    unsafe {
        let state = &mut STATE;

        //let mut ent = unsafe { unchecked_index(&mut state.entities[..]) };

        /* let me = &mut state.entities[state.you];
        if KEYS[37] != 0 { me.x -= 1 }
        if KEYS[38] != 0 { me.y += 1 }
        if KEYS[39] != 0 { me.x += 1 }
        if KEYS[40] != 0 { me.y -= 1 } */
        state.update();
        
        for i in 0..state.entity_count {
            let mut e = &mut state.entities[i as usize];
            rend2(e.x as f32, e.y as f32, 0xffffffffu32);
        }

        flush();
    }
}
