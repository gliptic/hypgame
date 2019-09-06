use crate::{GlslAst, GlslModule, GlslLit, GlslType, GlslOp, GlslUnop};
use crate::{hyp_parser as hyp, hyp_to_glsl, conflict_tree::ConflictTree};
use std::collections::{HashMap, HashSet};

// TODO: Move this and NeedSemi to common place with JS version
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


#[derive(PartialEq)]
pub enum NeedSemi { No, Yes, Empty }

const PREC_DOT_BRACKET: Prec = Prec(2, -1);
//const PREC_NEW: Prec = Prec(1, 1);
const PREC_CALL: Prec = Prec(2, -1);
const PREC_PRE_INC_DEC: Prec = Prec(3, 0);
const PREC_UNARY_PLUS_MINUS: Prec = Prec(3, 1);
const PREC_MUL_DIV_REM: Prec = Prec(4, -1);
const PREC_PLUS_MINUS: Prec = Prec(5, -1);
//const PREC_SHIFT: Prec = Prec(7, -1);
const PREC_INEQ: Prec = Prec(7, -1);
const PREC_EQ: Prec = Prec(8, -1);
//const PREC_BITAND: Prec = Prec(10, -1);
//const PREC_BITXOR: Prec = Prec(11, -1);
//const PREC_BITOR: Prec = Prec(12, -1);
const PREC_ANDAND: Prec = Prec(12, -1);
// TODO const PREC_XORXOR: Prec = Prec(13, -1);
const PREC_OROR: Prec = Prec(14, -1);
const PREC_SELECT: Prec = Prec(15, -1);
const PREC_ASSIGN: Prec = Prec(16, 1);
const PREC_COMMA: Prec = Prec(17, -1);
const PREC_MAX: Prec = Prec(18, 0);

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Op, // Can't be next to Op
    Anything, // Can be next to anything, like ( ) [ ] , .
    Alphanum, // Can't be next to Alphanum
}

/*
pub struct ModuleInfo {
    pub mapped_names: Vec<String>,
    pub local_types: Vec<GlslType>
}*/

pub struct Detokenizer {
    pub buf: String,
    minify: bool,
    indent_len: u32,
    prev_token_kind: TokenKind,
}

//pub struct GlslCollection(pub GlslBundler, pub Vec<GlslModule>);

pub struct GlslBundler<'a> {

    pub buf: Detokenizer,
    pub current_module: usize,
    pub exported_local: i32,
    pub module_infos: &'a Vec<hyp::ModuleInfo>,
}

pub struct BundledModule {
    pub prefix: String,
    pub exported_functions: Vec<(String, u32)>,
}

pub struct Token<'a>(&'a str, TokenKind);

const LPAREN: Token<'static> = Token("(", TokenKind::Anything);
const RPAREN: Token<'static> = Token(")", TokenKind::Anything);

macro_rules! T {
    (=) => { Token("=", TokenKind::Op) };
    (;) => { Token(";", TokenKind::Anything) };
}

fn short_name(index: usize) -> String {
    if index < 26 {
        return ((97 + index as u8) as char).to_string();
    } else if index < 52 {
        return ((65 + (index as u8 - 26)) as char).to_string();
    } else {
        return short_name(index % 52) + &short_name(index / 52);
    }
}

impl Detokenizer {
    pub fn token<'a>(&mut self, t: Token<'a>) {
        if self.prev_token_kind != TokenKind::Anything
        && self.prev_token_kind == t.1 {
            self.buf.push(' ');
        }

        self.buf.push_str(t.0);
        self.prev_token_kind = t.1;
    }

    pub fn pretty_token<'a>(&mut self, t: Token<'a>) {
        if !self.minify {
            self.token(t);
        }
    }

    pub fn lparen(&mut self) {
        self.token(LPAREN);
    }

    pub fn rparen(&mut self) {
        self.token(RPAREN);
    }
    
    pub fn wrap_p(&mut self, slot: Prec, op: Prec) -> bool {
        let p = op.need_paren(slot);
        if p {
            self.lparen();
        }
        p
    }

    pub fn unwrap_p(&mut self, f: bool) {
        if f {
            self.rparen();
        }
    }

    pub fn indent(&mut self) {
        if !self.minify {
            for _ in 0..self.indent_len {
                self.token(Token("  ", TokenKind::Anything));
            }
        }
    }

    pub fn nl(&mut self) {
        self.pretty_token(Token("\n", TokenKind::Anything));
    }

    pub fn pretty_space(&mut self) {
        self.pretty_token(Token(" ", TokenKind::Anything));
    }

    pub fn comma(&mut self) {
        self.token(Token(",", TokenKind::Anything));
        self.pretty_space();
    }

    pub fn period(&mut self) {
        // TODO: Maybe Anything should only apply if prev token isn't period(s)
        self.token(Token(".", TokenKind::Anything));
    }

    pub fn semi(&mut self) {
        //self.buf.push_str(";");
        self.token(T![;]);
    }

    pub fn assign(&mut self) {
        self.pretty_space();
        self.token(T![=]);
        self.pretty_space();
    }

    pub fn ident(&mut self, i: &str) {
        self.token(Token(i, TokenKind::Alphanum));
    }

    pub fn op(&mut self, i: &str) {
        self.token(Token(i, TokenKind::Op));
    }

    pub fn parse_type(&self, ty: &hyp::AstType) -> GlslType {
        match ty {
            hyp::AstType::F32 => GlslType::Float,
            hyp::AstType::I32 => GlslType::Int,
            hyp::AstType::Vec(_subty, dim) => GlslType::Vec(*dim), // TODO: ivec
            hyp::AstType::Mat(_subty, dimx, dimy) => GlslType::Mat(*dimx, *dimy), // TODO: imat
            hyp::AstType::Other(name) if &name[..] == "sampler2D" =>
                GlslType::Sampler2D,
            hyp::AstType::Any => GlslType::Unknown,
            hyp::AstType::None => GlslType::Void,
            _ => panic!("unimplemented type {:?}", &ty)
        }
    }

    pub fn type_to_glsl(&mut self, ty: &hyp::AstType) {
        let glsl_type = self.parse_type(ty);

        let name: String = match glsl_type {
            GlslType::Unknown => { panic!("type unknown") }
            GlslType::Void => "void".into(),
            GlslType::Int => "int".into(),
            GlslType::Float => "float".into(),
            GlslType::Vec(n) => "vec".to_owned() + &n.to_string(),
            GlslType::Mat(n, m) => if n == m {
                "mat".to_owned() + &n.to_string()
            } else {
                "mat".to_owned() + &n.to_string() + "x" + &m.to_string()
            },
            GlslType::Sampler2D => "sampler2D".into(),
            GlslType::Fn { .. } => { panic!("cannot write function type in code") },
            //_ => { panic!("unimplemented type formatting {:?}", &ty) }
        };

        self.ident(&name);
    }
}

impl<'a> GlslBundler<'a> {
    pub fn new(module_infos: &Vec<hyp::ModuleInfo>, exported_local: i32, debug: bool) -> GlslBundler {
        GlslBundler {
            buf: Detokenizer {
                buf: String::new(),
                minify: !debug,
                indent_len: 0,
                prev_token_kind: TokenKind::Anything,
            },
            module_infos,
            exported_local,
            current_module: 0
        }
    }

    pub fn shorten_names(
        module_infos: &mut Vec<hyp::ModuleInfo>,
        all_used_imports: HashSet<(u32, u32)>) {
        
        let mut whole_ct = ConflictTree::new();

        fn clone_and_annotate(ct: &ConflictTree<u32>, abs_index: u32) -> ConflictTree<(u32, u32)> {
            let mut new_ct = ConflictTree::new();
            new_ct.items = ct.items.iter().map(|&(id, count)| ((abs_index, id), count)).collect();
            new_ct.children = ct.children.iter()
                .map(|ch| clone_and_annotate(ch, abs_index)).collect();
            new_ct
        }

        for abs_index in 0..module_infos.len() {
            let module = &mut module_infos[abs_index];

            if module.language == hyp::Language::Glsl {
                let mut ct = clone_and_annotate(&module.conflict_tree, abs_index as u32);

                // Move varyings (or outputs connected to inputs) to whole_ct
                ct.items.drain_filter(|&mut (id, count)| { // TEMP!
                //module.conflict_tree.items.clone().drain_filter(|&mut (id_, count)| {
                    //let id = (abs_index as u32, id_);
                    if all_used_imports.contains(&id) {
                        let local = &module.locals[id.1 as usize];
                        println!("moving {}.{}", &module.name, &local.1);
                        whole_ct.items.push((id, count));
                        true
                    } else {
                        false
                    }
                });
                whole_ct.add_child(ct); // TEMP
            }
        }

        whole_ct.compute_sum_of_max();

        // Rename
        let mut locals = vec![];
        let mut name_index = 0;
        
        loop {
            let more = whole_ct.assign_best(&mut locals);

            for &(abs_index, local_index) in &locals {
                let new_name = short_name(name_index);
                let module = &mut module_infos[abs_index as usize];
                let local = &mut module.locals[local_index as usize];

                println!("renaming {}.{} to {} ({}.{})", &module.name, &local.1, &new_name, abs_index, local_index);
                local.1 = new_name;
            }
            //dbg!(&whole_ct);
            locals.clear();
            name_index += 1;
            if !more {
                break;
            }
        }
    }

    pub fn bundle(
        module_infos: &mut Vec<hyp::ModuleInfo>,
        module_lambdas: &Vec<hyp::AstLambda>,
        module_order: &[u32],
        debug: bool) -> HashMap<usize, BundledModule> {

        /*
        vertex_glsl {
            varying ...
        }
        fragment_glsl {
        }

        ->

        {
            varying ...
            vertex_glsl {
                varying ...
            }
            fragment_glsl {
            }
        }
        */


        let mut all_used_imports = HashSet::new();
        let mut converted_modules = vec![];
        
        for order_index in 0..module_order.len() {
            let abs_index = module_order[order_index] as usize;
            let module_lambda = &module_lambdas[abs_index];

            //println!("bundling #{}: {}", module_index, &self.module_infos[module_index as usize].name);
            let module = &module_infos[abs_index];

            if module.language == hyp::Language::Glsl {
                println!("Compiling glsl {}", &module.name);
                let mut enc = hyp_to_glsl::GlslEnc::new();
                enc.parse_hyp(&module_lambda.expr);

                for ui in &enc.module.used_imports {
                    all_used_imports.insert(*ui);
                }

                converted_modules.push((abs_index, enc.module));
            }
        }

        Self::shorten_names(module_infos, all_used_imports);

        let mut bundled_modules = HashMap::new();

        for (abs_index, converted_module) in converted_modules {
            
            let module = &module_infos[abs_index];

            let main = {
                    let mut glsl_bundler = GlslBundler::new(module_infos, -1, debug);
                    glsl_bundler.current_module = abs_index;
                    glsl_bundler.module_to_glsl(&converted_module);
                    glsl_bundler.buf.buf
                };

            let mut exported_functions = Vec::new();

            for &exported_index in &module.exports {
                if module.locals[exported_index as usize].0.is_fn() {
                    let mut glsl_bundler = GlslBundler::new(module_infos, exported_index as i32, debug);
                    glsl_bundler.current_module = abs_index;
                    glsl_bundler.module_to_glsl(&converted_module);
                    
                    exported_functions.push((glsl_bundler.buf.buf, exported_index));
                }
            }

            bundled_modules.insert(abs_index, BundledModule {
                prefix: main,
                exported_functions
            });
        }

        bundled_modules
    }

    pub fn stmts_inner_to_glsl(&mut self, stmts: &[GlslAst], slot: Prec, commas: bool) {
        let mut first = true;
        let prec = if stmts.len() == 1 { slot } else { PREC_COMMA };

        for s in stmts {
            if first {
                first = false;
            } else if commas {
                self.buf.comma();
                self.buf.pretty_space();
            }

            if !commas {
                self.buf.indent();
            }

            let need_semi = self.to_glsl(s, prec);
            if !commas {
                match need_semi {
                    NeedSemi::Yes => {
                        self.buf.semi();
                        self.buf.nl();
                    }
                    NeedSemi::No => {
                        self.buf.nl();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn stmts_to_glsl(&mut self, stmts: &[GlslAst], can_skip_braces: bool, slot: Prec) -> NeedSemi {
        if can_skip_braces && stmts.len() > 0
         && stmts.iter().all(|x| x.is_expr()) {

            let p = stmts.len() != 1 && self.buf.wrap_p(slot, PREC_COMMA);
            self.stmts_inner_to_glsl(stmts, slot, true);
            
            self.buf.unwrap_p(p);
            NeedSemi::Yes

        } else {
            self.buf.token(Token("{", TokenKind::Anything));
            self.buf.nl();
            self.buf.indent_len += 1;
            self.stmts_inner_to_glsl(stmts, PREC_MAX, false);
            self.buf.indent_len -= 1;
            self.buf.indent();
            self.buf.token(Token("}", TokenKind::Anything));
            NeedSemi::No
        }
    }

    pub fn ident_to_str(&self, ident: &hyp::Ident) -> String {
        ident.clone()
    }

    pub fn module_to_glsl(&mut self, module: &GlslModule) {

        let mut varyings: Vec<_> = module.varyings.iter().map(|&v| (self.current_module, v)).collect();
        let attributes: Vec<_> = module.attributes.iter().map(|&v| (self.current_module, v)).collect();
        let uniforms: Vec<_> = module.uniforms.iter().map(|&v| (self.current_module, v)).collect();

        // TODO: Distinguish imported varyings from imported uniforms
        for &(module_index, local_index) in &module.used_imports {
            varyings.push((module_index as usize, local_index));
        }

        let globals = vec![
            (&varyings, "varying"),
            (&attributes, "attribute"),
            (&uniforms, "uniform"),
        ];

        if self.exported_local < 0 {
            for (t, kind) in &globals {
                for &(module_index, local_index) in *t {
                    let (ty, name) = &self.module_infos[module_index].locals[local_index as usize];
                    //let name = &self.module_infos[module_index].locals[local_index as usize].1;

                    self.buf.ident(kind);
                    self.buf.type_to_glsl(ty);
                    self.buf.ident(&self.ident_to_str(name));
                    self.buf.semi();
                    self.buf.nl();
                }
            }
        }

        self.stmts_inner_to_glsl(&module.items, PREC_MAX, false);
    }

    pub fn to_glsl(&mut self, ast: &GlslAst, slot: Prec) -> NeedSemi {
        match ast {
            GlslAst::Fn { id, expr, args, exported } => {

                if (*exported && *id as i32 != self.exported_local)
                 || (!*exported && self.exported_local >= 0) {
                    return NeedSemi::Empty;
                }

                let name = &self.module_infos[self.current_module].locals[*id as usize].1;
                
                let fn_ty = &self.module_infos[self.current_module].locals[*id as usize].0;
                let (ret_ty, args_ty) = match &fn_ty {
                    hyp::AstType::Fn(ret, args) => (ret, args),
                    _ => { panic!("function doesn't have function type?") }
                };

                self.buf.type_to_glsl(ret_ty);

                if self.exported_local >= 0 {
                    self.buf.ident("main");
                } else {
                    self.buf.ident(&self.ident_to_str(name));
                }
                
                self.buf.lparen();
                let mut first = true;
                for (local_index, ty) in args.iter().zip(args_ty.iter()) {
                    if first {
                        first = false;
                    } else {
                        self.buf.comma();
                    }
                    self.buf.type_to_glsl(ty);

                    let (_, name) = &self.module_infos[self.current_module].locals[*local_index as usize];
                    self.buf.ident(name);
                }
                self.buf.rparen();
                self.buf.pretty_space();
                self.to_glsl(&expr, PREC_MAX);
                self.buf.nl();
                NeedSemi::No
            },
            GlslAst::Path { segments } => {
                let p = self.buf.wrap_p(slot, PREC_DOT_BRACKET);
                let mut first = true;
                for piece in segments {
                    if first {
                        first = false;
                    } else {
                        self.buf.period();
                    }
                    self.buf.ident(piece);
                }
                self.buf.unwrap_p(p);
                NeedSemi::Yes
            },
            GlslAst::Call { func, args } => {
                self.to_glsl(&func, PREC_CALL);
                self.buf.lparen();
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    } else {
                        self.buf.comma();
                    }
                    self.to_glsl(arg, PREC_COMMA.on_either());
                }
                self.buf.rparen();
                NeedSemi::Yes
            },
            GlslAst::Lit { lit } => {
                match lit {
                    GlslLit::Int(v) => {
                        let txt = format!("{}", v);
                        self.buf.token(Token(&txt, TokenKind::Alphanum));
                    },
                    GlslLit::Float(v) => {
                        let mut txt = format!("{}", v);
                        if !txt.contains('.') {
                            txt.push('.'); // 1 -> 1.
                        } else {
                            let mut bytes = txt.as_bytes();
                            while !bytes.is_empty()
                               && bytes[0] == b'0' {
                                bytes = &bytes[1..];
                            }

                            while bytes.last() == Some(&b'0') {
                                bytes = &bytes[0..bytes.len() - 1];
                            }

                            txt = std::str::from_utf8(bytes).unwrap().into();
                        }
                        self.buf.token(Token(&txt, TokenKind::Alphanum));
                    }
                }
                NeedSemi::Yes
            },
            GlslAst::Block { stmts } => {
                /*
                if stmts.len() != 0 && ast.is_expr() {
                    let prec = if stmts.len() == 1 { slot } else { PREC_COMMA };

                    let p = stmts.len() != 1 && self.buf.wrap_p(slot, PREC_COMMA);
                    let mut first = true;
                    for expr in stmts {
                        if first {
                            first = false;
                        } else {
                            self.buf.comma();
                        }
                        self.to_glsl(expr, prec);
                    }
                    
                    self.buf.unwrap_p(p);
                    NeedSemi::Yes
                } else {
                */
                // TODO: Better test for can_skip_braces
                self.stmts_to_glsl(stmts, slot != PREC_MAX, slot)
                /*}*/
            }/*,
            &GlslAst::Global { index, constant, ref value } => {
                let name = &self.module_infos[self.current_module].mapped_names[index as usize];
                // TODO: Use var instead of const for compactness?
                self.buf.push_str(if constant { "const " } else { "var " });
                self.buf.push_str(name);
                self.buf.push_str(" = ");
                self.to_js(value, PREC_ASSIGN.on_right());
                self.buf.push_str(";");
                NeedSemi::No
            },*/
            GlslAst::Locals { locs } => {
                let mut first = true;
                let mut prev_ty = hyp::AstType::None;

                //self.buf.push_str("var ");
                for i in 0..locs.len() {
                    let id = locs[i].2;
                    let (ty, name) = &self.module_infos[self.current_module].locals[id as usize];

                    if first || ty != &prev_ty {

                        if !first {
                            self.buf.semi();
                            self.buf.nl();
                            self.buf.indent();
                        }

                        first = false;

                        self.buf.type_to_glsl(ty);
                        prev_ty = ty.clone();
                        //self.buf.push_str(", ");
                        //self.buf.comma();
                    } else {
                        self.buf.comma();
                    }
                    
                    //let name = &locs[i].0;
                    self.buf.ident(name);
                    let init = &locs[i].1;
                    if let GlslAst::Undef = init {
                        // Do nothing
                    } else {
                        self.buf.assign();
                        self.to_glsl(&init, PREC_ASSIGN.on_right());
                    }
                }
                self.buf.semi();
                
                NeedSemi::No
            },
            GlslAst::Assign { left, right } => {
                let p = self.buf.wrap_p(slot, PREC_ASSIGN);
                self.to_glsl(left, PREC_ASSIGN.on_left());
                self.buf.assign();
                self.to_glsl(right, PREC_ASSIGN.on_right());
                self.buf.unwrap_p(p);
                NeedSemi::Yes
            }/*
            GlslAst::Array { elems } => {
                let mut first = true;
                self.buf.push('[');
                for e in elems {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.to_js(e, PREC_COMMA.on_either());
                }
                self.buf.push(']');
                NeedSemi::Yes
            },*/
            GlslAst::If { cond, then_branch, else_branch } => {
                if ast.is_expr() {
                    let p = self.buf.wrap_p(slot, PREC_SELECT);
                    self.to_glsl(cond, PREC_SELECT.on_left());
                    self.buf.pretty_space();
                    // TODO: Some of these tolerate anything after
                    self.buf.op("?");
                    self.buf.pretty_space();
                    // TODO: Avoid clone etc.
                    self.to_glsl(&GlslAst::Block { stmts: (*then_branch).clone() }, PREC_SELECT);
                    self.buf.pretty_space();
                    self.buf.op(":");
                    self.buf.pretty_space();
                    if let Some(else_ast) = else_branch {
                        self.to_glsl(else_ast, PREC_SELECT);
                    } else {
                        // TODO: Use some other literal based on what the type on the other side is
                        self.buf.token(Token("0", TokenKind::Alphanum));
                    }
                    self.buf.unwrap_p(p);
                    NeedSemi::Yes
                } else {
                    self.buf.ident("if");
                    self.buf.pretty_space();
                    self.buf.lparen();
                    self.to_glsl(cond, PREC_MAX);
                    self.buf.rparen();
                    self.buf.pretty_space();
                    let mut need_semi = self.stmts_to_glsl(then_branch, true, PREC_MAX);
                    match else_branch {
                        Some(e) => {
                            if need_semi == NeedSemi::Yes {
                                self.buf.semi();
                            }
                            //self.buf.push_str(" else ");
                            self.buf.pretty_space();
                            self.buf.ident("else");
                            self.buf.pretty_space();
                            need_semi = self.to_glsl(e, PREC_MAX);
                        },
                        None => {}
                    }
                    need_semi
                }
            },
            GlslAst::While { cond, body } => {
                self.buf.ident("for");
                self.buf.lparen();
                self.buf.semi();
                self.to_glsl(cond, PREC_MAX);
                self.buf.semi();
                self.buf.rparen();
                self.buf.pretty_space();
                self.stmts_to_glsl(body, true, PREC_MAX)
            },
            GlslAst::Loop { body } => {
                self.buf.ident("for");
                self.buf.lparen();
                self.buf.semi();
                self.buf.semi();
                self.buf.rparen();
                self.buf.pretty_space();
                self.stmts_to_glsl(body, true, PREC_MAX)
            },
            GlslAst::Unary { value, op } => {
                let prec = match op {
                    GlslUnop::Not | GlslUnop::Plus | GlslUnop::Minus => PREC_UNARY_PLUS_MINUS,
                    GlslUnop::PreInc | GlslUnop::PreDec => PREC_PRE_INC_DEC,
                };

                let p = self.buf.wrap_p(slot, prec);
                self.buf.op(match op {
                    GlslUnop::Plus => "+",
                    GlslUnop::Minus => "-",
                    GlslUnop::Not => "!",
                    GlslUnop::PreInc => "++",
                    GlslUnop::PreDec => "--",
                });
                self.to_glsl(value, prec.on_right());

                self.buf.unwrap_p(p);
                NeedSemi::Yes
            },
            GlslAst::Binary { left, op, right } => {
                let prec = match op {
                    GlslOp::Mul | GlslOp::Div | GlslOp::Rem => PREC_MUL_DIV_REM,
                    GlslOp::Add | GlslOp::Sub => PREC_PLUS_MINUS,
                    GlslOp::MulEq | GlslOp::DivEq | GlslOp::RemEq | GlslOp::AddEq | GlslOp::SubEq
                        => PREC_ASSIGN,
                    GlslOp::Eq | GlslOp::Ne
                        => PREC_EQ,
                    GlslOp::Lt | GlslOp::Le | GlslOp::Gt | GlslOp::Ge
                        => PREC_INEQ,
                    //GlslOp::BitAnd => PREC_BITAND,
                    //GlslOp::BitOr => PREC_BITOR,
                    //GlslOp::BitXor => PREC_BITXOR,
                    // TODO: GlslOp::XorXor => PREC_XORXOR,
                    GlslOp::AndAnd => PREC_ANDAND,
                    GlslOp::OrOr => PREC_OROR,
                    //GlslOp::Shl | GlslOp::Shr => PREC_SHIFT,
                };

                let p = self.buf.wrap_p(slot, prec);
                self.to_glsl(left, prec.on_left());
                self.buf.pretty_space();
                // TODO: Some of these tolerate anything after
                self.buf.op(match op {
                    GlslOp::Mul => "*",
                    GlslOp::Div => "/",
                    GlslOp::Rem => "%",
                    GlslOp::Add => "+",
                    GlslOp::Sub => "-",
                    GlslOp::MulEq => "*=",
                    GlslOp::DivEq => "/=",
                    GlslOp::RemEq => "%=",
                    GlslOp::AddEq => "+=",
                    GlslOp::SubEq => "-=",
                    //GlslOp::BitAnd => "&",
                    //GlslOp::BitOr => "|",
                    //GlslOp::BitXor => "^",
                    GlslOp::AndAnd => "&&",
                    GlslOp::OrOr => "||",
                    //GlslOp::Shl => "<<",
                    //GlslOp::Shr => ">>",
                    GlslOp::Eq => "==",
                    GlslOp::Lt => "<",
                    GlslOp::Le => "<=",
                    GlslOp::Ne => "!=",
                    GlslOp::Gt => ">",
                    GlslOp::Ge => ">=",
                });
                self.buf.pretty_space();
                self.to_glsl(right, prec.on_right());
                self.buf.unwrap_p(p);
                NeedSemi::Yes
            },
            GlslAst::Field { base, member } => {
                let p = self.buf.wrap_p(slot, PREC_DOT_BRACKET);
                self.to_glsl(base, PREC_DOT_BRACKET.on_left());
                self.buf.period();
                self.buf.ident(member);
                self.buf.unwrap_p(p);
                NeedSemi::Yes
            }
            &GlslAst::LocalRef { id } => {
                let name = &self.module_infos[self.current_module].locals[id as usize].1;

                self.buf.ident(&self.ident_to_str(name));
                NeedSemi::Yes
            }
            &GlslAst::ModuleMember { abs_index, local_index } => {
                let name = &self.module_infos[abs_index as usize].locals[local_index as usize].1;
                self.buf.ident(&self.ident_to_str(name));
                NeedSemi::Yes
            }
            /*
            GlslAst::Index { expr, index } => {
                let p = self.wrap_p(slot, PREC_DOT_BRACKET);
                self.to_js(expr, PREC_DOT_BRACKET.on_left());
                self.buf.push('[');
                self.to_js(index, PREC_MAX);
                self.buf.push(']');
                self.unwrap_p(p);
                NeedSemi::Yes
            },
            GlslAst::MethodCall { receiver, method, args } => {

                let debug_gl = false;

                if debug_gl {
                    match &**receiver { // TEMP
                        GlslAst::SelfMember { index }
                        if &self.module_infos[self.current_module].mapped_names[*index as usize] == "render_gl"
                        && method != "getError" => {
                            self.buf.push_str("render_checkErr");
                        },
                        _ => {}
                    }

                    self.buf.push('('); // TEMP
                }

                if method == "new" {
                    self.buf.push_str("new ");
                    self.to_js(receiver, PREC_NEW.on_left());
                } else {
                    self.to_js(receiver, PREC_DOT_BRACKET.on_left());
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
                    self.to_js(arg, PREC_COMMA.on_either());
                }
                self.buf.push(')');

                if debug_gl {
                    self.buf.push(')'); // TEMP
                }
                NeedSemi::Yes
            },
            GlslAst::Lambda { inputs, body } => {
                self.buf.push_str("function(");
                let mut first = true;
                for i in inputs {
                    if first {
                        first = false;
                    } else {
                        self.buf.push_str(", ");
                    }
                    self.buf.push_str(i);
                }
                self.buf.push_str(") ");
                // TODO: Wrap in {} if body is not a Block
                self.to_js(body, PREC_MAX);
                //self.buf.push_str("\n");
                NeedSemi::Yes
            },*/
            GlslAst::Return { value } => {
                self.buf.ident("return");
                self.to_glsl(value, PREC_MAX);
                NeedSemi::Yes
            }/*,
            &GlslAst::ModuleRef { module } => {
                let module = self.module_infos[self.current_module].import_map[module as usize];
                self.buf.push_str("/*not found*/ ");
                self.push_module_name(module);
                NeedSemi::Yes
            },
            &GlslAst::ModuleMember { module, ref member } => {
                let module = self.module_infos[self.current_module].import_map[module as usize];
                let module_info = &self.module_infos[module as usize];
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
                        self.push_module_name(module);
                        self.buf.push_str(".");
                        self.buf.push_str(member);
                        self.unwrap_p(p);
                    }
                }
                
                NeedSemi::Yes
            },
            GlslAst::SelfMember { index } => {
                let export_name = &self.module_infos[self.current_module].mapped_names[*index as usize];
                self.buf.push_str(export_name);
                NeedSemi::Yes
            },*/
            GlslAst::Undef => {
                panic!("undefined value cannot be written in code");
                //let p = self.wrap_p(slot, PREC_UNARY_PLUS_MINUS);
                //self.buf.push_str("void 0");
                //self.unwrap_p(p);
                //NeedSemi::Yes
            }
        }
    }
}