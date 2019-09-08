use crate::{JsAst, JsLit, JsOp, JsUnop, JsBuiltin, JsPattern};
//use crate::glsl_bundler::{self, GlslBundler, GlslCollection};
use crate::hyp::{self, ModuleInfo, Ident, Language, AstLambda};
use crate::{hyp_to_js, glsl_bundler};
use std::collections::{HashMap, HashSet};
use crate::conflict_tree::ConflictTree;
use crate::binary;

// TODO: Move JsBundler to own file
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prec(u32, i32);

impl Prec {
    fn need_paren(self, slot: Prec) -> bool {
        self.0 > slot.0 || (self.0 == slot.0 && self.1 != slot.1)
    }

    fn on_left(self) -> Prec {
        Prec(self.0, -1)
    }

    fn on_right(self) -> Prec {
        Prec(self.0, 1)
    }

    fn on_either(self) -> Prec {
        Prec(self.0, 0)
    }
}

const PREC_DOT_BRACKET: Prec = Prec(1, -1);
const PREC_NEW: Prec = Prec(1, 1);
const PREC_CALL: Prec = Prec(2, -1);
const PREC_MUL_DIV_REM: Prec = Prec(5, -1);
const PREC_PLUS_MINUS: Prec = Prec(6, -1);
const PREC_SHIFT: Prec = Prec(7, -1);
const PREC_INEQ: Prec = Prec(8, -1);
const PREC_EQ: Prec = Prec(9, -1);
const PREC_BITAND: Prec = Prec(10, -1);
const PREC_BITXOR: Prec = Prec(11, -1);
const PREC_BITOR: Prec = Prec(12, -1);
const PREC_ANDAND: Prec = Prec(13, -1);
const PREC_OROR: Prec = Prec(14, -1);
const PREC_SELECT: Prec = Prec(15, 1);
const PREC_ASSIGN: Prec = Prec(16, 1);
const PREC_COMMA: Prec = Prec(17, -1);
const PREC_MAX: Prec = Prec(18, 0);
const PREC_UNARY_PLUS_MINUS: Prec = Prec(4, 1);
const PREC_PRE_INC_DEC: Prec = Prec(3, 0);


// (a + b) * c
//   6 > 5, need ()
// (a * b) * c
//   6 = 6 and on left side, no ()
// a + (b * c)
//   5 < 6, no ()

#[derive(PartialEq)]
pub enum NeedSemi { No, Yes }

/*
pub struct ModuleInfo {
    pub name: String,
    pub export_names: Vec<String>,
    pub mapped_names: Vec<String>,
    pub import_map: Vec<u32>
}

impl ModuleInfo {
    pub fn from_module(module: JsModule) -> (ModuleInfo, Vec<JsAst>) {
        
        let JsModule { name, exports, items, import_map, .. } = module;

        let module_info = ModuleInfo {
            mapped_names: exports.iter()
                .map(|n| format!("{}_{}", &name, n)).collect(),
            name: name,
            export_names: exports,
            import_map
        };

        (module_info, items)
    }
}*/

pub struct JsBundler<'a> {
    pub buf: String,
    indent_len: u32,
    pub current_module: usize,
    pub module_infos: &'a mut Vec<ModuleInfo>,
    pub module_lambdas: &'a Vec<AstLambda>,
    pub glsl_fn_map: HashMap<(u32, u32), usize>,

    //pub glsl_collection: Option<GlslCollection>,
    pub wasm_offsets: HashMap<String, u64>,
}

fn other(thing: &mut Vec<ModuleInfo>) {
}

impl<'a> JsBundler<'a> {
    pub fn new<'b>(module_infos: &'b mut Vec<ModuleInfo>, module_lambdas: &'b Vec<AstLambda>) -> JsBundler<'b> {
        JsBundler {
            buf: String::new(),
            indent_len: 0,
            module_infos,
            module_lambdas,
            glsl_fn_map: HashMap::new(),
            //glsl_collection: None,
            wasm_offsets: HashMap::new(),
            current_module: 0
        }
    }

    pub fn resolve_name_conflicts(&mut self) {
        
        fn clone_and_annotate(ct: &ConflictTree<u32>, abs_index: u32) -> ConflictTree<(u32, u32)> {
            let mut new_ct = ConflictTree::new();
            new_ct.items = ct.items.iter().map(|&(id, count)| ((abs_index, id), count)).collect();
            new_ct.children = ct.children.iter()
                .map(|ch| clone_and_annotate(ch, abs_index)).collect();
            new_ct
        }

        fn rename(
            module_infos: &mut [hyp::ModuleInfo],
            node: &ConflictTree<(u32, u32)>,
            seen: &mut HashSet<String>) {

            for &((abs_index, local_index), _) in &node.items {
                let local_name = &mut module_infos[abs_index as usize].locals[local_index as usize].name;
                //seen.insert(local_name.clone());

                if seen.contains(local_name) {
                    for i in 0.. {
                        let new_name = format!("{}${}", local_name, i);
                        if !seen.contains(&new_name) {
                            *local_name = new_name.clone();
                            seen.insert(new_name);
                            break;
                        }
                    }
                } else {
                    seen.insert(local_name.clone());
                }
            }

            for ch in &node.children {
                rename(module_infos, ch, seen);
            }

            for &((abs_index, local_index), _) in &node.items {
                let local_name = &module_infos[abs_index as usize].locals[local_index as usize].name;
                seen.remove(local_name);
            }
        }

        let mut seen = HashSet::new();
        let mut bundled_ct = ConflictTree::new();

        for (abs_index, m) in self.module_infos.iter_mut().enumerate() {
            if m.language == hyp::Language::Js {
                bundled_ct.append(clone_and_annotate(&m.conflict_tree, abs_index as u32));
            }
        }

        rename(&mut self.module_infos, &bundled_ct, &mut seen);
    }

    pub fn write_glsl(&mut self, glsl: HashMap<usize, glsl_bundler::BundledModule>) {
        
        for (abs_index, mut bundled_module) in glsl {
            let module = &self.module_infos[abs_index];

            self.buf.push_str("var ");
            self.buf.push_str(&module.name);
            self.buf.push_str(" = ");

            if bundled_module.exported_functions.len() != 1 {
                self.buf.push_str("[");

                // Figure out common prefix because it's a free win

                'shift_loop: loop {
                    let mut it = bundled_module.exported_functions.iter_mut();
                    let first_char;
                    if let Some((first_src, _)) = it.next() {
                        if let Some(ch) = first_src.chars().next() {
                            first_char = ch;
                        } else {
                            break 'shift_loop;
                        }
                    } else {
                        break 'shift_loop;
                    }

                    while let Some((func_src, _)) = it.next() {
                        if let Some(ch) = func_src.chars().next() {
                            if ch != first_char {
                                break 'shift_loop;
                            }
                        } else {
                            break 'shift_loop;
                        }
                    }

                    // All sources have the same first char, remove it and put it last in prefix
                    for (func_src, _) in bundled_module.exported_functions.iter_mut() {
                        func_src.remove(0);
                    }
                    bundled_module.prefix.push(first_char);
                }

                for (i, (func_src, exported_index)) in bundled_module.exported_functions.into_iter().enumerate() {
                    if i > 0 {
                        self.buf.push_str(",");
                    }
                    self.str_lit(&func_src);
                    self.glsl_fn_map.insert((abs_index as u32, exported_index), i);
                }

                self.buf.push_str("].map(function (a) { return ");
                self.str_lit(&bundled_module.prefix);
                self.buf.push_str(" + a; });\n");
            } else {
                let (func_src, exported_index) = bundled_module.exported_functions.remove(0);
                self.glsl_fn_map.insert((abs_index as u32, exported_index), std::usize::MAX);
                self.str_lit(&(bundled_module.prefix + &func_src));
                self.buf.push_str(";\n");
            }
        }
    }

    pub fn run(&mut self, debug: bool) {

        self.resolve_name_conflicts();
        let module_order = self.find_module_ordering();

        self.write_iife_begin_plain();

        let glsl = glsl_bundler::GlslBundler::bundle(&mut self.module_infos, &self.module_lambdas, &module_order, debug);
        self.write_glsl(glsl);

        let bin = binary::bundler::Bundler::bundle(&mut self.module_infos);
        self.write_bin(bin);

        // TODO: Combine together e.g. binaries and process as a unit

        for order_index in 0..module_order.len() {
            let module_index = module_order[order_index];

            println!("bundling #{}: {}", module_index, &self.module_infos[module_index as usize].name);

            let lang = self.module_infos[module_index as usize].language;

            if lang == hyp::Language::Js {
                let mut enc = hyp_to_js::JsEnc::new();
                enc.parse_hyp(&self.module_lambdas[module_index as usize].expr);
                if enc.errors.len() > 0 {
                    for err in &enc.errors {
                        let mi = &self.module_infos[module_index as usize];
                        // TODO!!
                        mi.print_line_at(err);
                    }
                    panic!("errors in hyp to js conversion");
                    // TODO!!
                    //return Err(());
                }
                self.current_module = module_index as usize;
                self.stmts_inner_to_js(&enc.module.items, PREC_MAX, false);
            }
        }

        self.write_iife_end();
    }

    pub fn wrap_p(&mut self, slot: Prec, op: Prec) -> bool {
        let p = op.need_paren(slot);
        if p {
            self.buf.push_str("(");
        }
        p
    }

    pub fn unwrap_p(&mut self, f: bool) {
        if f {
            self.buf.push_str(")");
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indent_len {
            self.buf.push_str("  ");
        }
    }

    pub fn find_module_ordering(&self) -> Vec<u32> {
        let modules_len = self.module_infos.len();
        let mut marks = vec![0u8; modules_len];

        fn visit(cur: usize, infos: &Vec<ModuleInfo>, marks: &mut [u8], output: &mut Vec<u32>) {
            if marks[cur] == 2 { return }
            if marks[cur] == 1 {
                println!("cyclic reference!");
                return;
            }

            marks[cur] = 1;
            for &c in &infos[cur].import_map {
                visit(c as usize, infos, marks, output);
            }
            marks[cur] = 2;
            output.push(cur as u32);
        }

        let mut output = Vec::new();

        for i in 0..modules_len {
            if marks[i] != 2 {
                visit(i, self.module_infos, &mut marks, &mut output);
            }
        }

        output
    }

    pub fn push_module_name(&mut self, abs_index: u32) {
        let name = &self.module_infos[abs_index as usize].name;
        self.buf.push_str(name);
    }

    pub fn write_iife_begin_plain(&mut self) {
        self.buf.push_str(r#"(function(window){
"#);
    }

    pub fn write_iife_begin(&mut self, wasm_data: &[u8], wasm_imports: &[String]) {
        self.buf.push_str(r#"
(function(window){
var m = new WebAssembly.Module(Uint8Array.from(atob('"#);

        base64::encode_config_buf(wasm_data, base64::STANDARD, &mut self.buf);

        self.buf.push_str(r#"').split('').map(x => x.charCodeAt(0))));
var wasm = new WebAssembly.Instance(m, { i: {"#);

        let mut first = true;
        for i in wasm_imports {
            if first {
                first = false;
            } else {
                self.buf.push_str(", ");
            }
            self.buf.push_str(i);
            self.buf.push_str(": ");
            self.buf.push_str(i);
        }

        self.buf.push_str(r#"} }).exports;
"#);
    }

    pub fn write_iife_end(&mut self) {
        self.buf.push_str("})(this)");
    }

    pub fn stmts_inner_to_js(&mut self, stmts: &[JsAst], slot: Prec, commas: bool) {
        let mut first = true;
        let prec = if stmts.len() == 1 { slot } else { PREC_COMMA };

        for s in stmts {
            if first {
                first = false;
            } else if commas {
                self.buf.push_str(", ");
            }

            if !commas {
                self.indent();
            }

            let need_semi = self.to_js(s, prec, commas);
            if !commas {
                if need_semi == NeedSemi::Yes {
                    self.buf.push_str(";");
                }
                self.buf.push_str("\n");
            }
        }
    }

    pub fn stmts_to_js(&mut self, stmts: &[JsAst], can_skip_braces: bool, slot: Prec) -> NeedSemi {
        if can_skip_braces && stmts.len() > 0
         && stmts.iter().all(|x| x.is_expr()) {
            let p = stmts.len() != 1 && self.wrap_p(slot, PREC_COMMA);
            self.stmts_inner_to_js(stmts, slot, true);
            self.unwrap_p(p);
            NeedSemi::Yes
        } else {
            self.buf.push_str("{\n");
            self.indent_len += 1;
            self.stmts_inner_to_js(stmts, PREC_MAX, false);
            self.indent_len -= 1;
            self.indent();
            self.buf.push_str("}");
            NeedSemi::No
        }
    }

    pub fn str_lit(&mut self, s: &str) {
        // TODO: Escape
        if s.find('\r').is_some() ||
            s.find('\n').is_some() {
            self.buf.push_str(&format!("`{}`", s));
        } else {
            self.buf.push_str(&format!("\"{}\"", s));
        }
    }

    pub fn pattern_to_js(&mut self, pat: &JsPattern) {
        match pat {
            JsPattern::Local(i) => {
                let param_name = &self.module_infos[self.current_module].locals[*i as usize].name;
                self.buf.push_str(param_name);
            }
            JsPattern::Array(subpats) => {
                self.buf.push_str("[");
                let mut first = true;
                for e in subpats {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.pattern_to_js(e);
                }
                self.buf.push_str("]");
            }
        }
    }

    pub fn write_bin(&mut self, bun: binary::BundledModules) {
        if bun.js_safe.len() > 0 || bun.base64.len() > 0 {
            self.buf.push_str("var $bin=(");
            //let mi = &self.module_infos[self.current_module];
            if bun.js_safe.len() > 0 {
                self.buf.push_str("\"");
                self.buf.push_str(std::str::from_utf8(&bun.js_safe[..]).unwrap());
                self.buf.push_str("\"");
            }

            if bun.base64.len() > 0 {
                if bun.js_safe.len() > 0 {
                    self.buf.push_str("+");
                }

                self.buf.push_str("atob(\"");
                base64::encode_config_buf(&bun.base64, base64::STANDARD, &mut self.buf);
                while self.buf.as_bytes().last().unwrap() == &b'=' {
                    // The '=' is unnecessary
                    self.buf.pop();
                }
                self.buf.push_str("\")");
            }
            self.buf.push_str(").split('').map(x=>x.charCodeAt(0));");
        }
    }

    pub fn binary_to_js(&mut self) {
        let mi = &self.module_infos[self.current_module];
        self.buf.push_str("var ");
        self.buf.push_str(&mi.name);
        self.buf.push_str(" = ");
        self.buf.push_str("\"");
        // TODO: Remove unnecessary = at the end
        base64::encode_config_buf(&mi.src, base64::STANDARD, &mut self.buf);
        self.buf.push_str("\";");
    }

    pub fn to_js(&mut self, ast: &JsAst, slot: Prec, must_be_expr: bool) -> NeedSemi {
        match ast {
            JsAst::Fn { index, expr, args, .. } => {
                self.buf.push_str("function ");
                let name = &self.module_infos[self.current_module].locals[*index as usize].name;
                self.buf.push_str(&name);
                self.buf.push_str("(");
                let mut first = true;
                for i in args {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    //let param_name = &self.module_infos[self.current_module].locals[*i as usize].1;
                    //self.buf.push_str(param_name);
                    self.pattern_to_js(i);
                }
                self.buf.push_str(") ");
                self.to_js(&expr, PREC_MAX, false);
                self.buf.push_str("\n");
                NeedSemi::No
            },
            JsAst::Path { path } => {
                let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                let mut first = true;
                for piece in path {
                    if first {
                        first = false;
                    } else {
                        self.buf.push('.');
                    }
                    self.buf.push_str(piece);
                }
                
                self.unwrap_p(p);
                NeedSemi::Yes
            },
            JsAst::Builtin { builtin } => {
                match builtin {
                    JsBuiltin::Wasm => {
                        self.buf.push_str("wasm");
                        NeedSemi::Yes
                    }
                    _ => {
                        panic!("this builtin cannot be reified");
                    }
                }
                
            }
            JsAst::Call { func, args } => {
                self.to_js(&func, PREC_CALL, true);
                self.buf.push('(');
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.to_js(arg, PREC_COMMA.on_either(), true);
                }
                self.buf.push(')');
                NeedSemi::Yes
            },
            JsAst::Lit { lit } => {
                match lit {
                    JsLit::Int(v) => {
                        self.buf.push_str(&format!("{}", v));
                    },
                    JsLit::Str(v) => {
                        // TODO: Escape
                        self.str_lit(v);
                    }, 
                    JsLit::Bool(v) => {
                        self.buf.push_str(if *v { "true" } else { "false" });
                    },
                    JsLit::Float(v) => {
                        self.buf.push_str(&format!("{}", v));
                    }
                }
                NeedSemi::Yes
            },
            JsAst::Block { stmts } => {
                //self.stmts_to_js(stmts);
                //NeedSemi::No
                self.stmts_to_js(stmts, slot != PREC_MAX, slot)
            }
            JsAst::NewObject { assignments } => {
                self.buf.push('{');
                let mut first = true;
                for (name, value) in assignments {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.buf.push_str(name);
                    self.buf.push_str(": ");
                    self.to_js(value, PREC_COMMA.on_either(), true);
                }
                self.buf.push('}');
                NeedSemi::Yes
            }
            JsAst::NewCtor { ctor, params } => {
                self.buf.push_str("new ");
                self.to_js(ctor, PREC_NEW.on_left(), true);
                self.buf.push('(');
                let mut first = true;
                for value in params {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.to_js(value, PREC_COMMA.on_either(), true);
                }
                self.buf.push(')');
                NeedSemi::Yes
            }
            JsAst::Use { name: _name, rel_index: _rel_index } => {
                /*
                let abs_index = self.module_infos[self.current_module].import_map[*rel_index as usize];
                let module = &self.module_infos[abs_index as usize];
                let module_lambda = &self.module_lambdas[abs_index as usize];

                if module.language == Language::Glsl {
                    println!("Compiling glsl {}", &module.name);
                    let mut enc = hyp_to_glsl::GlslEnc::new();
                    enc.parse_hyp(&module_lambda.expr);

                    self.buf.push_str("var ");
                    self.buf.push_str(&module.name);
                    self.buf.push_str(" = ");

                    let main = {
                            let mut glsl_bundler = glsl_bundler::GlslBundler::new(self.module_infos, -1);
                            glsl_bundler.current_module = abs_index as usize;
                            glsl_bundler.module_to_glsl(&enc.module);
                            glsl_bundler.buf.buf
                        };

                    let mut exported_functions = Vec::new();

                    //let mut array_index = 0;
                    for &exported_index in &module.exports {
                        if module.locals[exported_index as usize].0.is_fn() {
                            let mut glsl_bundler = glsl_bundler::GlslBundler::new(self.module_infos, exported_index as i32);
                            glsl_bundler.current_module = abs_index as usize;
                            glsl_bundler.module_to_glsl(&enc.module);
                            
                            exported_functions.push((glsl_bundler.buf.buf, exported_index));
                            
                            //array_index += 1;
                        }
                    }

                    if exported_functions.len() != 1 {
                        self.buf.push_str("[");

                        for (i, (f, exported_index)) in exported_functions.into_iter().enumerate() {
                            if i > 0 {
                                self.buf.push_str(",");
                            }
                            self.str_lit(&f);
                            self.glsl_fn_map.insert((abs_index, exported_index), i);
                        }

                        self.buf.push_str("].map(function (a) { return ");
                        self.str_lit(&main);
                        self.buf.push_str(" + a; });");
                    } else {
                        let (f, exported_index) = exported_functions.remove(0);
                        self.glsl_fn_map.insert((abs_index, exported_index), std::usize::MAX);
                        self.str_lit(&(main + &f));
                        self.buf.push_str(";");
                    }
                }
                */
                NeedSemi::No
            },
            &JsAst::Global { index, constant, ref value } => {
                let name = &self.module_infos[self.current_module].locals[index as usize].name;
                // TODO: Use var instead of const for compactness?
                self.buf.push_str(if constant { "const " } else { "var " });
                self.buf.push_str(&name);
                self.buf.push_str(" = ");
                self.to_js(value, PREC_ASSIGN.on_right(), true);
                self.buf.push_str(";");
                NeedSemi::No
            },
            JsAst::Locals { local_indexes, values } => {
                let mut first = true;
                self.buf.push_str("var ");
                for i in 0..local_indexes.len() {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    let index = local_indexes[i];
                    let name = &self.module_infos[self.current_module].locals[index as usize].name;
                    
                    self.buf.push_str(name);
                    if i < values.len() {
                        self.buf.push_str(" = ");
                        self.to_js(&values[i], PREC_ASSIGN.on_right(), true);
                    }
                }
                self.buf.push_str(";");
                NeedSemi::No
            },
            JsAst::Assign { left, right } => {
                let p = self.wrap_p(slot, PREC_ASSIGN);
                self.to_js(left, PREC_ASSIGN.on_left(), true);
                self.buf.push_str(" = ");
                self.to_js(right, PREC_ASSIGN.on_right(), true);
                self.unwrap_p(p);
                NeedSemi::Yes
            },
            JsAst::Array { elems } => {
                let mut first = true;
                self.buf.push('[');
                for e in elems {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.to_js(e, PREC_COMMA.on_either(), true);
                }
                self.buf.push(']');
                NeedSemi::Yes
            },
            JsAst::If { cond, then_branch, else_branch } => {
                if must_be_expr && ast.is_expr() {
                    let p = self.wrap_p(slot, PREC_SELECT);
                    self.to_js(cond, PREC_MAX, true);
                    self.buf.push_str(" ? ");
                    // TODO: Avoid clone etc.
                    self.to_js(&JsAst::Block { stmts: (*then_branch).clone() }, PREC_SELECT, true);
                    self.buf.push_str(" : ");
                    if let Some(else_ast) = else_branch {
                        self.to_js(else_ast, PREC_SELECT, true);
                    } else {
                        // TODO: Use some other literal based on what the type on the other side is
                        self.buf.push_str("0");
                    }
                    self.unwrap_p(p);
                    NeedSemi::Yes
                } else {
                    self.buf.push_str("if (");
                    self.to_js(cond, PREC_MAX, true);
                    self.buf.push_str(") ");
                    let mut need_semi = self.stmts_to_js(then_branch, true, PREC_MAX);
                    match else_branch {
                        Some(e) => {
                            if need_semi == NeedSemi::Yes {
                                self.buf.push_str(";\n");
                            }
                            self.buf.push_str(" else ");
                            need_semi = self.to_js(e, PREC_MAX, false);
                        },
                        None => {}
                    }
                    need_semi
                }
            },
            JsAst::Break => {
                self.buf.push_str("break");
                NeedSemi::Yes
            }
            JsAst::While { cond, body } => {
                self.buf.push_str("for (;");
                self.to_js(cond, PREC_MAX, false);
                self.buf.push_str(";) ");
                self.stmts_to_js(body, true, PREC_MAX)
            }
            JsAst::For { pre, cond, post, body } => {
                self.buf.push_str("for (");
                if self.to_js(pre, PREC_MAX, false) == NeedSemi::Yes {
                    self.buf.push_str(";");
                }
                self.to_js(cond, PREC_MAX, true);
                self.buf.push_str(";");
                self.to_js(post, PREC_MAX, true);
                self.buf.push_str(") ");
                self.stmts_to_js(body, true, PREC_MAX)
            }
            JsAst::Loop { body } => {
                self.buf.push_str("for (;;) ");
                self.stmts_to_js(body, true, PREC_MAX)
            },
            JsAst::Unary { value, op } => {
                let prec = match op {
                    JsUnop::Not | JsUnop::Plus | JsUnop::Minus | JsUnop::BitNot => PREC_UNARY_PLUS_MINUS,
                    JsUnop::PreInc | JsUnop::PreDec => PREC_PRE_INC_DEC,
                };

                let p = self.wrap_p(slot, prec);
                self.buf.push_str(match op {
                    JsUnop::Plus => "+",
                    JsUnop::Minus => "-",
                    JsUnop::BitNot => "~",
                    JsUnop::Not => "!",
                    JsUnop::PreInc => "++",
                    JsUnop::PreDec => "--",
                });
                self.to_js(value, prec.on_right(), true);

                self.unwrap_p(p);
                NeedSemi::Yes
            },
            JsAst::Binary { left, op, right } => {
                let prec = match op {
                    JsOp::Mul | JsOp::Div | JsOp::Rem => PREC_MUL_DIV_REM,
                    JsOp::Add | JsOp::Sub => PREC_PLUS_MINUS,
                    JsOp::MulEq | JsOp::DivEq | JsOp::RemEq | JsOp::AddEq | JsOp::SubEq
                      | JsOp::OrEq | JsOp::AndEq | JsOp::XorEq
                        => PREC_ASSIGN,
                    JsOp::Eq | JsOp::Ne
                        => PREC_EQ,
                    JsOp::Lt | JsOp::Le | JsOp::Gt | JsOp::Ge
                        => PREC_INEQ,
                    JsOp::BitAnd => PREC_BITAND,
                    JsOp::BitOr => PREC_BITOR,
                    JsOp::BitXor => PREC_BITXOR,
                    JsOp::AndAnd => PREC_ANDAND,
                    JsOp::OrOr => PREC_OROR,
                    JsOp::Shl | JsOp::Shr | JsOp::Lshr => PREC_SHIFT,

                };

                let p = self.wrap_p(slot, prec);
                self.to_js(left, prec.on_left(), true);
                self.buf.push_str(match op {
                    JsOp::Mul => " * ",
                    JsOp::Div => " / ",
                    JsOp::Rem => " % ",
                    JsOp::Add => " + ",
                    JsOp::Sub => " - ",
                    JsOp::MulEq => " *= ",
                    JsOp::DivEq => " /= ",
                    JsOp::RemEq => " %= ",
                    JsOp::OrEq => " |= ",
                    JsOp::AndEq => " &= ",
                    JsOp::XorEq => " ^= ",
                    JsOp::AddEq => " += ",
                    JsOp::SubEq => " -= ",
                    JsOp::BitAnd => " & ",
                    JsOp::BitOr => " | ",
                    JsOp::BitXor => " ^ ",
                    JsOp::AndAnd => " && ",
                    JsOp::OrOr => " || ",
                    JsOp::Shl => " << ",
                    JsOp::Shr => " >> ",
                    JsOp::Lshr => " >>> ",
                    JsOp::Eq => " === ",
                    JsOp::Lt => " < ",
                    JsOp::Le => " <= ",
                    JsOp::Ne => " !== ",
                    JsOp::Gt => " > ",
                    JsOp::Ge => " >= ",
                });
                self.to_js(right, prec.on_right(), true);
                self.unwrap_p(p);
                NeedSemi::Yes
            },
            JsAst::Field { base, member } => {
                match **base {
                    /*
                    JsAst::Builtin { builtin: JsBuiltin::Glsl } => {
                        let GlslCollection(glsl_bun, glsl_modules) = self.glsl_collection.as_mut().unwrap();

                        let mut glsl_src = String::new();

                        for (module_index, m) in glsl_modules.iter().enumerate() {
                            if &m.name == member {
                                glsl_bun.current_module = module_index as usize;
                                glsl_bun.module_to_glsl(&m);

                                std::mem::swap(&mut glsl_bun.buf.buf, &mut glsl_src);
                            }
                        }

                        self.str_lit(&glsl_src);
                    }*/
                    JsAst::Builtin { builtin: JsBuiltin::Wasm } => {
                        if let Some(offs) = self.wasm_offsets.get(member) {
                            self.buf.push_str(&format!("{}", offs));
                        } else {
                            let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                            self.buf.push_str("wasm.");
                            self.buf.push_str(member);
                            self.unwrap_p(p);
                        }
                    }
                    _ => {
                        if member == "new" {
                            self.buf.push_str("new ");
                            self.to_js(base, PREC_NEW.on_left(), true);
                        } else {
                            let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                            self.to_js(base, PREC_DOT_BRACKET.on_left(), true);
                            self.buf.push('.');
                            self.buf.push_str(member);
                            self.unwrap_p(p);
                        }
                    }
                }
                NeedSemi::Yes
            },
            JsAst::Index { expr, index } => {
                let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                self.to_js(expr, PREC_DOT_BRACKET.on_left(), true);
                self.buf.push('[');
                self.to_js(index, PREC_MAX, true);
                self.buf.push(']');
                self.unwrap_p(p);
                NeedSemi::Yes
            },
            JsAst::MethodCall { receiver, method, args } => {

/*
                let debug_gl = false;

                if debug_gl {
                    match &**receiver { // TEMP
                        JsAst::SelfMember { index }
                        if &self.module_infos[self.current_module].mapped_names[*index as usize] == "render_gl"
                        && method != "getError" => {
                            self.buf.push_str("render_checkErr");
                        },
                        _ => {}
                    }

                    self.buf.push('('); // TEMP
                }*/

                if method == "new" {
                    self.buf.push_str("new ");
                    self.to_js(receiver, PREC_NEW.on_left(), true);
                } else {
                    self.to_js(receiver, PREC_DOT_BRACKET.on_left(), true);
                    self.buf.push('.');
                    self.buf.push_str(method);
                }
                self.buf.push('(');
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.to_js(arg, PREC_COMMA.on_either(), true);
                }
                self.buf.push(')');

/*
                if debug_gl {
                    self.buf.push(')'); // TEMP
                }
                */
                NeedSemi::Yes
            },
            JsAst::Lambda { inputs, body } => {
                self.buf.push_str("function(");
                let mut first = true;

                for i in inputs {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.pattern_to_js(i);
                }

                self.buf.push_str(") ");
                // TODO: Wrap in {} if body is not a Block
                self.to_js(body, PREC_MAX, false);
                //self.buf.push_str("\n");
                NeedSemi::Yes
            },
            JsAst::Return { value } => {
                self.buf.push_str("return ");
                self.to_js(value, PREC_MAX, true);
                NeedSemi::Yes
            },
            &JsAst::ModuleRef { abs_index } => {
                self.buf.push_str("/*not found*/ ");
                self.push_module_name(abs_index);
                NeedSemi::Yes
            },
            &JsAst::ModuleMember { abs_index, local_index } => {
                let mi = &self.module_infos[abs_index as usize];

                if mi.language == Language::Glsl {
                    let hyp::LocalDef { ty, name, .. } = &mi.locals[local_index as usize];
                    
                    if ty.is_fn() {
                        // TODO: Look up aux data for the local to find index into stringified source
                        //self.buf.push_str("glsl func ref");
                        if let Some(&array_index) = self.glsl_fn_map.get(&(abs_index, local_index)) {
                            self.buf.push_str(&mi.name);
                            if array_index != std::usize::MAX {
                                self.buf.push_str("[");
                                self.buf.push_str(&format!("{}", array_index));
                                self.buf.push_str("]");
                            }
                        } else {
                            self.buf.push_str("<invalid glsl function ref>");
                        }
                    } else {
                        // TODO: This is temporary until we separate buffer writer like in glsl
                        let name_clone = name.clone();
                        self.str_lit(&name_clone);
                    }

                } else {
                    let name = &mi.locals[local_index as usize].name;
                    self.buf.push_str(&name);
                }
                /*
                let export_index = module_info
                    .export_names
                    .iter()
                    .position(|x| x == member);
                
                match export_index {
                    Some(i) =>
                        self.buf.push_str(&module_info.mapped_names[i]),
                    None => {

                        let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                        self.buf.push_str("/*not found*/ ");
                        self.push_module_name(abs_index);
                        self.buf.push_str(".");
                        self.buf.push_str(member);
                        self.unwrap_p(p);
                    }
                }*/
                
                NeedSemi::Yes
            }
            
            JsAst::Local { index } => {
                let name = &self.module_infos[self.current_module].locals[*index as usize].name;
                self.buf.push_str(&name);
                NeedSemi::Yes
            }
            JsAst::Undefined => {
                let p = self.wrap_p(slot, PREC_UNARY_PLUS_MINUS);
                self.buf.push_str("void 0");
                self.unwrap_p(p);
                NeedSemi::Yes
            }
        }
    }
}
