(function(window){
var main = [`vec3 a = normalize(vec3(m.x, n.y, m.z));
  float b = a.y * -.5 + .5;
  gl_FragColor = vec4(vec3(i(a)) + mix(mix(vec3(0), vec3(.2, .1, .43), b + .3), vec3(.08, .61, .83), b * 1.5 - .4), 1);
}

`,`vec3 a = normalize(vec3(m.x, -n.y, m.z));
  float b = a.y * -.5 + .5;
  gl_FragColor = vec4(vec3(i(a)) + mix(mix(vec3(0), vec3(.2, .1, .43), b + .3), vec3(.08, .61, .83), b * 1.5 - .4), 1);
}

`,`gl_FragColor = texture2D(k, n) * o;
}

`,`gl_FragColor = vec4(o.rgb, .5);
}

`,`vec2 c = n.xy * 5., b = step(2. * abs(fract(c - .5) - .5), vec2(.8));
  float a = 1. - min(b.x, b.y);
  gl_FragColor = vec4(a * .1, a, a, 1);
}

`,`float a = 1., b = 2.;
  if (a == 1.) a = 3.; else {
    a = 4.;
    b = 5.;
    return z;
  }
  gl_FragColor = vec4(1, 1, 1, 1 == 1 ? 1 : 1);
}

`].map(function (a) { return `varying vec2 n;
varying vec4 o;
varying vec3 m;
uniform sampler2D k;
float g(vec2 a) {
  return fract(sin(dot(a.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float d(vec3 c) {
  float e = 1. / 700.;
  vec2 a = c.xy * (.5 / e) / max(.001, abs(c.z));
  float p = 20., f = 0.;
  a = floor(a);
  if (g(a.xy * e) > .997) {
    float b = g(a.xy * .5), j = 0.;
    f += b * (.3 * sin(j * 5. * (b * 5.) + b) + .7) * 1.5;
  }
  return f * abs(c.z);
}

mat3 l(float a, float b) {
  return mat3(cos(a), 0, -sin(a), 0, 1, 0, sin(a), 0, cos(a)) * mat3(1, 0, 0, 0, cos(b), sin(b), 0, -sin(b), cos(b));
}

float h(vec3 a) {
  return d(a.xyz) + d(a.yzx) + d(a.zxy);
}

float i(vec3 a) {
  float c = 3.1415927, b = c / 180.;
  return h(a) + h(l(45. * b, 45. * b) * a);
}

void main() {
  ` + a; });
var vertex = `varying vec3 m;
varying vec2 n;
varying vec4 o;
attribute vec3 a;
attribute vec2 c;
attribute vec4 d;
uniform float e;
uniform mat3 f;
uniform mat3 g;
void main() {
  m = a;
  n = c;
  o = d;
  vec3 b = g * f * a;
  gl_Position = mat4(1.83, 0, 0, 0, 0, 1.83 * e, 0, 0, 0, 0, -1, -1, 0, 0, -2, 0) * vec4(b.xyz, 1);
}

`;
var $bin=("test").split('').map(x=>x.charCodeAt(0));function trans(mat, x, y$0) {
  mat[6] += mat[0] * x + mat[3] * y$0;
  mat[7] += mat[1] * x + mat[4] * y$0;
}

function scale(mat, x, y$0) {
  mat[0] *= x;
  mat[1] *= x;
  mat[3] *= y$0;
  mat[4] *= y$0;
}

function rotvec(mat, x, y$0) {
  var a = mat[0];
  var b = mat[1];
  var c = mat[3];
  var d = mat[4];
  mat[0] = a * x + c * -y$0;
  mat[1] = b * x + d * -y$0;
  mat[3] = a * y$0 + c * x;
  mat[4] = b * y$0 + d * x;
}

function vsub2([x0, y0], [x1, y1]) {
  return [x0 - x1, y0 - y1];
}

function vadd2([x0, y0], [x1, y1]) {
  return [x0 + x1, y0 + y1];
}

function vmul2([x0, y0], s) {
  return [x0 * s, y0 * s];
}

function vrotate([x0, y0], [x1, y1]) {
  return [x0 * x1 - y0 * y1, x0 * y1 + y0 * x1];
}

function vnormalize2([x, y$0]) {
  var l = Math.hypot(x, y$0);
  return [x / l, y$0 / l];
}



var gl;
var canvas_w;
var canvas_h;
var viewTrans;
var viewTrans2;
var GL_VERTEX_SHADER = 35633;
var GL_FRAGMENT_SHADER = 35632;
var GL_ELEMENT_ARRAY_BUFFER = 34963;
var GL_ARRAY_BUFFER = 34962;
var GL_TEXTURE0 = 33984;
var GL_ONE = 1;
var GL_ZERO = 0;
var GL_SRC_ALPHA = 770;
var GL_ONE_MINUS_SRC_ALPHA = 771;
var GL_DST_ALPHA = 772;
var GL_ONE_MINUS_DST_ALPHA = 773;
var GL_DST_COLOR = 774;
var GL_ONE_MINUS_DST_COLOR = 775;
var GL_BLEND = 3042;
var GL_RGBA = 6408;
var GL_LUMINANCE = 6409;
var GL_TRIANGLES = 4;
var GL_TRIANGLE_STRIP = 5;
var GL_UNSIGNED_BYTE = 5121;
var GL_UNSIGNED_SHORT = 5123;
var GL_FLOAT = 5126;
var GL_STATIC_DRAW = 35044;
var GL_DYNAMIC_DRAW = 35048;
var GL_COMPILE_STATUS = 35713;
var GL_LINK_STATUS = 35714;
var GL_TEXTURE_2D = 3553;
var GL_TEXTURE_WRAP_S = 10242;
var GL_TEXTURE_WRAP_T = 10243;
var GL_TEXTURE_MAG_FILTER = 10240;
var GL_TEXTURE_MIN_FILTER = 10241;
var GL_NEAREST = 9728;
var GL_LINEAR = 9729;
var GL_CLAMP_TO_EDGE = 33071;
var GL_COLOR_BUFFER_BIT = 16384;
var GL_DEPTH_TEST = 2929;
var GL_FRAMEBUFFER = 36160;
var GL_COLOR_ATTACHMENT0 = 36064;
var basicVs = vertex;
var VERTEX_SIZE = 4 * 3 + 4 * 2 + 4 * 4;
var MAX_BATCH = 10922;
var VERTICES_PER_QUAD = 6;
var QUAD_SIZE_IN_WORDS = VERTEX_SIZE * VERTICES_PER_QUAD / 4;
var VERTEX_DATA_SIZE = VERTEX_SIZE * (MAX_BATCH * 6 + 32);
var VBO;
var currentTexture;
var vertexData = [];
var locPos = 0;
var locUV = 1;
var locColor = 2;
var col = 4294967295;
function initGl(canvas) {
  gl = canvas.getContext("webgl");
  canvas_w = canvas.width;
  canvas_h = canvas.height;
  gl.clearColor(0, 0, 0, 1);
}

function createBuffer(bufferType, size, usage) {
  var buffer = gl.createBuffer();
  gl.bindBuffer(bufferType, buffer);
  gl.bufferData(bufferType, size, usage);
  return buffer;
}

function createTexture(image, side, ty) {
  var texture = gl.createTexture();
  gl.bindTexture(GL_TEXTURE_2D, texture);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  gl.texImage2D(GL_TEXTURE_2D, 0, ty, side, side, 0, ty, GL_UNSIGNED_BYTE, new Uint8Array(image.buffer));
  console.log("texImage");
  return texture;
}

function createFramebufferTexture(w, h) {
  var texture = gl.createTexture();
  gl.bindTexture(GL_TEXTURE_2D, texture);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
  gl.texParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  var ty = GL_RGBA;
  gl.texImage2D(GL_TEXTURE_2D, 0, ty, w, h, 0, ty, GL_UNSIGNED_BYTE, null);
  checkErr();
  console.log(texture);
  var fb = gl.createFramebuffer();
  gl.bindFramebuffer(GL_FRAMEBUFFER, fb);
  checkErr();
  gl.framebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, texture, 0);
  checkErr();
  console.log(fb);
  fb.texture = texture;
  return fb;
}

function deleteFramebufferTexture(fb) {
  gl.deleteTexture(fb.texture);
  gl.deleteFramebuffer(fb);
}

function bindFramebufferTexture(fb) {
  gl.bindFramebuffer(GL_FRAMEBUFFER, fb);
}

function unbindFramebufferTexture() {
  gl.bindFramebuffer(GL_FRAMEBUFFER, null);
}

function setViewTransform(shader) {
  gl.uniform1f(gl.getUniformLocation(shader, "e"), canvas_w / canvas_h);
  gl.uniformMatrix3fv(gl.getUniformLocation(shader, "f"), 0, viewTrans);
  gl.uniformMatrix3fv(gl.getUniformLocation(shader, "g"), 0, viewTrans2);
}

function color(c) {
  col = c;
}

function setView2([aimx, aimy], [aimvx, aimvy]) {
  viewTrans = [-aimy, 0, -aimx, 0, 1, 0, aimx, 0, -aimy];
  viewTrans2 = [1, 0, 0, 0, aimvx, aimvy, 0, -aimvy, aimvx];
}

function setView(x, y$0, rotx, roty, zoom) {
  var ratio = canvas_h / canvas_w;
  viewTrans = [1, 0, 0, 0, 1, 0, 0, 0, 1];
  scale(viewTrans, 2 / (1024 * zoom), 2 / (1024 * zoom * ratio));
  rotvec(viewTrans, rotx, roty);
  trans(viewTrans, -x, -y$0);
}

function flush() {
  if (vertexData.length) gl.vertexAttribPointer(locPos, 3, GL_FLOAT, 0, VERTEX_SIZE, 0), gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE, 12), gl.vertexAttribPointer(locColor, 4, GL_FLOAT, 0, VERTEX_SIZE, 20), gl.bufferSubData(GL_ARRAY_BUFFER, 0, new Float32Array(vertexData)), gl.drawArrays(GL_TRIANGLES, 0, vertexData.length / (VERTEX_SIZE / 4)), vertexData.length = 0;
  currentTexture = null;
}

function prism(texture, fx, fy, fz, ex, ey, ez) {
  var e1x = fx + ex;
  var e1y = fy + ey;
  var e2x = fx - ey;
  var e2y = fy + ex;
  var e3x = fx - ex;
  var e3y = fy - ey;
  var e4x = fx + ey;
  var e4y = fy - ex;
  var mz = fz + ez;
  var cz = fz + ez + ez;
  var abgr = col;
  if (texture !== currentTexture) flush(), currentTexture = texture, gl.bindTexture(GL_TEXTURE_2D, currentTexture);
  var a = (abgr >>> 24) / 255;
  var b = (abgr >> 16 & 255) / 255;
  var g = (abgr >> 8 & 255) / 255;
  var r = (abgr & 255) / 255;
  if (vertexData.push(fx, fz, fy, 0, 0, r, g, b, a, e1x, mz, e1y, 0, 0, r, g, b, a, e2x, mz, e2y, 0, 0, r, g, b, a, fx, fz, fy, 0, 0, r, g, b, a, e2x, mz, e2y, 0, 0, r, g, b, a, e3x, mz, e3y, 0, 0, r, g, b, a, fx, fz, fy, 0, 0, r, g, b, a, e3x, mz, e3y, 0, 0, r, g, b, a, e4x, mz, e4y, 0, 0, r, g, b, a, fx, fz, fy, 0, 0, r, g, b, a, e4x, mz, e4y, 0, 0, r, g, b, a, e1x, mz, e1y, 0, 0, r, g, b, a, fx, cz, fy, 0, 0, r, g, b, a, e1x, mz, e1y, 0, 0, r, g, b, a, e2x, mz, e2y, 0, 0, r, g, b, a, fx, cz, fy, 0, 0, r, g, b, a, e2x, mz, e2y, 0, 0, r, g, b, a, e3x, mz, e3y, 0, 0, r, g, b, a, fx, cz, fy, 0, 0, r, g, b, a, e3x, mz, e3y, 0, 0, r, g, b, a, e4x, mz, e4y, 0, 0, r, g, b, a, fx, cz, fy, 0, 0, r, g, b, a, e4x, mz, e4y, 0, 0, r, g, b, a, e1x, mz, e1y, 0, 0, r, g, b, a) >= MAX_BATCH * QUAD_SIZE_IN_WORDS) flush();
}

function wall3d(texture, fx0, fz0, fx1, fz1, fy, cy, u0, v0, u1, v1) {
  var x0 = fx0;
  var y0 = fy;
  var z0 = fz0;
  var x1 = fx1;
  var y1 = fy;
  var z1 = fz1;
  var x2 = fx1;
  var y2 = cy;
  var z2 = fz1;
  var x3 = fx0;
  var y3 = cy;
  var z3 = fz0;
  var abgr = col;
  if (texture !== currentTexture) flush(), currentTexture = texture, gl.bindTexture(GL_TEXTURE_2D, currentTexture);
  var a = (abgr >>> 24) / 255;
  var b = (abgr >> 16 & 255) / 255;
  var g = (abgr >> 8 & 255) / 255;
  var r = (abgr & 255) / 255;
  if (vertexData.push(x0, y0, z0, u0, v1, r, g, b, a, x1, y1, z1, u1, v1, r, g, b, a, x3, y3, z3, u0, v0, r, g, b, a, x1, y1, z1, u1, v1, r, g, b, a, x2, y2, z2, u1, v0, r, g, b, a, x3, y3, z3, u0, v0, r, g, b, a) >= MAX_BATCH * QUAD_SIZE_IN_WORDS) flush();
}

function activateShader(shader) {
  gl.useProgram(shader);
  setViewTransform(shader);
}

function compileShader(source, ty) {
  var shader = gl.createShader(ty);
  gl.shaderSource(shader, "#extension GL_OES_standard_derivatives:enable\nprecision lowp float;" + source);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, GL_COMPILE_STATUS)) console.log(gl.getShaderInfoLog(shader));
  return shader;
}

function bindAttribLocations(shader) {
  ["a", "c", "d"].map(function(name, i) {
    gl.bindAttribLocation(shader, i, name);
  });
}

function createShaderProgram(vsSource, fsSource) {
  var program = gl.createProgram();
  var vShader = compileShader(vsSource, GL_VERTEX_SHADER);
  var fShader = compileShader(fsSource, GL_FRAGMENT_SHADER);
  gl.attachShader(program, vShader);
  gl.attachShader(program, fShader);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, GL_LINK_STATUS)) console.log("Error linking shader program:"), console.log(gl.getProgramInfoLog(program));
  bindAttribLocations(program);
  return program;
}

function checkErr(v) {
  var err = gl.getError();
  if (err !== 0) console.log("error:", err), console.trace();
  return v;
}

function init(canvas) {
  initGl(canvas);
  gl.blendFuncSeparate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
  gl.enable(GL_BLEND);
  gl.enable(GL_DEPTH_TEST);
  gl.getExtension("OES_standard_derivatives");
  VBO = createBuffer(GL_ARRAY_BUFFER, VERTEX_DATA_SIZE, GL_DYNAMIC_DRAW);
  gl.enableVertexAttribArray(locPos);
  gl.enableVertexAttribArray(locUV);
  gl.enableVertexAttribArray(locColor);
  gl.activeTexture(GL_TEXTURE0);
}

function drawText(fontBits$0, whiteTex$0, text, x, y$0, z, dirx, dirz, diry, scale$0) {
  for (var i = 0;i < text.length;++i) {
    var ch = text.charCodeAt(i);
    var id = 200;
    if (ch >= 48 && ch <= 57) id = ch - 48;
 else if (ch >= 65 && ch <= 90) id = ch - 65 + 10;
    var step = scale$0;
    for (var column = 0;column < 3;++column) {
      for (var row = 0;row < 5;++row) {
        var pos = (4 - row) * 36 * 3 + id * 3 + column;
        if (fontBits$0[pos >> 3] >> (pos & 7) & 1) {
          var abs_column = i * 4 + column;
          wall3d(whiteTex$0, z + dirz * step * abs_column, x + dirx * step * abs_column, z + dirz * step * (abs_column + 1), x + dirx * step * (abs_column + 1), y$0 + diry * (row + 1) * step, y$0 + diry * row * step, 0, 0, 1, 1);
        }
      }
    }
  }
}


var W = 32;
var H = 32;
var W_SHIFT = 5;
var H_SHIFT = 5;
var MAP_SECTION_SIZE = W * H;
var MAP_SIZE = MAP_SECTION_SIZE * 3;
var SECOND = 60;
var MINUTE = 60 * SECOND;
var TIME_IN_MINUTES = 1;
var TIME_IN_SECONDS = TIME_IN_MINUTES * 60;
var INTERVAL = 10 * SECOND;
var FRAMES = TIME_IN_MINUTES * MINUTE;
var STATES = FRAMES / INTERVAL;
var VISUAL_W = 512;
var VISUAL_H = 512;
var VISUAL_SIZE = VISUAL_W * VISUAL_H * 3;
var PLAYER_RADIUS = 0.3;
var WALK_SPEED = 0.1;
var CLOAK_TIME = SECOND * 30;
var RESET_TIMER = SECOND * 5;
var wall_render = 0;
var cell_render = 0;
var ray_steps = 0;
console.log("INTERVAL", INTERVAL);
console.log("STATES", STATES);
console.log("FRAMES", FRAMES);
var WALL_FLOOR = 1;
var WALL_BLOCK0 = 2;
var WALL_BLOCK_MAX = WALL_BLOCK0 + 8;
var WALL_TELEPORT0 = WALL_BLOCK_MAX;
var WALL_TELEPORT_MAX = WALL_TELEPORT0 + 5 * 2;
var WALL_SWITCH0 = WALL_TELEPORT_MAX;
var WALL_SWITCH_MAX = WALL_SWITCH0 + 10;
var WALL_DOOR0 = WALL_SWITCH_MAX;
var WALL_DOOR_MAX = WALL_DOOR0 + 10;
var WALL_WINDOW = WALL_DOOR_MAX;
var WALL_CONFLICT = 255;
console.log("WALL_SWITCH0", WALL_SWITCH0);
console.log("WALL_DOOR0", WALL_DOOR0);
console.log("WALL_TELEPORT0", WALL_TELEPORT0);
console.log("WALL_BLOCK0", WALL_BLOCK0);
console.log("WALL_WINDOW", WALL_WINDOW);
var CELL_FLOOR = 1;
var CELL_PLAYER = 2;
var CELL_EXIT = 3;
var CELL_KEY0 = 4;
var CELL_KEY_MAX = CELL_KEY0 + 5;
var CELL_CLOAK = CELL_KEY_MAX;
var ACTION_CHANGE = 0;
var ACTION_MOVE = 1;
function is_block(cell) {
  return cell >= WALL_BLOCK0 && cell < WALL_BLOCK_MAX;
}

function is_switch(cell) {
  return cell >= WALL_SWITCH0 && cell < WALL_SWITCH_MAX;
}

function is_door(cell) {
  return cell >= WALL_DOOR0 && cell < WALL_DOOR_MAX;
}

function is_wall(cell) {
  return is_block(cell) || is_switch(cell) || cell === WALL_WINDOW;
}

function is_teleport(cell) {
  return cell >= WALL_TELEPORT0 && cell < WALL_TELEPORT_MAX;
}

function is_key(cell) {
  return cell >= CELL_KEY0 && cell < CELL_KEY_MAX;
}

function is_pickable(cell) {
  return is_key(cell);
}

function map_create() {
  return new Uint8Array(MAP_SIZE);
}

function set_cell(map, [x, y$0], what) {
  var prev = map[y$0 << W_SHIFT | x];
  map[y$0 << W_SHIFT | x] = what;
  return prev;
}

function set_wall(map, [x, y$0], wall_dir, what) {
  var prev = map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
  map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
  return prev;
}

function set_wall_(map, x, y$0, wall_dir, what) {
  var prev = map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
  map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
  return prev;
}

function set_cell_(map, x, y$0, what) {
  var prev = map[y$0 << W_SHIFT | x];
  map[y$0 << W_SHIFT | x] = what;
  return prev;
}

function set_observed_wall(world, time, x, y$0, wall_dir, what) {
  var index = time * MAP_SIZE + (y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir;
  var prev = world.observed_map[index];
  if (what === 0) {
    return prev;
  } else if (prev && prev !== what) {
    if (prev !== WALL_CONFLICT) world.conflicts += 1, world.observed_map[index] = WALL_CONFLICT;
  } else {
    world.observed_map[index] = what;
  }
  return prev;
}

function get_observed_wall(world, time, x, y$0, wall_dir) {
  var index = time * MAP_SIZE + (y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir;
  return world.observed_map[index];
}

function set_observed_cell(world, time, x, y$0, what) {
  var index = time * MAP_SIZE + (y$0 << W_SHIFT | x);
  var prev = world.observed_map[index];
  if (what === 0) {
    return prev;
  } else if (prev && prev !== what) {
    if (prev !== WALL_CONFLICT) world.conflicts += 1, world.observed_map[index] = WALL_CONFLICT;
  } else {
    world.observed_map[index] = what;
  }
  return prev;
}

function get_observed_cell(world, time, x, y$0) {
  var index = time * MAP_SIZE + (y$0 << W_SHIFT | x);
  return world.observed_map[index];
}

function get_cell(map, [x, y$0]) {
  return map[y$0 << W_SHIFT | x];
}

function get_cell_(map, x, y$0) {
  return map[y$0 << W_SHIFT | x];
}

function get_wall(map, [x, y$0], wall_dir) {
  return map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
}

function get_wall_(map, x, y$0, wall_dir) {
  return map[(y$0 << W_SHIFT | x) + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
}

function loc_from_coords([x, y$0]) {
  return y$0 << W_SHIFT | x;
}

function loc_from_coords_(x, y$0) {
  return y$0 << W_SHIFT | x;
}

function set_loc(map, loc, what) {
  map[loc] = what;
}

function get_loc(map, loc) {
  return map[loc];
}

function set_wall_loc(map, loc, wall_dir, what) {
  map[loc + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
}

function get_wall_loc(map, loc, wall_dir) {
  return map[loc + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
}

function state_create() {
  var self = {map: map_create(), visual: map_create(), options: {}, prev_switches: 0, switches: 0, prev_outputs: 0, outputs: 0};
  self.options[WALL_TELEPORT0] = {};
  self.options[WALL_TELEPORT0]["pos_diff"] = [-31, 0];
  self.options[WALL_TELEPORT0]["state_diff"] = -1;
  self.options[WALL_BLOCK0 + 1] = {};
  self.options[WALL_BLOCK0 + 1]["text"] = "OH HELLO";
  for (var y$0 = 0;y$0 < H;++y$0) {
    for (var x = 0;x < W;++x) {
      set_wall(self.map, [x, y$0], false, WALL_FLOOR);
      set_wall(self.map, [x, y$0], true, WALL_FLOOR);
      if (x === 0 || x === W - 1 || y$0 === 0 || y$0 === H - 1) {
        if (y$0 === 0 || y$0 === H - 1) set_wall(self.map, [x, y$0], true, WALL_BLOCK0);
        if (x === 0 || x === W - 1) set_wall(self.map, [x, y$0], false, WALL_BLOCK0);
      }
      set_cell(self.map, [x, y$0], WALL_FLOOR);
    }
  }
  console.log(self.map);
  return self;
}

function state_preprocess(state, frame, time) {
  for (var i = 0;i < MAP_SIZE;++i) {
    var cell = state.map[i];
    if (is_teleport(cell)) {
      var opt = state.options[cell & ~1];
      var sign = cell & 1 ? -1 : 1;
      var state_diff = sign * opt["state_diff"];
      var new_time = time + state_diff * INTERVAL;
      if (new_time >= FRAMES || new_time < 0) cell = WALL_BLOCK0;
    } else if (is_door(cell)) {
      var door_id = cell - WALL_DOOR0;
      if (!(state.outputs & 1 << door_id)) cell = WALL_BLOCK0;
    }
    state.visual[i] = cell;
  }
  Object.values(frame.player_states).forEach(function(s) {
    if (s.cloak_timer < 0) set_cell(state.visual, s.pos, CELL_PLAYER);
  });
}

function state_update(state, frame, time) {
  var current = frame.switches;
  var change_on = ~state.prev_switches & current;
  var any_change = state.prev_switches !== current;
  state.prev_switches = current;
  if (any_change) console.log("switches:", current);
  state.prev_outputs = state.outputs;
  state.outputs = current & current >> 1;
  if (state.prev_outputs !== state.outputs) console.log("outputs changed:", state.outputs);
  frame.actions.forEach(function([ty, a, b, c]) {
    if (ty === ACTION_CHANGE) set_loc(state.map, a, b);
 else if (ty === ACTION_MOVE) {
    }
  });
}

function time_in_state(world, s) {
  return (world.offset + s * INTERVAL) % FRAMES;
}

function copy_to(from, to) {
  if (from.constructor === Uint8Array) to = from.slice();
 else if (Array.isArray(from)) {
    to = to || [];
    to.length = from.length;
    var i = 0;
    for (;i < from.length;) to[i] = copy_to(from[i], to[i]), i += 1;
  } else if (typeof(from) === "object") to = to || {}, Object.keys(from).forEach(function(k) {
    to[k] = copy_to(from[k], to[k]);
  });
 else {
    to = from;
  }
  return to;
}

function clone(obj) {
  return copy_to(obj, {});
}

function world_reset(w) {
  w.frames = Array(FRAMES).fill(0).map(function() {
    return {switches: 0, actions: [], player_states: {}};
  });
  w.observed_map.fill(0);
  w.reset_on = -1;
  w.states = [];
  w.offset = 0;
  w.conflicts = 0;
  w.current_state = 0;
  w.current_player = 0;
  w.current_player_state = {aim: [1, 0], pos: [15.5, 15.5], cloak_timer: -1, inventory: []};
  w.visual = new Uint8Array(VISUAL_SIZE);
  w.states.push(clone(w.first_state));
  var time = 0;
  for (var i = 1;i < STATES;++i) {
    var state = clone(w.states[w.states.length - 1]);
    for (var j = 0;j < INTERVAL;++j) state_update(state, w.frames[time], time), time += 1;
    w.states.push(state);
  }
  console.log(w.states);
}

function world_create() {
  var w = {first_state: state_create(), observed_map: new Uint8Array(FRAMES * MAP_SIZE)};
  world_reset(w);
  return w;
}

function seen_wall(world, visual_wall_id) {
  var index = visual_wall_id + VISUAL_W * VISUAL_H + 2 * (VISUAL_W / 2) * (VISUAL_H / 2);
  var prev = world.visual[index];
  world.visual[index] = 1;
  return prev;
}

function seen_cell(world, visual_cell_id) {
  var index = visual_cell_id + 2 * (VISUAL_W / 2) * (VISUAL_H / 2);
  var prev = world.visual[index];
  world.visual[index] = 1;
  return prev;
}

function sweep5(world, s, originx, originy, dirx, diry, max_len, renders) {
  var iposx = originx | 0;
  var iposy = originy | 0;
  var piposx = iposx;
  var piposy = iposy;
  var blocked = true;
  var safe_counter = 0;
  var length = 0;
  var cur_wall;
  var visual_wall_id = 0;
  var div_start_x;
  var div_start_y;
  var north_south;
  var s_visual = world.states[s].visual;
  var inv_divx = 1 / dirx;
  var inv_divy = 1 / diry;
  for (;;) {
    ray_steps += 1;
    if (iposx < 0 || iposy < 0 || iposx >= W || iposy >= H) {
      break;
    }
    var visual_cell_id = visual_wall_id >> 1;
    if (renders && !seen_cell(world, visual_cell_id)) {
      cell_render += 1;
      var cur_cell = get_cell_(s_visual, iposx, iposy);
      var time = time_in_state(world, s);
      if (cur_cell === CELL_PLAYER) Object.values(world.frames[time].player_states).forEach(function(p) {
        if (iposx === (p.pos[0] | 0) && iposy === (p.pos[1] | 0) && p.cloak_timer < 0) renders.lines.push({fromx: p.pos[0] - originx, fromy: p.pos[1] - originy, tox: p.pos[0] + p.aim[0] - originx, toy: p.pos[1] + p.aim[1] - originy, color: 4278190335});
      });
 else if (is_key(cur_cell)) {
        var p = [iposx + 0.5, iposy + 0.5];
        renders.lines.push({fromx: iposx - originx, fromy: iposy + 0.5 - originy, tox: iposx + 1 - originx, toy: iposy + 0.5 - originy, color: 4278255615});
      } else if (cur_cell === CELL_EXIT) {
        var p = [iposx + 0.5, iposy + 0.5];
        renders.lines.push({fromx: iposx - originx, fromy: iposy + 0.5 - originy, tox: iposx + 1 - originx, toy: iposy + 0.5 - originy, color: 4294967040});
      } else if (cur_cell === CELL_CLOAK) {
        var p = [iposx + 0.5, iposy + 0.5];
        renders.lines.push({fromx: iposx - originx, fromy: iposy + 0.5 - originy, tox: iposx + 1 - originx, toy: iposy + 0.5 - originy, color: 4289374890});
      }
      set_observed_cell(world, time, iposx, iposy, cur_cell);
    }
    var dir_positivex = dirx >= 0 ? 1 : 0;
    var dir_positivey = diry >= 0 ? 1 : 0;
    var leapx = iposx - originx + dir_positivex;
    var leapy = iposy - originy + dir_positivey;
    var length_we = leapx * inv_divx;
    var length_ns = leapy * inv_divy;
    north_south = length_ns < length_we;
    piposx = iposx;
    piposy = iposy;
    if (north_south) iposy += Math.sign(diry), visual_wall_id += Math.sign(diry) * (VISUAL_W * 2), div_start_x = iposx, div_start_y = piposy + dir_positivey, length = length_ns;
 else {
      iposx += Math.sign(dirx);
      visual_wall_id += Math.sign(dirx) * 2;
      div_start_x = piposx + dir_positivex;
      div_start_y = iposy;
      length = length_we;
    }
    cur_wall = get_wall_(s_visual, div_start_x, div_start_y, north_south);
    if (length >= max_len) {
      length = max_len;
      blocked = false;
      break;
    }
    if (is_wall(cur_wall)) {
      var angled_visual_wall_id = visual_wall_id + north_south;
      var div_endx = div_start_x + north_south;
      var div_endy = div_start_y + !north_south;
      if (renders && !seen_wall(world, angled_visual_wall_id)) {
        wall_render += 1;
        var renderer = cur_wall === WALL_WINDOW ? renders.sky : renders.lines;
        var ax = div_start_x - originx;
        var ay = div_start_y - originy;
        var bx = div_endx - originx;
        var by = div_endy - originy;
        var swap = north_south ? dir_positivey : !dir_positivex;
        if (swap) {
          var tempx = ax;
          var tempy = ay;
          ax = bx;
          ay = by;
          bx = tempx;
          by = tempy;
        }
        renderer.push({fromx: ax, fromy: ay, tox: bx, toy: by, color: is_switch(cur_wall) ? 4294967040 : 4294967295, s: s});
        var opt = world.states[s].options[cur_wall];
        if (opt && opt["text"]) renders.portal_texts.push({fromx: ax, fromy: ay, tox: bx, toy: by, color: 4294901760, text: opt["text"]});
        var time = time_in_state(world, s);
        set_observed_wall(world, time, div_start_x, div_start_y, north_south, cur_wall);
      }
      break;
    } else if (is_teleport(cur_wall)) {
      var angled_visual_wall_id = visual_wall_id + north_south;
      var div_endx = div_start_x + north_south;
      var div_endy = div_start_y + !north_south;
      var opt = world.states[s].options[cur_wall & ~1];
      var sign = cur_wall & 1 ? -1 : 1;
      var pos_diffx = sign * opt["pos_diff"][0];
      var pos_diffy = sign * opt["pos_diff"][1];
      var state_diff = sign * opt["state_diff"];
      if (renders && !seen_wall(world, angled_visual_wall_id)) {
        var ax = div_start_x - originx;
        var ay = div_start_y - originy;
        var bx = div_endx - originx;
        var by = div_endy - originy;
        renders.lines.push({fromx: ax, fromy: ay, tox: bx, toy: by, color: 4294901760});
        var swap = north_south ? dir_positivey : !dir_positivex;
        if (swap) {
          var tempx = ax;
          var tempy = ay;
          ax = bx;
          ay = by;
          bx = tempx;
          by = tempy;
        }
        renders.portal_texts.push({fromx: ax, fromy: ay, tox: bx, toy: by, color: 4294901760});
      }
      iposx = iposx + pos_diffx - dir_positivex;
      iposy = iposy + pos_diffy - dir_positivey;
      originx += pos_diffx;
      originy += pos_diffy;
      var linedx = dirx * length;
      var linedy = diry * length;
      originx -= originx + linedx;
      originy -= originy + linedy;
      iposx = iposx + (dirx >= 0 ? 1 : 0);
      iposy = iposy + (diry >= 0 ? 1 : 0);
      s = (s + STATES + state_diff) % STATES;
      s_visual = world.states[s].visual;
    }
  }
  return {origin: [originx, originy], blocked: blocked, end_s: s, end_pos: vadd2([originx, originy], vmul2([dirx, diry], length)), end_ipos: [iposx, iposy], end_div_start: [div_start_x, div_start_y], end_ns: north_south, end_wall: cur_wall, end_length: length};
}

function sweep(world, origin, aim, renders) {
  world.visual.fill(0);
  var fov = 1;
  var fov_vec = [Math.cos(fov / 2), Math.sin(fov / 2)];
  var a = vrotate(aim, [fov_vec[0], -fov_vec[1]]);
  var b = vsub2(vrotate(aim, fov_vec), a);
  var i = 0;
  var rays = 512;
  for (;i < rays;) {
    var m = vadd2(a, vmul2(b, i / rays));
    m = vmul2(m, 1 / Math.hypot(m[0], m[1]));
    sweep5(world, world.current_state, origin[0], origin[1], m[0], m[1], 64, renders);
    i += 1;
  }
}

function current_player_time(world) {
  return time_in_state(world, world.current_state);
}

function world_update(world, action, renders, paused$0) {
  if (!paused$0) {
    for (var s = 0;s < STATES;++s) {
      var time = time_in_state(world, s);
      state_update(world.states[s], world.frames[time], time);
      if (time === FRAMES - 1) {
        if (world.current_state === s) world.current_player += 1;
        world.states[s] = clone(world.first_state);
      }
    }
  }
  var switch_ons = 0;
  world.current_player_state.aim = action.aim;
  if (action.act) {
    var act_result = sweep5(world, world.current_state, world.current_player_state.pos[0], world.current_player_state.pos[1], action.aim[0], action.aim[1], 1);
    if (act_result.blocked) {
      if (is_switch(act_result.end_wall)) {
        var opt = world.states[world.current_state].options[act_result.end_wall];
        var ok = false;
        if (opt && opt.required_key) {
          if (world.current_player_state.inventory[opt.required_key]) world.current_player_state.inventory[opt.required_key] -= 1, ok = true;
        }
        if (ok) {
          var switch_id = act_result.end_wall - WALL_SWITCH0;
          switch_ons |= 1 << switch_id;
          console.log("switched");
        } else {
          console.log("denied");
        }
      }
    } else {
    }
  }
  if (action.walk[0] || action.walk[1]) {
    var pi = world.current_player_state.pos;
    var walk_dir = vnormalize2(action.walk);
    var result = sweep5(world, world.current_state, world.current_player_state.pos[0], world.current_player_state.pos[1], walk_dir[0], walk_dir[1], WALK_SPEED);
    if (!result.blocked) {
      var time = current_player_time(world);
      if (time_in_state(world, result.end_s) < time) console.log("going back to", time_in_state(world, result.end_s)), world.current_player += 1;
      if (time_in_state(world, result.end_s) !== time) world.current_player += 1;
      world.current_state = result.end_s;
      world.current_player_state.pos = result.end_pos;
    }
    var ang = 0;
    var min_dist = 10;
    var min_dir;
    for (;ang < Math.PI * 2;) {
      var dir = [Math.cos(ang), Math.sin(ang)];
      var res = sweep5(world, world.current_state, world.current_player_state.pos[0], world.current_player_state.pos[1], dir[0], dir[1], PLAYER_RADIUS);
      if (res.blocked && res.end_length < min_dist) min_dist = res.end_length, min_dir = dir;
      ang += 0.2;
    }
    if (min_dir) world.current_player_state.pos = vsub2(world.current_player_state.pos, vmul2(min_dir, PLAYER_RADIUS - min_dist));
    var at = loc_from_coords(world.current_player_state.pos);
    var on_cell = get_loc(world.states[world.current_state].map, at);
    if (on_cell === CELL_EXIT) {
      if (world.reset_on < 0) world.reset_on = world.offset + RESET_TIMER;
    } else if (on_cell === CELL_CLOAK) world.current_player_state.cloak_timer = CLOAK_TIME;
 else if (is_pickable(on_cell)) {
      var time = current_player_time(world);
      world.current_player_state.inventory[on_cell] |= 1;
      var to = CELL_FLOOR;
      set_loc(world.states[world.current_state].map, at, to);
      world.frames[time].actions.push([ACTION_CHANGE, at, to]);
      console.log("current inventory:", world.current_player_state.inventory);
    }
  }
  world.current_player_state.cloak_timer -= 1;
  if (!paused$0) world.offset += 1;
  var now = current_player_time(world);
  world.frames[now].player_states[world.current_player] = clone(world.current_player_state);
  world.frames[now].switches |= switch_ons;
  var origin = world.current_player_state.pos;
  var aim = world.current_player_state.aim;
  for (var s = 0;s < STATES;++s) {
    var time = time_in_state(world, s);
    state_preprocess(world.states[s], world.frames[time], time);
  }
  var prev_conflicts = world.conflicts;
  sweep(world, origin, aim, renders);
  if (world.conflicts !== prev_conflicts && !paused$0) {
    if (get_observed_cell(world, now, world.current_player_state.pos[0] | 0, world.current_player_state.pos[1] | 0) !== CELL_PLAYER) {
      for (var i = 0;i < 20;++i) {
        world.current_player_state.pos = [Math.random() * 31, Math.random() * 31];
        var dest = get_observed_cell(world, now, world.current_player_state.pos[0], world.current_player_state.pos[1]);
        if (dest === 0 || dest === CELL_PLAYER) {
          break;
        }
      }
      world.frames[now].player_states[world.current_player] = clone(world.current_player_state);
    } else {
    }
  }
  if (world.offset === world.reset_on) world_reset(world);
  if (!paused$0 && now % SECOND === 0) {
    var seconds = now / SECOND;
    console.log(seconds / 60 | 0, seconds % 60, "player ", world.current_player);
    console.log("renders:", wall_render, cell_render, ray_steps);
    wall_render = 0;
    cell_render = 0;
    ray_steps = 0;
  }
}



var draw;
var paused;
function get_wall_name(wall) {
  if (wall === WALL_FLOOR) {
    return "FLOOR";
  } else if (is_block(wall)) {
    var id = wall - WALL_BLOCK0;
    return "BLOCK " + id;
  } else if (is_teleport(wall)) {
    var id = wall - WALL_TELEPORT0;
    return "TELEPORT " + (id >> 1) + (id & 1 ? " REV" : "");
  } else if (is_switch(wall)) {
    var id = wall - WALL_SWITCH0;
    return "SWITCH " + id;
  } else if (is_door(wall)) {
    var id = wall - WALL_DOOR0;
    return "DOOR " + id;
  } else if (wall === WALL_WINDOW) {
    return "WINDOW";
  }
  return "" + wall;
}

function get_cell_name(cell) {
  if (cell === CELL_FLOOR) {
    return "FLOOR";
  } else if (is_key(cell)) {
    var id = cell - CELL_KEY0;
    return "KEY " + id;
  } else if (cell === CELL_EXIT) {
    return "EXIT";
  } else if (cell === CELL_CLOAK) {
    return "CLOAK";
  }
  return "" + cell;
}

function start(world) {
  var cursor = [0, 0];
  var cursor_dir = true;
  var cursor_type = 1;
  var menu = false;
  paused = false;
  var editor_selected_wall = WALL_BLOCK0;
  var last_selected_teleport = WALL_TELEPORT0;
  var last_selected_switch = WALL_SWITCH0;
  var last_selected_door = WALL_DOOR0;
  var last_selected_block = WALL_BLOCK0;
  var editor_selected_cell = CELL_FLOOR;
  var last_selected_key = CELL_KEY0;
  function to_json(st) {
    return JSON.stringify({version: 2, map: world.first_state.map, options: world.first_state.options});
  }

  function to_binary(st) {
    var arr = new Uint8Array(world.first_state.map);
    return arr;
  }

  function from_json(json, st) {
    var map = JSON.parse(json);
    if (map && map.map && map.options) {
      var options = map.options;
      if (map.version < 2) {
        var ADDED_6_BLOCKS = 6;
        var VERSION1_BLOCK_MAX = 4;
        for (var i = 0;i < MAP_SIZE;++i) {
          if (map.map[i] >= VERSION1_BLOCK_MAX) map.map[i] += ADDED_6_BLOCKS;
        }
        options = {};
        Object.keys(map.options).forEach(function(k) {
          var new_k = +k;
          if (k >= VERSION1_BLOCK_MAX) new_k += ADDED_6_BLOCKS;
          options[new_k] = map.options[k];
        });
      }
      st.map = new Uint8Array(map.map);
      st.options = options;
      console.log(options);
      console.log(map.map);
    }
  }

  function load_map() {
    var data = window.localStorage.getItem("hyp__map");
    if (data) from_json(data, world.first_state);
  }

  world_reset(world);
  function save_map() {
    var data = to_json(world.first_state);
    window.localStorage.setItem("hyp__map", data);
    console.log("saved", data);
  }

  var last_filename = "map.txt";
  function save_blob_to_disk(blob, filename) {
    var a = document.createElement("a");
    a.download = filename;
    a.rel = "noopener";
    a.href = URL.createObjectURL(blob);
    setTimeout(function() {
      URL.revokeObjectURL(a.href);
    }, 40000);
    setTimeout(function() {
      a.dispatchEvent(new MouseEvent("click"));
      console.log("clicked");
    }, 0);
  }

  function save_map_to_disk() {
    var data = to_json(world.first_state);
    save_blob_to_disk(new File([data], last_filename), last_filename);
  }

  function save_binary_map_to_disk() {
    var data = to_binary(world.first_state);
    console.log(data);
    save_blob_to_disk(new File([data], last_filename), last_filename);
  }

  function reset_world() {
    save_map();
    world_reset(world);
    paused = false;
  }

  function empty_world() {
    world.first_state = state_create();
    world_reset(world);
    paused = true;
  }

  function load_map_from_disk(text) {
    if (text.includes("songData")) {
      var song = new Function(text + "; return song")();
    } else {
      from_json(text, world.first_state);
      reset_world();
    }
  }

  var drop_div = document.getElementById("d");
  drop_div.ondrop = function(ev) {
    console.log("drop", ev);
    if (ev.dataTransfer.items) {
      for (var i = 0;i < ev.dataTransfer.items.length;++i) {
        if (ev.dataTransfer.items[i].kind === "file") {
          var file = ev.dataTransfer.items[i].getAsFile();
          last_filename = file.name;
          var reader = new FileReader();
          reader.onload = function(rev) {
            load_map_from_disk(rev.target.result);
          };
          reader.readAsText(file);
        }
      }
    }
    return false;
  };
  drop_div.ondragover = function(ev) {
    return false;
  };
  window.addEventListener("wheel", function(ev) {
    console.info(ev.deltaY);
  });
  window.addEventListener("keydown", function(ev) {
    if (ev.keyCode === 72) menu = !menu;
    if (menu) {
      if (ev.keyCode === 76) reset_world(), menu = false;
      if (ev.keyCode === 78) save_map_to_disk(), menu = false;
      if (ev.keyCode === 69) empty_world(), menu = false;
      if (ev.keyCode === 82) save_binary_map_to_disk(), menu = false;
    } else {
      if (ev.keyCode === 69) cursor[1] += 1;
 else if (ev.keyCode === 85) cursor[1] -= 1;
      if (ev.keyCode === 65) cursor[0] -= 1;
 else if (ev.keyCode === 79) cursor[0] += 1;
      if (ev.keyCode === 88) cursor_dir = !cursor_dir;
      if (ev.keyCode === 90) cursor_type = 1 - cursor_type;
      if (ev.keyCode === 32) {
        if (cursor_type === 0) {
          var cur_wall = get_wall(world.first_state.map, cursor, cursor_dir);
          var draw_wall = cur_wall === editor_selected_wall ? WALL_FLOOR : editor_selected_wall;
          set_wall(world.first_state.map, cursor, cursor_dir, draw_wall);
        } else {
          var cur_cell = get_cell(world.first_state.map, cursor);
          var draw_cell = cur_cell === editor_selected_cell ? CELL_FLOOR : editor_selected_cell;
          set_cell(world.first_state.map, cursor, draw_cell);
        }
      }
      if (ev.keyCode >= 48 && ev.keyCode <= 58) {
        var num = ev.keyCode - 48;
        if (cursor_type === 0) {
          if (num === 1) editor_selected_wall = WALL_FLOOR;
 else if (num === 2) is_block(editor_selected_wall) ? is_block(editor_selected_wall + 1) ? (editor_selected_wall += 1) : (editor_selected_wall = WALL_BLOCK0) : (editor_selected_wall = last_selected_block), last_selected_block = editor_selected_wall;
 else if (num === 3) is_teleport(editor_selected_wall) ? is_teleport(editor_selected_wall + 1) ? (editor_selected_wall += 1) : (editor_selected_wall = WALL_TELEPORT0) : (editor_selected_wall = last_selected_teleport), last_selected_teleport = editor_selected_wall;
 else if (num === 4) is_switch(editor_selected_wall) ? is_switch(editor_selected_wall + 1) ? (editor_selected_wall += 1) : (editor_selected_wall = WALL_SWITCH0) : (editor_selected_wall = last_selected_switch), last_selected_switch = editor_selected_wall;
 else if (num === 5) is_door(editor_selected_wall) ? is_door(editor_selected_wall + 1) ? (editor_selected_wall += 1) : (editor_selected_wall = WALL_DOOR0) : (editor_selected_wall = last_selected_door), last_selected_door = editor_selected_wall;
 else if (num === 6) editor_selected_wall = WALL_WINDOW;
        } else {
          if (num === 1) editor_selected_cell = CELL_FLOOR;
 else if (num === 2) is_block(editor_selected_cell) ? is_block(editor_selected_cell + 1) ? (editor_selected_cell += 1) : (editor_selected_cell = CELL_KEY0) : (editor_selected_cell = last_selected_key), last_selected_key = editor_selected_cell;
 else if (num === 3) editor_selected_cell = CELL_EXIT;
 else if (num === 4) editor_selected_cell = CELL_CLOAK;
        }
      }
      if (ev.keyCode === 9) paused = !paused, ev.preventDefault();
    }
    console.log("key", ev.keyCode);
    return true;
  });
  draw = function(whiteTex$0, fontBits$0, imgShader) {
    gl.disable(GL_DEPTH_TEST);
    color(4294967295);
    setView2([1, 0], [1, 0]);
    activateShader(imgShader);
    var scale_down = 100;
    var drawWall = function(x, y$0, dir, width) {
      var down = !dir ? 1 : -width;
      var up = !dir ? 0 : width;
      var right = dir ? 1 : width;
      var left = dir ? 0 : -width;
      wall3d(whiteTex$0, scale_down, 10 + x + left, scale_down, 10 + x + right, -y$0 - up, -y$0 - down, 0, 0, 1, 1);
    };
    var drawCell = function(x, y$0) {
      wall3d(whiteTex$0, scale_down, 10 + x + 0, scale_down, 10 + x + 1, -y$0 - 0, -y$0 - 1, 0, 0, 1, 1);
    };
    var drawWallText = function(x, y$0, dir, text, mirror) {
      var down = !dir ? 1 : 0;
      var up = !dir ? 0 : 0;
      var right = dir ? 1 : 0;
      var left = dir ? 0 : 0;
      var textScale = 0.4 / 3;
      var stridex = mirror ? -1 : 1;
      var midx = 10 + x + (left + right) / 2;
      var midy = -y$0 - (up + down) / 2 - textScale * 5 / 2;
      if (mirror) midx += textScale * 3 / 2;
 else {
        midx -= textScale * 3 / 2;
      }
      drawText(fontBits$0, whiteTex$0, text, midx, midy, 100, stridex, 0, 1, 0.4 / 3);
    };
    var drawCellText = function(x, y$0, text, mirror) {
      var textScale = 0.4 / 3;
      var stridex = mirror ? -1 : 1;
      var midx = 10 + x + 1 / 2;
      var midy = -y$0 - 1 / 2 - textScale * 5 / 2;
      if (mirror) midx += textScale * 3 / 2;
 else {
        midx -= textScale * 3 / 2;
      }
      drawText(fontBits$0, whiteTex$0, text, midx, midy, 100, stridex, 0, 1, 0.4 / 3);
    };
    color(2130706432);
    wall3d(whiteTex$0, scale_down, 10 + 0, scale_down, 10 + 0 + W, -0, -0 - H, 0, 0, 1, 1);
    if (cursor_type === 0) color(4278255360), drawWall(cursor[0], cursor[1], cursor_dir, 0.1);
 else {
      color(2852192000);
      drawCell(cursor[0], cursor[1]);
    }
    var now = current_player_time(world);
    for (var my = 0;my < 32;++my) {
      for (var mx = 0;mx < 32;++mx) {
        var step = 1;
        var st = world.first_state;
        var down = get_wall(st.map, [mx, my], false);
        var right = get_wall(st.map, [mx, my], true);
        var cell = get_cell(st.map, [mx, my]);
        var width = 0.05;
        var obs = get_observed_cell(world, now, mx, my);
        if (obs) obs === 1 ? color(872415231) : obs === 255 ? color(1442775295) : color(855638271), wall3d(whiteTex$0, scale_down, 10 + mx + 0, scale_down, 10 + mx + 1, -my - 0, -my - 1, 0, 0, 1, 1);
        if (is_key(cell)) {
          var id = cell - CELL_KEY0;
          color(1073741823);
          drawCell(mx, my);
          color(4278255615);
          drawCellText(mx, my, "" + id, false);
        } else if (cell === CELL_EXIT) color(1073741823), drawCell(mx, my), color(4294967040), drawCellText(mx, my, "E", false);
 else if (cell === CELL_CLOAK) color(1073741823), drawCell(mx, my), color(4289374890), drawCellText(mx, my, "C", false);
        [[down, false], [right, true]].forEach(function([wall, dir]) {
          if (is_block(wall)) wall > WALL_BLOCK0 ? (color(4289374890), drawWall(mx, my, dir, width), color(4294967295), drawWallText(mx, my, dir, "" + (wall - WALL_BLOCK0), false)) : (color(4294967295), drawWall(mx, my, dir, width));
 else if (is_teleport(wall)) {
            var teleport_id = wall - WALL_TELEPORT0 >> 1;
            color(4294945450);
            drawWallText(mx, my, dir, "" + teleport_id, wall & 1);
          } else if (is_switch(wall)) {
            var switch_id = wall - WALL_SWITCH0;
            color(4294967210);
            drawWallText(mx, my, dir, "" + switch_id, false);
          } else if (is_door(wall)) {
            var door_id = wall - WALL_DOOR0;
            color(4294945535);
            drawWallText(mx, my, dir, "" + door_id, false);
          }
        });
      }
    }
    color(4278190335);
    var player_pos = world.current_player_state.pos;
    var player_width = 0.1;
    wall3d(whiteTex$0, scale_down, 10 + player_pos[0] - player_width, scale_down, 10 + player_pos[0] + player_width, -player_pos[1] - player_width, -player_pos[1] + player_width, 0, 0, 1, 1);
    Object.keys(world.frames[now].player_states).forEach(function(k) {
      if (+k !== world.current_player) {
        var st = world.frames[now].player_states[k];
        color(4289374975);
        var other_player_pos = st.pos;
        wall3d(whiteTex$0, scale_down, 10 + other_player_pos[0] - player_width, scale_down, 10 + other_player_pos[0] + player_width, -other_player_pos[1] - player_width, -other_player_pos[1] + player_width, 0, 0, 1, 1);
      }
    });
    color(4294967295);
    if (cursor_type === 0) drawText(fontBits$0, whiteTex$0, get_wall_name(editor_selected_wall), 10, 0, 100, 1, 0, 1, 1 / 3);
 else {
      drawText(fontBits$0, whiteTex$0, get_cell_name(editor_selected_cell), 10, 0, 100, 1, 0, 1, 1 / 3);
    }
    drawText(fontBits$0, whiteTex$0, cursor[0] + " " + cursor[1], 30, 0, 100, 1, 0, 1, 1 / 3);
    if (menu) drawText(fontBits$0, whiteTex$0, "N  SAVE", 10, 5, 100, 1, 0, 1, 1 / 3), drawText(fontBits$0, whiteTex$0, "L  RESET", 10, 7, 100, 1, 0, 1, 1 / 3), drawText(fontBits$0, whiteTex$0, "E  EMPTY MAP", 10, 9, 100, 1, 0, 1, 1 / 3);
    flush();
    gl.enable(GL_DEPTH_TEST);
  };
}

function osc_sin(value) {
  return Math.sin(value * 6.283184);
}

function osc_saw(value) {
  return 2 * (value % 1) - 1;
}

function osc_square(value) {
  return value % 1 < 0.5 ? 1 : -1;
}

function osc_tri(value) {
  var v2 = value % 1 * 4;
  if (v2 < 2) {
    return v2 - 1;
  }
  return 3 - v2;
}

function getnotefreq(n) {
  return 0.003959503758 * Math.pow(2, (n - 128) / 12);
}

function createNote(instr, n, rowLen) {
  var osc1 = mOscillators[instr.i[0]];
  var o1vol = instr.i[1];
  var o1xenv = instr.i[3];
  var osc2 = mOscillators[instr.i[4]];
  var o2vol = instr.i[5];
  var o2xenv = instr.i[8];
  var noiseVol = instr.i[9];
  var attack = instr.i[10] * instr.i[10] * 4;
  var sustain = instr.i[11] * instr.i[11] * 4;
  var release = instr.i[12] * instr.i[12] * 4;
  var releaseInv = 1 / release;
  var arp = instr.i[13];
  var arpInterval = rowLen * Math.pow(2, 2 - instr.i[14]);
  var noteBuf = new Int32Array(attack + sustain + release);
  var c1 = 0;
  var c2 = 0;
  var o1t;
  var o2t;
  var j = 0;
  var j2 = 0;
  for (;j < attack + sustain + release;) {
    if (j2 >= 0) arp = arp >> 8 | (arp & 255) << 4, j2 -= arpInterval, o1t = getnotefreq(n + (arp & 15) + instr.i[2] - 128), o2t = getnotefreq(n + (arp & 15) + instr.i[6] - 128) * (1 + 0.0008 * instr.i[7]);
    var e = 1;
    if (j < attack) e = j / attack;
 else if (j >= attack + sustain) e -= (j - attack - sustain) * releaseInv;
    var t = o1t;
    if (o1xenv) t *= e * e;
    c1 += t;
    var rsample = osc1(c1) * o1vol;
    t = o2t;
    if (o2xenv) t *= e * e;
    c2 += t;
    rsample += osc2(c2) * o2vol;
    if (noiseVol) rsample += (2 * Math.random() - 1) * noiseVol;
    noteBuf[j] = 80 * rsample * e | 0;
    j += 1;
    j2 += 1;
  }
  return noteBuf;
}

var mOscillators = [osc_sin, osc_square, osc_saw, osc_tri];
var mSong;
var mLastRow;
var mCurrentCol;
var mNumWords;
var mMixBuf;
function init$0(song) {
  mSong = song;
  mLastRow = song.endPattern;
  mCurrentCol = 0;
  mNumWords = song.rowLen * song.patternLen * (mLastRow + 1) * 2;
  mMixBuf = new Int32Array(mNumWords);
}

function generate() {
  var i;
  var j;
  var b;
  var n;
  var cp;
  var k;
  var t;
  var lfor;
  var e;
  var x;
  var rsample;
  var rowStartSample;
  var f$0;
  var da;
  var chnBuf = new Int32Array(mNumWords);
  var instr = mSong.songData[mCurrentCol];
  var rowLen = mSong.rowLen;
  var patternLen = mSong.patternLen;
  var low = 0;
  var band = 0;
  var high;
  var lsample;
  var filterActive = false;
  var noteCache = [];
  for (var p = 0;p < mLastRow + 1;++p) {
    cp = instr.p[p];
    for (var row = 0;row < patternLen;++row) {
      var cmdNo = cp ? instr.c[cp - 1].f[row] : 0;
      if (cmdNo) {
        instr.i[cmdNo - 1] = instr.c[cp - 1].f[row + patternLen] || 0;
        if (cmdNo < 16) noteCache = [];
      }
      var oscLFO = mOscillators[instr.i[15]];
      var lfoAmt = instr.i[16] / 512;
      var lfoFreq = Math.pow(2, instr.i[17] - 9) / rowLen;
      var fxLFO = instr.i[18];
      var fxFilter = instr.i[19];
      var fxFreq = instr.i[20] * 43.23529 * 3.141592 / 44100;
      var q = 1 - instr.i[21] / 255;
      var dist = instr.i[22] * 0.00001;
      var drive = instr.i[23] / 32;
      var panAmt = instr.i[24] / 512;
      var panFreq = 6.283184 * Math.pow(2, instr.i[25] - 9) / rowLen;
      var dlyAmt = instr.i[26] / 255;
      var dly = instr.i[27] * rowLen & ~1;
      rowStartSample = (p * patternLen + row) * rowLen;
      for (var col$0 = 0;col$0 < 4;++col$0) {
        n = cp ? instr.c[cp - 1].n[row + col$0 * patternLen] : 0;
        if (n) {
          if (!noteCache[n]) noteCache[n] = createNote(instr, n, rowLen);
          var noteBuf = noteCache[n];
          j = 0;
          i = rowStartSample * 2;
          for (;j < noteBuf.length;) chnBuf[i] += noteBuf[j], j += 1, i += 2;
        }
      }
      j = 0;
      for (;j < rowLen;) {
        k = (rowStartSample + j) * 2;
        rsample = chnBuf[k];
        if (rsample || filterActive) {
          f$0 = fxFreq;
          if (fxLFO) f$0 *= oscLFO(lfoFreq * k) * lfoAmt + 0.5;
          f$0 = 1.5 * Math.sin(f$0);
          low += f$0 * band;
          high = q * (rsample - band) - low;
          band += f$0 * high;
          rsample = fxFilter === 3 ? band : fxFilter === 1 ? high : low;
          if (dist) rsample *= dist, rsample = rsample < 1 ? rsample > -1 ? osc_sin(rsample * 0.25) : -1 : 1, rsample /= dist;
          rsample *= drive;
          filterActive = rsample * rsample > 0.00001;
          t = Math.sin(panFreq * k) * panAmt + 0.5;
          lsample = rsample * (1 - t);
          rsample *= t;
        } else {
          lsample = 0;
        }
        if (k >= dly) lsample += chnBuf[k - dly + 1] * dlyAmt, rsample += chnBuf[k - dly] * dlyAmt;
        chnBuf[k] = lsample | 0;
        chnBuf[k + 1] = rsample | 0;
        mMixBuf[k] += lsample | 0;
        mMixBuf[k + 1] += rsample | 0;
        j += 1;
      }
    }
  }
  mCurrentCol += 1;
  return mCurrentCol / mSong.numChannels;
}

function getData(t, n) {
  var i = 2 * Math.floor(t * 44100);
  var d = new Array(n);
  for (var j = 0;j < 2 * n;++j) {
    var k = i + j;
    d[j] = t > 0 && k < mMixBuf.length ? mMixBuf[k] / 32768 : 0;
  }
  return d;
}

function getDataTyped(i, n, d1, d2) {
  for (var j = 0;j < n;++j) {
    var k = (i + j) * 2;
    d1[j] = i > 0 && k < mMixBuf.length ? mMixBuf[k] / 65536 : 0;
    d2[j] = i > 0 && k + 1 < mMixBuf.length ? mMixBuf[k + 1] / 65536 : 0;
  }
}







var fs = main[2];
function genTex(pixels, ty, f$0) {
  var side = Math.sqrt(pixels.length);
  var w = side;
  var h = side;
  var i = 0;
  var y$0 = 0;
  for (;y$0 < h;) {
    var x = 0;
    for (;x < w;) pixels[i] = f$0(x, y$0), x += 1, i += 1;
    y$0 += 1;
  }
  var pixtex = createTexture(pixels, side, ty);
  console.log(pixels);
  pixtex.d = pixels;
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

var fontBits = atob("1tb7v+b97tm0mua3bd+RTdi2aZKKzd+2UduWlfRd/5f+v7j8XS+1l1SFZFO2adKqTd98VPUq08c9X+c97tW3ijtZrQ4=").split("").map(function(x) {
  return x.charCodeAt(0);
});
var whiteTex;
function startGame() {
  var gl$0;
  var imgShader;
  var translucentShader;
  var skyShader;
  var skyMirrorShader;
  var gridShader;
  var fb;
  var canvas = document.getElementById("g");
  init(canvas);
  gl$0 = gl;
  skyShader = createShaderProgram(basicVs, main[0]);
  skyMirrorShader = createShaderProgram(basicVs, main[1]);
  gridShader = createShaderProgram(basicVs, main[4]);
  imgShader = createShaderProgram(basicVs, fs);
  translucentShader = createShaderProgram(basicVs, main[3]);
  function getPointColor(x, y$0) {
    var alpha = (7.5 - Math.hypot(x - 8, y$0 - 8)) * 64 | 0;
    return 1052927 + (clamp(alpha, 0, 255) << 24);
  }

  var pointTex = genTex(new Uint32Array(16 * 16), GL_RGBA, getPointColor);
  whiteTex = genTex(new Uint32Array(1), GL_RGBA, function() {
    return 4294967295;
  });
  var world = world_create();
  var keys = {};
  var audio;
  var aim = [1, 0];
  var aimv = [1, 0];
  window.onkeydown = function(ev) {
    keys[ev.keyCode] = 1;
  };
  window.onkeyup = function(ev) {
    keys[ev.keyCode] = 0;
  };
  function play2(buf) {
    if (audio) {
      var src = audio.createBufferSource();
      var pan = audio.createStereoPanner();
      var gain = audio.createGain();
      gain.gain.value = 0.5;
      pan.pan.value = 0;
      src.buffer = buf;
      src.connect(gain).connect(pan).connect(audio.destination);
      src.start();
    }
  }

  function osc_sin$0(value) {
    return Math.sin(value * 6.283184);
  }

  function osc_tri$0(value) {
    var v2 = value % 1 * 4;
    if (v2 < 2) {
      return v2 - 1;
    }
    return 3 - v2;
  }

  function osc_square$0(value) {
    return value % 1 < 0.5 ? 1 : -1;
  }

  function getnotefreq$0(n) {
    return 0.003959503758 * Math.pow(2, (n - 128) / 12);
  }

  canvas.onclick = function() {
    if (!audio) {
      audio = new AudioContext();
      var bufSize = 16384;
      var scriptProc = audio.createScriptProcessor(bufSize, 0, 2);
      var time = 0;
      var tick = 0;
      var start$0 = 44100 * 4;
      var beep = Array(44100 * 4).fill(0);
      var rowLen = 5513;
      var delay = (rowLen * 6 & ~1) / 2;
      var delayAmount = 17 / 255;
      var fxFreq = 30 * 43.23529 * 3.141592 / 44100;
      var lfoFreq_ = 0;
      var lfoFreq = Math.pow(2, lfoFreq_ - 9) / rowLen;
      var lfoAmount = 0;
      var dist = 119 * 0.00001;
      var oscLFO = osc_tri$0;
      var q = 1 - 184 / 255;
      var low = 0;
      var band = 0;
      var high;
      var drive = 244 / 32;
      var o1vol = 100;
      var o2vol = 201;
      var o1semi = 128 - 128;
      var o2semi = 128 - 128;
      var attack = 0 * 0 * 4;
      var sustain = 6 * 6 * 4;
      var release = 49 * 49 * 4;
      var osc2detune = 0;
      var note = 126;
      for (var x = 0;x < beep.length;++x) {
        var o1 = getnotefreq$0(note + o1semi);
        var o2 = getnotefreq$0(note + o2semi) * (1 + 0.0008 * osc2detune);
        var e = 1;
        if (x < attack) e = x / attack;
 else if (x >= attack + sustain && x <= attack + sustain + release) e -= (x - attack - sustain) / release;
 else {
          e = 0;
        }
        var s = e * 80 * (osc_tri$0(o1 * x) * o1vol + osc_tri$0(o2 * x) * o2vol) | 0;
        var k = x * 2;
        var fxF = fxFreq;
        fxF *= oscLFO(lfoFreq * k) * lfoAmount + 0.5;
        fxF = 1.5 * Math.sin(fxF);
        low += fxF * band;
        high = q * (s - band) - low;
        band += fxF * high;
        s = band;
        s *= dist;
        if (s < 1) s > -1 ? (s = osc_sin$0(s * 0.25)) : (s = -1);
 else {
          s = 1;
        }
        s /= dist;
        s *= drive;
        if (x >= delay) s += beep[x - delay] * delayAmount;
        beep[x] = s / 65536;
      }
      var samples = [];
      function play1() {
        samples.push([tick, beep]);
      }

      console.log(beep);
      init$0(window.song);
      generate();
      var t = 0;
      scriptProc.onaudioprocess = function(e) {
        var left = e.outputBuffer.getChannelData(0);
        var right = e.outputBuffer.getChannelData(1);
        getDataTyped(tick, bufSize, left, right);
        tick += bufSize;
      };
      scriptProc.connect(audio.destination);
      var buf = audio.createBuffer(1, beep.length, 44100);
      buf.getChannelData(0).set(beep);
      play2(buf);
    }
    canvas.requestPointerLock();
  };
  canvas.onmousemove = function(e) {
    if (document.pointerLockElement === canvas) {
      var xdiff = e.movementX / 500;
      var ydiff = e.movementY / 500;
      aim = vrotate(aim, [Math.cos(xdiff), Math.sin(xdiff)]);
      aimv = vrotate(aimv, [Math.cos(ydiff), Math.sin(ydiff)]);
    }
  };
  start(world);
  function render_map(renders, sign) {
    activateShader(skyShader);
    var height = 100;
    var step = 100;
    var bottom = sign * -height / 2 - (sign < 0 ? height : 0);
    var top = sign * height / 2 - (sign < 0 ? height : 0);
    renders.sky.forEach(function(r) {
      var x = r.fromx * step;
      var z = r.fromy * step;
      var x2 = r.tox * step;
      var z2 = r.toy * step;
      color(r.color);
      wall3d(whiteTex, x, z, x2, z2, bottom, top, 0, sign * top, 1, sign * bottom);
    });
    flush();
    gl$0.depthMask(0);
    activateShader(translucentShader);
    color(4278255360);
    prism(whiteTex, 5.5 * step, 0, bottom, 0.2 * step, 0.2 * step, (top - bottom) / 2);
    renders.lines.forEach(function(r) {
      var x = r.fromx * step;
      var z = r.fromy * step;
      var x2 = r.tox * step;
      var z2 = r.toy * step;
      color(r.color);
      wall3d(whiteTex, x, z, x2, z2, bottom, top, 0, 0, 1, 1);
    });
    flush();
    gl$0.depthMask(1);
    activateShader(imgShader);
    renders.portal_texts.forEach(function(r) {
      var x = r.fromx * step;
      var z = r.fromy * step;
      var x2 = r.tox * step;
      var z2 = r.toy * step;
      color(4278255360);
      var textdir = vnormalize2([x2 - x, z2 - z]);
      drawText(fontBits, whiteTex, r.text || "THING", z, top, x, textdir[1], textdir[0], sign, 10 / 3);
    });
    flush();
  }

  var nextFrame = 0;
  function update() {
    window.requestAnimationFrame(function(currentTime) {
      update();
      var walk = [0, 0];
      if (keys[71]) walk = vadd2(walk, aim);
 else if (keys[83]) walk = vsub2(walk, aim);
      if (keys[68]) walk = vadd2(walk, [aim[1], -aim[0]]);
 else if (keys[84]) walk = vsub2(walk, [aim[1], -aim[0]]);
      if (currentTime < nextFrame) {
        return void 0;
      }
      nextFrame = currentTime + 16.6;
      var renders;
      var speed = keys[16] ? 5 : 1;
      for (var i = 0;i < speed;++i) renders = {lines: [], sky: [], portal_texts: []}, world_update(world, {aim: aim, walk: walk, act: keys[81]}, renders, paused);
      renders.lines.sort(function(a, b) {
        var len_a = a.fromx * a.fromx + a.fromy * a.fromy;
        var len_b = b.fromx * b.fromx + b.fromy * b.fromy;
        if (len_a < len_b) {
          return -1;
        } else if (len_a === len_b) {
          return 0;
        } else {
          return 1;
        }
      });
      if (!fb) canvas_w = canvas.width = window.innerWidth, canvas_h = canvas.height = window.innerHeight, fb = createFramebufferTexture(canvas_w, canvas_h);
 else if (window.innerWidth !== canvas_w || window.innerHeight !== canvas_h) canvas_w = canvas.width = window.innerWidth, canvas_h = canvas.height = window.innerHeight, deleteFramebufferTexture(fb), fb = createFramebufferTexture(canvas_w, canvas_h);
      gl$0.viewport(0, 0, gl$0.drawingBufferWidth, gl$0.drawingBufferHeight);
      bindFramebufferTexture(fb);
      gl$0.clear(GL_COLOR_BUFFER_BIT);
      color(4294967295);
      setView2(aim, aimv);
      render_map(renders, -1);
      unbindFramebufferTexture();
      gl$0.clear(GL_COLOR_BUFFER_BIT);
      gl$0.depthMask(0);
      setView2([1, 0], [1, 0]);
      activateShader(imgShader);
      color(1442840575);
      wall3d(fb.texture, 1.83, -1, 1.83, 1, canvas_h / canvas_w, -canvas_h / canvas_w, 0, 0, 1, 1);
      flush();
      gl$0.depthMask(1);
      setView2(aim, aimv);
      render_map(renders, 1);
      draw(whiteTex, fontBits, imgShader);
      setView2([1, 0], [1, 0]);
      activateShader(imgShader);
      var shade = world.offset;
      if (world.reset_on > 0) shade = Math.min(shade, world.reset_on - world.offset);
      shade /= 120;
      if (shade < 1) color((1 - shade) * 255 << 24), wall3d(whiteTex, 1.83, -1, 1.83, 1, canvas_h / canvas_w, -canvas_h / canvas_w, 0, 0, 1, 1);
      color(4294967295);
      var now = current_player_time(world);
      var second = TIME_IN_SECONDS - now / SECOND | 0;
      var minute = second / 60 | 0;
      second %= 60;
      drawText(fontBits, whiteTex, minute + " " + second, -200, 250, 1000, 1, 0, 1, 10 / 3);
      drawText(fontBits, whiteTex, world.conflicts + " CONFLICTS", -200, 300, 1000, 1, 0, 1, 10 / 3);
      flush();
    });
  }

  update();
}

window.onload = function() {
  startGame();
};




function foo([x, y$0]) {
  return {x: x, y: x};
}

var f = 0;
var y = f.y;
})(this)