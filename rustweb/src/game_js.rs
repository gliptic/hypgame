//use wasm_bindgen::prelude::*;
use rustweb_macro::{javascript, glsl, glsl2};

javascript!{ game
    use render; // Import

    // wasm.memory

    pub fn foo(x: u32) {
        return -x;
    }

    let uniLocs;
    let uniNames = ["t", "P", "R"];

    let fs = glsl.main;
    let wasm_U8 = Uint8Array.new(wasm.memory.buffer);

    //console.log(wasm.RENDER_BUF);

    fn genTex(pixels: _, ty: _, f: _) {
        let side = Math.sqrt(pixels.length);
        let (w, h) = (side, side);
        let i = 0;
        let y = 0;

        while y < h {
            let x = 0;
            while x < w {
                pixels[i] = f(x, y);
                x += 1;
                i += 1;
            }
            y += 1;
        }

        let pixtex = render.createTexture(pixels, side, ty);
        pixtex.d = pixels;
        return pixtex;
    }

    fn clamp(x: _, min: _, max: _) {
        if x <= min {
            return min;
        } else if x >= max {
            return max;
        } else {
            return x;
        }
    }

    fn startGame() {
        let (gl, imgShader);
        let canvas = document.getElementById("g");
        //let canvas = document.getElementsByTagName("canvas")[0];
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        
        render.init(canvas);
        gl = render.gl;

        imgShader = render.createShaderProgram(render.basicVs, fs);
        //uniLocs = uniNames.map(|name| { return gl.getUniformLocation(imgShader, name) });
        //console.log();
        
        fn getPointColor(x: _, y: _) {
            let alpha = ((7.5 - Math.hypot(x - 8, y - 8)) * 64.0) | 0;
            //return 0x00ffffff + (clamp(alpha, 0, 255) << 24);
            return 0x001010ff + (clamp(alpha, 0, 255) << 24);
        }

        let pointTex = genTex(Uint32Array.new(16*16), render.GL_RGBA, getPointColor);

        let rot = 0.0;
        fn update() {
            window.requestAnimationFrame(|currentTime| {
                update();
                render.w = canvas.width = window.innerWidth;
                render.h = canvas.height = window.innerHeight;
                gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
                gl.clear(render.GL_COLOR_BUFFER_BIT);
                render.setView(0, 0, Math.cos(rot), Math.sin(rot), 1.0);
                //rot += 0.01;
                render.activateShader(imgShader);
                render.img_simple(Math.random(), Math.random(), 0xffffffff);
                if (false) {
                    //render.color(0xffff1010);
                    render.color(0xffffffff);
                    render.img(pointTex, 0.0, 0.0, 50, 50, 0, 0, 1, 1);
                    
                    render.flush();
                } else {
                    gl.bindTexture(render.GL_TEXTURE_2D, pointTex);
                    wasm.rend();
                }
            });
        }

        update();
    }

    window.onload = || {
        startGame();

        window.onkeydown = |ev| {
            wasm_U8[wasm.KEYS + ev.keyCode] = 1;
        };

        window.onkeyup = |ev| {
            wasm_U8[wasm.KEYS + ev.keyCode] = 0;
        };
    };


}