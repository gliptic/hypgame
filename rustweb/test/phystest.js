(function(window){
var vertex = `varying vec3 e;
varying vec2 i;
varying vec4 j;
attribute vec3 a;
attribute vec2 c;
attribute vec4 d;
uniform float f;
uniform mat3 g;
uniform mat3 h;
void main() {
  e = a;
  i = c;
  j = d;
  vec3 b = h * g * a;
  gl_Position = mat4(1.83, 0, 0, 0, 0, 1.83 * f, 0, 0, 0, 0, -1, -1, 0, 0, -2, 0) * vec4(b.xyz, 1);
}

`;
var main = [`texture2D(a, i) * j;
}

`,`vec4(j.rgb, .5);
}

`].map(function (a) { return `varying vec2 i;
varying vec4 j;
uniform sampler2D a;
uniform float b;
void main() {
  gl_FragColor = ` + a; });
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
var viewTrans2;
var time = 0;
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
  gl.uniform1f(gl.getUniformLocation(shader, "f"), canvas_w / canvas_h);
  gl.uniformMatrix3fv(gl.getUniformLocation(shader, "g"), 0, viewTrans);
  gl.uniformMatrix3fv(gl.getUniformLocation(shader, "h"), 0, viewTrans2);
}

function color(c) {
  col = c;
}

function setView2([aimx, aimy], [aimvx, aimvy]) {
  viewTrans = [-aimy, 0, -aimx, 0, 1, 0, aimx, 0, -aimy];
  viewTrans2 = [1, 0, 0, 0, aimvx, aimvy, 0, -aimvy, aimvx];
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

function drawText(fontBits$0, whiteTex$0, text, x, y, z, dirx, dirz, diry, scale$0) {
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
          wall3d(whiteTex$0, z + dirz * step * abs_column, x + dirx * step * abs_column, z + dirz * step * (abs_column + 1), x + dirx * step * (abs_column + 1), y + diry * (row + 1) * step, y + diry * row * step, 0, 0, 1, 1);
        }
      }
    }
  }
}




var fs = main[0];
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
  var skyMirrorShader;
  var gridShader;
  var fb;
  var canvas = document.getElementById("g");
  init(canvas);
  gl$0 = gl;
  imgShader = createShaderProgram(basicVs, fs);
  function getPointColor(x, y) {
    var alpha = (7.5 - Math.hypot(x - 8, y - 8)) * 64 | 0;
    return 1052927 + (clamp(alpha, 0, 255) << 24);
  }

  var pointTex = genTex(new Uint32Array(16 * 16), GL_RGBA, getPointColor);
  whiteTex = genTex(new Uint32Array(1), GL_RGBA, function() {
    return 4294967295;
  });
  var world = state.world_create();
  var keys = {};
  window.onkeydown = function(ev) {
    keys[ev.keyCode] = 1;
  };
  window.onkeyup = function(ev) {
    keys[ev.keyCode] = 0;
  };
  canvas.onclick = function() {
    canvas.requestPointerLock();
  };
  canvas.onmousemove = function(e) {
    if (document.pointerLockElement === canvas) {
      var xdiff = e.movementX / 500;
      var ydiff = e.movementY / 500;
    }
  };
  var nextFrame = 0;
  function update() {
    window.requestAnimationFrame(function(currentTime) {
      update();
      time += 1 / 60;
      var speed = keys[16] ? 5 : 1;
      for (var i = 0;i < speed;++i) {
      }
      gl$0.viewport(0, 0, gl$0.drawingBufferWidth, gl$0.drawingBufferHeight);
      gl$0.clear(GL_COLOR_BUFFER_BIT);
      activateShader(imgShader);
      setView2([1, 0], [1, 0]);
      wall3d(whiteTex, 10, 0, 20, 0, 10, 20, 0, 0, 1, 1);
      flush();
    });
  }

  update();
}

window.onload = function() {
  startGame();
};
})(this)