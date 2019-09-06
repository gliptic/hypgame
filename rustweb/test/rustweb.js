(function(window){
var vertex = `varying vec3 m;
varying vec2 n;
varying vec4 o;
attribute vec3 a;
attribute vec2 c;
attribute vec4 d;
uniform float e;
uniform mat2 f;
void main() {
  m = a;
  n = c;
  o = d;
  vec2 b = f * a.xz;
  gl_Position = mat4(1.83, 0, 0, 0, 0, 1.83 * e, 0, 0, 0, 0, -1, -1, 0, 0, -2, 0) * vec4(b.x, a.y, b.y, 1);
}

`;
var main = [`vec3 a = normalize(m.xyz);
  float b = a.y * -.5 + .5;
  gl_FragColor = vec4(vec3(l(a)) + mix(mix(vec3(0), vec3(.2, .1, .43), b + .3), vec3(.08, .61, .83), b * 1.5 - .4), 1);
}

`,`gl_FragColor = texture2D(k, n) * o;
}

`,`gl_FragColor = vec4(o.rgb, .5);
}

`,`float a = 1., b = 2.;
  if (a == 1.) a = 3.; else {
    a = 4.;
    b = 5.;
    return z;
  }
  gl_FragColor = vec4(1, 1, 1, 1 == 1 ? 1 : 1);
}

`].map(function (a) { return `varying vec3 m;
varying vec2 n;
varying vec4 o;
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
    float b = g(a.xy * .5), i = 0.;
    f += b * (.3 * sin(i * 5. * (b * 5.) + b) + .7) * 1.5;
  }
  return f * abs(c.z);
}

mat3 j(float a, float b) {
  return mat3(cos(a), 0, -sin(a), 0, 1, 0, sin(a), 0, cos(a)) * mat3(1, 0, 0, 0, cos(b), sin(b), 0, -sin(b), cos(b));
}

float h(vec3 a) {
  return d(a.xyz) + d(a.yzx) + d(a.zxy);
}

float l(vec3 a) {
  float c = 3.1415927, b = c / 180.;
  return h(a) + h(j(45. * b, 45. * b) * a);
}

void main() {
  ` + a; });
function trans(mat, x, y) {
  mat[6] += mat[0] * x + mat[3] * y;
  mat[7] += mat[1] * x + mat[4] * y;
}

function scale(mat, x, y) {
  mat[0] *= x;
  mat[1] *= x;
  mat[3] *= y;
  mat[4] *= y;
}

function rotvec(mat, x, y) {
  var a = mat[0];
  var b = mat[1];
  var c = mat[3];
  var d = mat[4];
  mat[0] = a * x + c * -y;
  mat[1] = b * x + d * -y;
  mat[3] = a * y + c * x;
  mat[4] = b * y + d * x;
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

function vnormalize2([x, y]) {
  var l = Math.hypot(x, y);
  return [x / l, y / l];
}



var gl;
var canvas_w;
var canvas_h;
var viewTrans;
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
var basicVs = vertex;
var VERTEX_SIZE = 4 * 3 + 4 * 2 + 4 * 4;
var MAX_BATCH = 10922;
var VERTICES_PER_QUAD = 6;
var QUAD_SIZE_IN_WORDS = VERTEX_SIZE * VERTICES_PER_QUAD / 4;
var VERTEX_DATA_SIZE = VERTEX_SIZE * MAX_BATCH * 6;
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
  gl.clearColor(0.3, 0.35, 0.5, 1);
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

function setViewTransform(shader) {
  gl.uniform1f(gl.getUniformLocation(shader, "e"), canvas_w / canvas_h);
  gl.uniformMatrix2fv(gl.getUniformLocation(shader, "f"), 0, viewTrans);
}

function color(c) {
  col = c;
}

function setView2([aimx, aimy]) {
  viewTrans = [-aimy, -aimx, aimx, -aimy];
}

function setView(x, y, rotx, roty, zoom) {
  var ratio = canvas_h / canvas_w;
  viewTrans = [1, 0, 0, 0, 1, 0, 0, 0, 1];
  scale(viewTrans, 2 / (1024 * zoom), 2 / (1024 * zoom * ratio));
  rotvec(viewTrans, rotx, roty);
  trans(viewTrans, -x, -y);
}

function flush() {
  if (vertexData.length) gl.vertexAttribPointer(locPos, 3, GL_FLOAT, 0, VERTEX_SIZE, 0), gl.vertexAttribPointer(locUV, 2, GL_FLOAT, 0, VERTEX_SIZE, 12), gl.vertexAttribPointer(locColor, 4, GL_FLOAT, 0, VERTEX_SIZE, 20), gl.bufferSubData(GL_ARRAY_BUFFER, 0, new Float32Array(vertexData)), gl.drawArrays(GL_TRIANGLES, 0, vertexData.length / (VERTEX_SIZE / 4)), vertexData.length = 0;
  currentTexture = null;
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
  gl.getExtension("OES_standard_derivatives");
  VBO = createBuffer(GL_ARRAY_BUFFER, VERTEX_DATA_SIZE, GL_DYNAMIC_DRAW);
  gl.enableVertexAttribArray(locPos);
  gl.enableVertexAttribArray(locUV);
  gl.enableVertexAttribArray(locColor);
  gl.activeTexture(GL_TEXTURE0);
}

function drawText(fontBits$0, whiteTex$0, text, x, y) {
  for (var i = 0;i < text.length;++i) {
    var ch = text.charCodeAt(i);
    var id = 200;
    if (ch >= 48 && ch <= 57) id = ch - 48;
 else if (ch >= 65 && ch <= 90) id = ch - 65 + 10;
    var scale$0 = 1;
    var step = 10 * scale$0 / 3;
    for (var dx = 0;dx < 3;++dx) {
      for (var dy = 0;dy < 5;++dy) {
        var pos = (4 - dy) * 36 * 3 + id * 3 + dx;
        if (fontBits$0[pos >> 3] >> (pos & 7) & 1) {
          var z = 1000;
          wall3d(whiteTex$0, z, x + step * dx, z, x + step * dx + step, y + dy * step, y + dy * step - step, 0, 0, 1, 1);
        }
      }
    }
    x += 12 * scale$0;
  }
}


var W = 32;
var H = 32;
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
var VISUAL_SIZE = VISUAL_W * VISUAL_H * 2;
console.log("INTERVAL", INTERVAL);
console.log("STATES", STATES);
console.log("FRAMES", FRAMES);
var CELL_FLOOR = 1;
var CELL_BLOCK = 2;
var CELL_TELEPORT0 = 4;
var CELL_SWITCH0 = 10;
var CELL_DOOR0 = 20;
var ACTION_CHANGE = 0;
var ACTION_MOVE = 1;
function map_create() {
  return Array(MAP_SIZE).fill(0);
}

function set_cell(map, [x, y], what) {
  var prev = map[y * W + x];
  map[y * W + x] = what;
  return prev;
}

function set_wall(map, [x, y], wall_dir, what) {
  var prev = map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
  map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
  return prev;
}

function set_wall_(map, x, y, wall_dir, what) {
  var prev = map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
  map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
  return prev;
}

function get_cell(map, [x, y]) {
  return map[y * W + x];
}

function get_wall(map, [x, y], wall_dir) {
  return map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
}

function get_wall_(map, x, y, wall_dir) {
  return map[y * W + x + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir];
}

function set_loc(map, loc, what) {
  map[loc] = what;
}

function set_wall_loc(map, loc, wall_dir, what) {
  map[loc + MAP_SECTION_SIZE + MAP_SECTION_SIZE * wall_dir] = what;
}

function state_create() {
  var self = {map: map_create(), prev_switches: 0, switches: 0, prev_outputs: 0, outputs: 0};
  var y = 0;
  for (;y < H;) {
    var x = 0;
    for (;x < W;) {
      set_wall(self.map, [x, y], false, CELL_FLOOR);
      set_wall(self.map, [x, y], true, CELL_FLOOR);
      if (x === 0 || x === W - 1 || y === 0 || y === H - 1) {
        if (y === 0 || y === H - 1) set_wall(self.map, [x, y], true, CELL_BLOCK);
        if (x === 0 || x === W - 1) set_wall(self.map, [x, y], false, CELL_BLOCK);
      }
      set_cell(self.map, [x, y], CELL_FLOOR);
      x += 1;
    }
    y += 1;
  }
  set_wall(self.map, [31, 9], false, CELL_TELEPORT0);
  set_wall(self.map, [31, 10], false, CELL_TELEPORT0);
  var i = 0;
  for (;i <= 10;) i += 1;
  console.log(self.map);
  return self;
}

function state_update(state, frame) {
  var change = ~state.prev_switches & state.switches;
  state.prev_switches = state.switches;
  state.switches = 0;
  state.prev_outputs = state.outputs;
  state.outputs = state.outputs ^ change;
  if (state.prev_outputs !== state.outputs) console.log("outputs changed:", state.outputs);
  if (change) console.log("switches:", change);
  frame.actions.forEach(function([type, a, b, c]) {
    if (type === ACTION_CHANGE) set_loc(state.map, a, b);
 else if (type === ACTION_MOVE) {
    }
  });
}

function time_in_state(world, s) {
  return (world.offset + s * INTERVAL) % FRAMES;
}

function copy_to(from, to) {
  if (Array.isArray(from)) {
    to = to || [];
    to.length = from.length;
    var i = 0;
    for (;i < from.length;) to[i] = copy_to(from[i], to[i]), i += 1;
  } else typeof(from) === "object" ? (to = to || {}, Object.keys(from).forEach(function(k) {
    to[k] = copy_to(from[k], to[k]);
  })) : (to = from);
  return to;
}

function clone(obj) {
  return copy_to(obj, {});
}

function world_reset(w) {
  w.frames = Array(FRAMES).fill(0).map(function() {
    return {observed_map: map_create(), actions: [], player_states: {}};
  });
  w.states = [];
  w.offset = 0;
  w.current_state = 0;
  w.current_player = 0;
  w.current_player_state = {aim: [1, 0], pos: [15.5, 15.5], inventory: []};
  w.visual = new Uint8Array(VISUAL_SIZE);
  w.states.push(clone(w.first_state));
  var i = 1;
  var time = 0;
  for (;i < STATES;) {
    var state = clone(w.states[w.states.length - 1]);
    var j = 0;
    for (;j < INTERVAL;) state_update(state, w.frames[time]), time += 1, j += 1;
    w.states.push(state);
    i += 1;
  }
  console.log(w.states);
}

function world_create() {
  var w = {first_state: state_create(), options: function() {
  }};
  world_reset(w);
  w.options[CELL_TELEPORT0] = {pos_diff: [-31, 0], state_diff: -1};
  return w;
}

function is_wall(cell) {
  return cell === CELL_BLOCK;
}

function is_teleport(cell) {
  return cell === CELL_TELEPORT0;
}

function seen_wall(world, visual_wall_id) {
  var index = visual_wall_id + 2 * (VISUAL_W / 2) * (VISUAL_H / 2);
  var prev = world.visual[index];
  world.visual[index] = 1;
  return prev;
}

function sweep5(world, s, origin, dir, max_len, renders, line_renders, sky_line_renders) {
  var iposx = origin[0] | 0;
  var iposy = origin[1] | 0;
  var piposx = iposx;
  var piposy = iposy;
  var blocked = true;
  var safe_counter = 0;
  var length = 0;
  var composite_transform = [1, 0, 0, 1];
  var visual_wall_id = 0;
  var div_start_x;
  var div_start_y;
  var north_south;
  for (;;) {
    safe_counter += 1;
    if (safe_counter > 100) {
      break;
    }
    var dir_positivex = dir[0] >= 0 ? 1 : 0;
    var dir_positivey = dir[1] >= 0 ? 1 : 0;
    var leapx = iposx - origin[0] + dir_positivex;
    var leapy = iposy - origin[1] + dir_positivey;
    var length_we = leapx / dir[0];
    var length_ns = leapy / dir[1];
    north_south = length_ns < length_we;
    piposx = iposx;
    piposy = iposy;
    div_start_x = iposx;
    div_start_y = iposy;
    north_south ? (iposy += Math.sign(dir[1]), visual_wall_id += Math.sign(dir[1]) * (VISUAL_W * 2), div_start_y = Math.max(piposy, iposy), length = length_ns) : (iposx += Math.sign(dir[0]), visual_wall_id += Math.sign(dir[0]) * 2, div_start_x = Math.max(piposx, iposx), length = length_we);
    var div_endx = div_start_x + north_south;
    var div_endy = div_start_y + !north_south;
    var cur_wall = get_wall_(world.states[s].map, div_start_x, div_start_y, north_south);
    if (length >= max_len) {
      length = max_len;
      blocked = false;
      break;
    }
    var angled_visual_wall_id = visual_wall_id + north_south;
    if (is_wall(cur_wall)) {
      if (line_renders && !seen_wall(world, angled_visual_wall_id)) line_renders.push({from: vsub2([div_start_x, div_start_y], origin), to: vsub2([div_endx, div_endy], origin), color: 4294967295, trans: composite_transform});
      break;
    } else if (is_teleport(cur_wall)) {
      var opt = world.options[cur_wall & ~1];
      var sign = cur_wall & 1 ? 1 : -1;
      var pos_diff = opt.pos_diff;
      var state_diff = opt.state_diff;
      if (line_renders && !seen_wall(world, angled_visual_wall_id)) line_renders.push({from: vsub2([div_start_x, div_start_y], origin), to: vsub2([div_endx, div_endy], origin), color: 4294901760, trans: composite_transform});
      iposx = iposx + pos_diff[0] - dir_positivex;
      iposy = iposy + pos_diff[1] - dir_positivey;
      origin = vadd2(origin, pos_diff);
      var lined = vmul2(dir, length);
      origin = vsub2(vadd2(origin, lined), lined);
      iposx = iposx + dir[0] >= 0 ? 1 : 0;
      iposy = iposy + dir[1] >= 0 ? 1 : 0;
      s = (s + STATES + state_diff) % STATES;
    }
  }
  return {origin: origin, blocked: blocked, end_s: s, end_pos: vadd2(origin, vmul2(dir, length)), end_ipos: [iposx, iposy], end_div_start: [div_start_x, div_start_y], end_ns: north_south, trans: composite_transform};
}

function sweep(world, origin, aim, renders, line_renders, sky_line_renders) {
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
    sweep5(world, world.current_state, origin, m, 64, renders, line_renders, sky_line_renders);
    i += 1;
  }
}

function current_player_time(world) {
  return time_in_state(world, world.current_state);
}

function world_update(world, action, renders, line_renders, sky_line_renders, paused$0) {
  if (!paused$0) {
    for (var s = 0;s < STATES;++s) {
      var time$0 = time_in_state(world, s);
      state_update(world.states[s], world.frames[time$0]);
      if (time$0 === FRAMES - 1) world.states[s] = clone(world.first_state);
    }
  }
  world.current_player_state.aim = action.aim;
  if (action.act) {
    var act_result = sweep5(world, world.current_state, world.current_player_state.pos, action.aim, 0.3);
    if (act_result.blocked) world.states[world.current_state].switches = 1;
 else {
    }
  }
  if (action.walk[0] || action.walk[1]) {
    var pi = world.current_player_state.pos;
    var walk_dir = vnormalize2(action.walk);
    var result = sweep5(world, world.current_state, world.current_player_state.pos, walk_dir, 0.1);
    if (!result.blocked) {
      var time$0 = current_player_time(world);
      if (time_in_state(world, result.end_s) < time$0) console.log("going back to", time_in_state(world, result.end_s)), world.current_player += 1;
      world.current_state = result.end_s;
      world.current_player_state.pos = result.end_pos;
    }
  }
  if (!paused$0) world.offset += 1;
  var origin = world.current_player_state.pos;
  var aim = world.current_player_state.aim;
  sweep(world, origin, aim, renders, line_renders, sky_line_renders);
  var time = current_player_time(world);
  world.frames[time].player_states[world.current_player] = clone(world.current_player_state);
  if ((time + 1) % SECOND === 0) {
    var seconds = time / SECOND;
    console.log(seconds / 60 | 0, seconds % 60);
  }
}



var draw;
var paused;
function get_cell_name(cell) {
  if (cell === CELL_FLOOR) {
    return "FLOOR";
  } else if (cell === CELL_BLOCK) {
    return "BLOCK";
  }
}

function start(world) {
  var cursor = [0, 0];
  var cursor_dir = true;
  paused = true;
  var editor_selected_cell = CELL_BLOCK;
  var load_map = function() {
    var data = window.localStorage.getItem("hyp__map");
    if (data) world.first_state.map = JSON.parse(data);
  };
  load_map();
  world_reset(world);
  var save_map = function() {
    var data = JSON.stringify(world.first_state.map);
    window.localStorage.setItem("hyp__map", data);
    console.log("saved", data);
  };
  var last_filename = "map.txt";
  var save_map_to_disk = function() {
    var data = JSON.stringify(world.first_state.map);
    var a = document.createElement("a");
    a.download = last_filename;
    a.rel = "noopener";
    var blob = new File([data], last_filename);
    a.href = URL.createObjectURL(blob);
    setTimeout(function() {
      URL.revokeObjectURL(a.href);
    }, 40000);
    setTimeout(function() {
      a.dispatchEvent(new MouseEvent("click"));
      console.log("clicked");
    }, 0);
  };
  var load_map_from_disk = function(text) {
    var map = JSON.parse(text);
    if (map) world.first_state.map = map;
  };
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
  window.addEventListener("keydown", function(ev) {
    if (ev.keyCode === 69) cursor[1] += 1;
 else if (ev.keyCode === 85) cursor[1] -= 1;
    if (ev.keyCode === 65) cursor[0] -= 1;
 else if (ev.keyCode === 79) cursor[0] += 1;
    if (ev.keyCode === 88) cursor_dir = !cursor_dir;
    if (ev.keyCode === 32) {
      var cur_cell = get_wall(world.first_state.map, cursor, cursor_dir);
      var draw_cell = cur_cell === editor_selected_cell ? CELL_FLOOR : editor_selected_cell;
      set_wall(world.first_state.map, cursor, cursor_dir, draw_cell);
    }
    if (ev.keyCode >= 48 && ev.keyCode <= 58) editor_selected_cell = CELL_FLOOR;
    if (ev.keyCode === 9) paused = !paused;
    if (ev.keyCode === 76) save_map(), world_reset(world);
    if (ev.keyCode === 78) save_map_to_disk();
    console.log("key", ev.keyCode);
    return true;
  });
  draw = function(whiteTex$0, fontBits$0, imgShader) {
    color(4294967295);
    setView2([1, 0]);
    activateShader(imgShader);
    var scale_down = 100;
    var drawWall = function(x, y, dir, width) {
      var down = !dir ? -1 : width;
      var up = !dir ? 0 : -width;
      var right = dir ? 1 : width;
      var left = dir ? 0 : -width;
      wall3d(whiteTex$0, scale_down, 10 + x + left, scale_down, 10 + x + right, -y + up, -y + down, 0, 0, 1, 1);
    };
    color(2130706432);
    wall3d(whiteTex$0, scale_down, 10 + 0, scale_down, 10 + 0 + W, -0, -0 - H, 0, 0, 1, 1);
    color(4278255360);
    drawWall(cursor[0], cursor[1], cursor_dir, 0.1);
    for (var my = 0;my < 32;++my) {
      for (var mx = 0;mx < 32;++mx) {
        var step = 1;
        var st = world.first_state;
        var down = get_wall(st.map, [mx, my], false);
        var right = get_wall(st.map, [mx, my], true);
        var width = 0.05;
        color(4294967295);
        if (down === CELL_BLOCK) drawWall(mx, my, false, width);
        if (right === CELL_BLOCK) drawWall(mx, my, true, width);
      }
    }
    color(4278190335);
    var player_pos = world.current_player_state.pos;
    var player_width = 0.1;
    wall3d(whiteTex$0, scale_down, 10 + player_pos[0] - player_width, scale_down, 10 + player_pos[0] + player_width, -player_pos[1] - player_width, -player_pos[1] + player_width, 0, 0, 1, 1);
    drawText(fontBits$0, whiteTex$0, get_cell_name(editor_selected_cell), 100, 10);
  };
}






var fs = main[1];
function genTex(pixels, ty, f) {
  var side = Math.sqrt(pixels.length);
  var w = side;
  var h = side;
  var i = 0;
  var y = 0;
  for (;y < h;) {
    var x = 0;
    for (;x < w;) pixels[i] = f(x, y), x += 1, i += 1;
    y += 1;
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
  var canvas = document.getElementById("g");
  init(canvas);
  gl$0 = gl;
  skyShader = createShaderProgram(basicVs, main[0]);
  imgShader = createShaderProgram(basicVs, fs);
  translucentShader = createShaderProgram(basicVs, main[2]);
  function getPointColor(x, y) {
    var alpha = (7.5 - Math.hypot(x - 8, y - 8)) * 64 | 0;
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
      pan.pan.value = 0.8;
      src.buffer = buf;
      src.connect(gain).connect(pan).connect(audio.destination);
      src.start();
    }
  }

  canvas.onclick = function() {
    if (!audio) {
      audio = new AudioContext();
      var bufSize = 16384;
      var scriptProc = audio.createScriptProcessor(bufSize, 0, 1);
      var time = 0;
      var tick = 0;
      var start$0 = 44100 * 2;
      var beep = Array(44100 >> 2).fill(0).map(function(s, x) {
        return 0.2 * Math.sin(220 * 2 * Math.PI * x / 44100);
      });
      var samples = [];
      function play1() {
        samples.push([tick, beep]);
      }

      console.log(beep);
      var t = 0;
      scriptProc.onaudioprocess = function(e) {
        var left = e.outputBuffer.getChannelData(0);
        for (var i = 0;i < bufSize;++i) left[i] = "%,IW7:A".charCodeAt(i % 7) * t % 0.1 * (1 - t / (Math.tan(i % 7) + 9) % 1), t += 0.00002;
        tick += bufSize;
      };
      var buf = audio.createBuffer(1, beep.length, 44100);
      buf.getChannelData(0).set(beep);
      play2(buf);
    }
    canvas.requestPointerLock();
  };
  canvas.onmousemove = function(e) {
    if (document.pointerLockElement === canvas) {
      var xdiff = e.movementX / 500;
      aim = vrotate(aim, [Math.cos(xdiff), Math.sin(xdiff)]);
    }
  };
  start(world);
  function update() {
    window.requestAnimationFrame(function(currentTime) {
      update();
      var walkSpeed = 0.1;
      var walk = [0, 0];
      if (keys[71]) walk = vadd2(walk, vmul2(aim, walkSpeed));
 else if (keys[83]) walk = vsub2(walk, vmul2(aim, walkSpeed));
      if (keys[68]) walk = vadd2(walk, vmul2([aim[1], -aim[0]], walkSpeed));
 else if (keys[84]) walk = vsub2(walk, vmul2([aim[1], -aim[0]], walkSpeed));
      var renders = [];
      var line_renders = [];
      var sky_line_renders = [];
      world_update(world, {aim: aim, walk: walk, act: keys[81]}, renders, line_renders, sky_line_renders, paused);
      canvas_w = canvas.width = window.innerWidth;
      canvas_h = canvas.height = window.innerHeight;
      gl$0.viewport(0, 0, gl$0.drawingBufferWidth, gl$0.drawingBufferHeight);
      gl$0.clear(GL_COLOR_BUFFER_BIT);
      color(4294967295);
      setView2(aim);
      activateShader(translucentShader);
      renders.forEach(function(r) {
        var step = 100;
        var z = (-r.ul[0] - 1) * step;
        var x = r.ul[1] * step;
        wall3d(pointTex, x + step, z, x, z, -step / 2, step / 2, 0, 0, 1, 1);
        wall3d(pointTex, x, z, x, z + step, -step / 2, step / 2, 0, 0, 1, 1);
        wall3d(pointTex, x + step, z + step, x + step, z, -step / 2, step / 2, 0, 0, 1, 1);
        wall3d(pointTex, x, z + step, x + step, z + step, -step / 2, step / 2, 0, 0, 1, 1);
      });
      line_renders.sort(function(a, b) {
        var len_a = a.from[0] * a.from[0] + a.from[1] * a.from[1];
        var len_b = b.from[0] * b.from[0] + b.from[1] * b.from[1];
        if (len_a < len_b) {
          return -1;
        } else if (len_a === len_b) {
          return 0;
        } else {
          return 1;
        }
      });
      line_renders.forEach(function(r) {
        var step = 100;
        var x = r.from[0] * step;
        var z = r.from[1] * step;
        var x2 = r.to[0] * step;
        var z2 = r.to[1] * step;
        color(r.color);
        wall3d(whiteTex, x, z, x2, z2, -step / 2, step / 2, 0, 0, 1, 1);
      });
      flush();
      draw(whiteTex, fontBits, imgShader);
      color(4294967295);
      var now = current_player_time(world);
      var second = TIME_IN_SECONDS - now / SECOND | 0;
      var minute = second / 60 | 0;
      second %= 60;
      drawText(fontBits, whiteTex, minute + " " + second, -200, 250);
      flush();
    });
  }

  update();
}

window.onload = function() {
  startGame();
};
var data = "dGVzdA==";


function foo([x, y]) {
  return {x: x, y: x};
}

})(this)