import * as render from './render.mjs';
import * as mat from './mat.mjs';

let uniLocs;
let uniNames = ["t", "P", "R"];

let rand = x => lim => (x^=(x^=(x^=x<<5)<<13)>>17)%lim>>>0;
let r = rand(Math.random() * 10000 | 0);

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

    let pixtex = render.createTexture(pixels, side, ty);
    pixtex.d = pixels;
    pixtex.reload = function() {
        regen();
        render.updateTexture(pixtex, pixels, side, ty);
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

var Grid = function(w,h) {
    var g = {
        c: Array(w*h).fill(0),
        get: function(x, y) {
            return this.c[y * w + x];
        },
        set: function(x, y, v) {
            this.c[y * w + x] = v;
        },
    };

    for (var y = 0; y < h; ++y) {
        for (var x = 0; x < w; ++x) {
            if (x == 0 || x == w - 1 || y == 0 || y == h - 1) {
                g.set(x, y, 1);
            }
        }
    }
    return g;
}

var map = Grid(32, 32);
var visual = Grid(32, 32);
var sweeps = [];

for (var i = 0; i < 50; ++i) {
    map.set(r(31), r(31), 1);
}

function sweep6(x, y, dx, dy) {
    let ix = x | 0;
    let iy = y | 0;

    let stepX = Math.sign(dx), stepY = Math.sign(dy);
    let sideDistX = ix - x + (dx >= 0);
    let sideDistY = iy - y + (dy >= 0);

    while (!map.c[ix + iy*32]) {
        visual.c[ix + iy*32] = 0xffff00ff;
        let northSouth = sideDistY * stepX * stepY * dx < sideDistX * stepX * stepY * dy;

        if (northSouth) {
            sideDistY += stepY;
            iy += stepY;
        } else {
            sideDistX += stepX;
            ix += stepX;
        }
    }
}

function sweep5(x, y, dx, dy) {
    let ix = x | 0;
    let iy = y | 0;

    while (!map.c[ix + iy*32]) {
        visual.c[ix + iy*32] = 0xffff00ff;
        let northSouth = (iy - y + (dy >= 0)) / dy < (ix - x + (dx >= 0)) / dx;

        if (northSouth) {
            iy += Math.sign(dy);
        } else {
            ix += Math.sign(dx);
        }
    }
}

function sweep4(x, y, dx, dy) {
    while (!map.c[x | (y << 5)]) {
        visual.c[x | (y << 5)] = 0xffff00ff;
        x += dx;
        y += dy;
    }
}

function sweep3(origin, x, y0, y1) {

    for (var y = Math.floor(y0); y < y1 + 1; ++y) {
        // Check wall (x - dx, y) <-> (x, y)
        if (map.get(x, y) || y >= y1) {
            var segy0 = y0;
            var segy1 = Math.min(y, y1);
            if (segy0 < segy1) {
                var nextx = x + 1;
                var s0 = (segy0 - origin[1]) / (x - origin[0]);
                var s1 = (segy1 - origin[1]) / (x - origin[0]);
                var nexty0 = segy0 + s0;
                var nexty1 = segy1 + s1;
                
                var begin = Math.floor(segy0);
                var end = Math.ceil(segy1);
                var sign0 = Math.sign(s0);
                var sign1 = Math.sign(s1);

                for (;;) {
                    if (begin <= nexty0) {
                        // Nothing blocked
                        begin = nexty0;
                        break;
                    }

                    // Check wall (x, begin - sign0) <-> (x, begin)
                    if (map.get(x, begin - (s0 < 0))) {
                        // TODO: Beam hits horizontal wall
                        break;
                    }

                    begin += sign0;
                }

                for (;;) {
                    if (end >= nexty1) {
                        // Nothing blocked
                        end = nexty1;
                        break;
                    }

                    // Check wall (x, end - sign1) <-> (x, end)
                    if (map.get(x, end - (s1 < 0))) {
                        // TODO: Beam hits horizontal wall
                        // To split the beam, do beam confinement
                        // on the other side with:
                        //     other.begin = end
                        //     other.end = end
                        //     other.nexty0 = end
                        //     other.nexty1 = nexty1
                        break;
                    }

                    end += sign1;
                }

                sweeps.push([x, segy0, segy1]);
                //sweeps.push([nextx, begin, begin + 0.1]);

                sweep3(
                    origin,
                    nextx,
                    begin,
                    end);

                if (map.get(x, begin - (s0 < 0))) {
                    visual.set(x, begin - (s0 < 0), 0xffff00ff);
                }

                if (map.get(x, end - (s1 < 0))) {
                    visual.set(x, end - (s1 < 0), 0xffff00ff);
                }
            }
            if (map.get(x, y) && !visual.get(x, y)) {
                visual.set(x, y, 0xffff00ff);
                console.log(x, y);
            }
            y0 = y + 1;
        }
    }
}

function sweep2(origin, x, y0, y1) {

    // (y0 % 1) / -s0
    var nextx = x + 1;
    var s0 = (y0 - origin[1]) / (x - origin[0]);
    var s1 = (y1 - origin[1]) / (x - origin[0]);
    var nexty0 = y0 + s0;
    var nexty1 = y1 + s1;
    
    // scan (x0, |y0|)-(x0, |nexty0|)

    var begin = Math.floor(y0);
    var sign0 = Math.sign(s0);
    for (;;) {
        if (sign0 > 0 ? begin >= nexty0 : begin <= nexty0) {
            // Nothing blocked
            begin = nexty0;
            break;
        }

        if (map.get(x, begin - (s0 < 0))) {
            // TODO: Beam hits horizontal wall
            break;
        }

        begin += sign0;
    }

    var end = Math.ceil(y1);
    var sign1 = Math.sign(s1);
    for (;;) {
        if (end >= nexty1) {
            // Nothing blocked
            end = nexty1;
            break;
        }

        if (map.get(x, end - (s1 < 0))) {
            // TODO: Beam hits horizontal wall
            break;
        }

        end += sign1;
    }

    for (var nexty = Math.floor(begin); nexty < Math.ceil(end); ++nexty) {
        if (map.get(nextx, nexty)) {
            var blockedy = nexty;
            if (begin < blockedy) {
                sweeps.push([nextx, begin, blockedy]);
                //sweeps.push([nextx, begin, begin + 0.1]);

                sweep2(
                    origin,
                    nextx,
                    begin,
                    blockedy);
            }
            begin = nexty + 1;
        }
    }

    if (begin < end) {
        sweeps.push([nextx, begin, end]);
        //sweeps.push([nextx, begin, begin + 0.1]);

        sweep2(
            origin,
            nextx,
            begin,
            end);
    }
}

function sweep(origin, x, y0, y1) {
    var start = y0;
    var xadj = 0;
    var stepy = 1;

    for (var y = Math.floor(y0); y <= Math.floor(y1); ++y) {

        if (map.get(x, y)) {
            // [start, iy] ok
            var end = y;
            if (start < end) {
                let nextx = x + 1;
                var nexty0 = start + (start - origin[1]) / (nextx - origin[0]);
                var nexty1 = end + (end - origin[1]) / (nextx - origin[0]);

                sweeps.push([nextx, nexty0, nexty1]);

                sweep(
                    origin,
                    nextx,
                    nexty0,
                    nexty1);
            }
            start = y + stepy;
        } else {
            visual.set(x, y, 0xffff00ff);
        }
    }

    var end = y1;
    if (start < end) {
        let nextx = x + 1;
        var nexty0 = start + (start - origin[1]) / (nextx - origin[0]);
        var nexty1 = end + (end - origin[1]) / (nextx - origin[0]);

        sweeps.push([nextx, nexty0, nexty1]);

        sweep(
            origin,
            nextx,
            nexty0,
            nexty1);
    }
}

function render_los(p) {
    var fov = 3;

    if (true) {
        var b = performance.now();
        var c = 0;
        for (var i = 0; i < 1000; ++i) {
            for (var a = -fov / 2; a < fov / 2; a += (fov / 1024.0)) {
                sweep5(p[0], p[1], Math.cos(a), Math.sin(a));
                sweep5(p[0], p[1], -Math.cos(a), Math.sin(a));
                ++c;
            }
        }
        console.log('p', performance.now() - b, c);
    }

    if (false) {
        var b = performance.now();
        var c = 0;
        for (var i = 0; i < 1000; ++i) {
            for (var a = -fov / 2; a < fov / 2; a += (fov / 1024.0)) {
                sweep4(p[0], p[1], Math.cos(a) * 0.95, Math.sin(a) * 0.95);
                ++c;
            }
        }
        console.log('p', performance.now() - b, c);
    }

    if (false) {
        var s0 = mat.vangle2(-fov / 2);
        var s1 = mat.vangle2(fov / 2);

        var x = (p[0] | 0) + 1;

        var firstStep = (x - p[0]) / Math.abs(s0[0]);
        console.log('firstStep', firstStep);
        var p0 = mat.vadd2(p, mat.vscale2(s0, firstStep));
        var p1 = mat.vadd2(p, mat.vscale2(s1, firstStep));
        //var p0 = mat.vadd2(p, mat.vscale2(s0, 0.01));
        //var p1 = mat.vadd2(p, mat.vscale2(s1, 0.01));
        //s0 = mat.vadd2(s0, mat.vscale2(s0, firstStep));
        //s1 = mat.vadd2(s1, mat.vscale2(s1, firstStep));
        //s0 = mat.vscale2(s0, 1 / Math.abs(s0[0]));
        //s1 = mat.vscale2(s1, 1 / Math.abs(s1[0]));

        console.log(x);

        sweep3(p, x, p0[1], p1[1]);
    }

    //sweep2(p, p[0] + 0.01, p0[1], p1[1]);
    if (false) {
        console.log(s0, s1);
        console.log(x);

        for (; x < 32;) {
            var ip0 = mat.vfloor2(p0);
            var ip1 = mat.vfloor2(p1);

            for (var y = ip0[1]; y <= ip1[1]; ++y) {
                if (y >= 0 && y < 32) {
                    visual.set(x, y, 0xffff00ff);
                }
            }

            p0 = mat.vadd2(p0, s0);
            p1 = mat.vadd2(p1, s1);
            ++x;
        }
    }

    visual.set(p[0] | 0, p[1] | 0, 0xff0000ff);
    console.log(p[0] | 0);
}

render_los([15.5, 15.5]);

async function getText(p) {
    let resp = await p;
    let txt = await resp.text();
    return txt;
}

export async function startGame() {
    let sdfShader;
    let imgShader;
    let shader3d, keyShader;
    let gl;
    let canvas = document.getElementById("g");
    
    render.init(canvas);
    gl = render.gl;

    let fs3d = await getText(fetch('./fs3d.glsl'));
    let key = await getText(fetch('./key.glsl'));
    let fs = await getText(fetch('./fs2d.glsl'));
    let fsImg = await getText(fetch('./fs_img.glsl'));
    let vs3d = await getText(fetch('./vs3d.glsl'));

    sdfShader = render.createShaderProgram(render.basicVs, fs);
    imgShader = render.createShaderProgram(render.basicVs, fsImg);
    shader3d = render.createShaderProgram(vs3d, fs3d, true);
    keyShader = render.createShaderProgram(vs3d, key, true);
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

    let pointTex = genTex(new Uint32Array(128*128), render.GL_RGBA, getPointColor);
    let whiteTex = genTex(new Uint32Array(1), render.GL_RGBA, () => 0xffffffff);
    //let pointTex = genTex(new Uint32Array(512*512), GL_RGBA, rkey);

    var rotx = 0.0;
    var roty = 0.0;
    var posx = 0.0;
    var posy = 0.0;

    canvas.onmousemove = function(e) {
        if (document.pointerLockElement === canvas) {
            rotx -= e.movementY / 500;
            roty -= e.movementX / 500;
        }
    }

    var keys = [];

    window.onkeydown = function(e) {
        keys[e.keyCode] = 1;
        console.log(e.keyCode);
    }
    window.onkeyup = function(e) {
        keys[e.keyCode] = 0;
    }

    // 71 up
    // 83 down
    // 68 left
    // 84 right

    let t = 10.0;
    function update() {
        window.requestAnimationFrame(function(currentTime) {
            update();
            render.updateWindow(canvas);
            gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
            gl.clear(render.GL_COLOR_BUFFER_BIT);

            {
                let walkSpeed = 40;
                if (keys[71]) {
                    posx += Math.sin(roty) * walkSpeed;
                    posy += Math.cos(roty) * walkSpeed;
                } else if (keys[83]) {
                    posx -= Math.sin(roty) * walkSpeed;
                    posy -= Math.cos(roty) * walkSpeed;
                }

                if (keys[68]) {
                    posx += Math.cos(roty) * walkSpeed;
                    posy -= Math.sin(roty) * walkSpeed;
                } else if (keys[84]) {
                    posx -= Math.cos(roty) * walkSpeed;
                    posy += Math.sin(roty) * walkSpeed;
                }
                //t += 5;
                //setView3d(rotx / 10, 1.0);
                render.setView3d(roty, rotx, posx, posy, 1.0);
                //render.activateShader3d(shader3d);
                render.activateShader3d(shader3d);

                render.color(0xff770077);
                for (var x = -10000; x < 10000; x += 512 * 2 * 1.5) {
                    //render.img3d(pointTex, x, -256 * 4, -2500, 512 * 2, 512 * 4, 0, 0, 1, 1);
                    var z = -2500;
                    render.wall3d(pointTex, x, z, x + 512 * 2, z, -256 * 4, 256 * 4, 0, 0, 1, 1);
                }

                render.flush3d();
            }

            if (false) {
                render.setView(0, 0, 1, 0, 1.0);
                //t += 0.01;
                render.activateShader(sdfShader);
                //img_simple(Math.random(), Math.random(), 0xffffffff);
                render.color(0xffffffff);
                //img(pointTex, -64.0, -64.0, 128, 128, 0, 0, 1, 1);
                //img(pointTex, -128.0, -128.0, 256, 256, 0, 0, 1, 1);
                render.img(pointTex, -256, -256, 512, 512, 0, 0, 1, 1);
                render.flush();

                render.activateShader(imgShader);

                let debugx = -512, debugy = 200;
                let cellsize = 16;
                for (var y = 0; y < 32; ++y) {
                    for (var x = 0; x < 32; ++x) {
                        if (visual.get(x, y)) {
                            render.color(visual.get(x, y));
                        } else if (map.get(x, y)) {
                            render.color(0xffffffff);
                        } else {
                            render.color(0xff000000);
                        }

                        if (false) {
                            render.img(whiteTex,
                                debugx + x * cellsize + 1,
                                debugy + y * cellsize + 1,
                                cellsize - 2, cellsize - 2,
                                0, 0, 1, 1);
                        } else {
                            render.img(whiteTex,
                                debugx + x * cellsize,
                                debugy + -y * cellsize,
                                cellsize, -cellsize,
                                0, 0, 1, 1);
                        }
                    }
                }

                for (var s of sweeps) {
                    render.color(0xff00ff00);
                    render.img(whiteTex,
                        debugx + s[0] * cellsize - 2,
                        debugy + -s[1] * cellsize,
                        2,
                        -(s[2] - s[1]) * cellsize,
                        0, 0, 1, 1);
                }
                
                render.flush();
            }
        });
    }

    update();
}
