use serde::{Serialize, Deserialize};
use crate::hyp_parser as hyp;
//use std::collections::{HashMap, hash_map};

//type TypedAst = (GlslAst, GlslType);
type Id = u32;

#[derive(Clone, Debug)]
pub enum GlslLocal {
    //ModuleRef(u32),
    //ModuleMember(u32, String),
    //SelfMember(u32),
    Local(u32)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GlslLit {
    Int(u64),
    //Str(String),
    Float(f64)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GlslAst {
    Undef,
    Fn { id: Id, exported: bool, arg_names: Vec<String>, expr: Box<GlslAst> },
    Lit { lit: GlslLit },
    Block { stmts: Vec<GlslAst> },
    Locals { locs: Vec<(String, GlslAst, Id)> },
    LocalRef { id: Id },
    Path { segments: Vec<String> },
    Assign { left: Box<GlslAst>, right: Box<GlslAst> },
    Field { base: Box<GlslAst>, member: String },
    Return { value: Box<GlslAst> },
    Unary { value: Box<GlslAst>, op: GlslUnop },
    Binary { left: Box<GlslAst>, op: GlslOp, right: Box<GlslAst> },
    Call { func: Box<GlslAst>, args: Vec<GlslAst> }, // TODO: Record overload?
    If { cond: Box<GlslAst>, then_branch: Vec<GlslAst>, else_branch: Option<Box<GlslAst>> },
    While { cond: Box<GlslAst>, body: Vec<GlslAst> },
    Loop { body: Vec<GlslAst> },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum GlslType {
    Unknown,
    Void,
    Int,
    Float,
    Vec(u32),
    Mat(u32, u32),
    Sampler2D,
    Fn { ret: Box<GlslType>, args: Vec<GlslType> }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum GlslOp {
    Mul,
    Add,
    Sub,
    Div,
    Rem,
    MulEq,
    AddEq,
    SubEq,
    DivEq,
    RemEq,
    //BitOr,
    //BitAnd,
    //BitXor,
    AndAnd,
    OrOr,
    //Shl,
    //Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum GlslUnop {
    Plus,
    Minus,
    Not,
    PreInc,
    PreDec,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GlslGlobal {
    Varying(u32),
    Uniform(u32),
    Attribute(u32)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlslModule {
    pub items: Vec<GlslAst>,
    pub locals: Vec<(GlslType, String)>,
    pub exports: Vec<u32>, // Referenced to locals
    //pub globals: Vec<GlslGlobal>
    pub attributes: Vec<Id>,
    pub varyings: Vec<Id>,
    pub uniforms: Vec<Id>,
}

pub struct GlslEnc {
    pub module: GlslModule
}

impl GlslEnc {
    pub fn new() -> GlslEnc {
        GlslEnc {
            module: GlslModule {
                items: Vec::new(),
                locals: Vec::new(),
                exports: Vec::new(),
                varyings: Vec::new(),
                attributes: Vec::new(),
                uniforms: Vec::new(),
            }
        }
    }

    pub fn map_unop(&self, op: &hyp::Ident) -> GlslUnop {
        match &op[..] {
            "-" => GlslUnop::Minus,
            "!" => GlslUnop::Not,
            _ => panic!("unimplemented unary op {:?}", op)
        }
    }

    pub fn map_binop(&self, op: &hyp::Ident) -> GlslOp {
        match &op[..] {
            "*" => GlslOp::Mul,
            "/" => GlslOp::Div,
            "+" => GlslOp::Add,
            "-" => GlslOp::Sub,
            "%" => GlslOp::Rem,
            "*=" => GlslOp::MulEq,
            "/=" => GlslOp::DivEq,
            "+=" => GlslOp::AddEq,
            "-=" => GlslOp::SubEq,
            "%=" => GlslOp::RemEq,
            //syn::BinOp::BitOr(_) => GlslOp::BitOr,
            //syn::BinOp::BitAnd(_) => GlslOp::BitAnd,
            //syn::BinOp::BitXor(_) => GlslOp::BitXor,
            "&&" => GlslOp::AndAnd,
            "||" => GlslOp::OrOr,
            //syn::BinOp::Shl(_) => GlslOp::Shl,
            //syn::BinOp::Shr(_) => GlslOp::Shr,
            "==" => GlslOp::Eq,
            "<"  => GlslOp::Lt,
            "<=" => GlslOp::Le,
            "!=" => GlslOp::Ne,
            ">"  => GlslOp::Gt,
            ">=" => GlslOp::Ge,
            _ => panic!("unimplemented binary op {:?}", op)
        }
    }

    fn get_return_type(&self, ty: &GlslType) -> GlslType {
        match ty {
            GlslType::Fn { ret, .. } => ret.as_ref().clone(),
            GlslType::Unknown => GlslType::Unknown, // Could be anything
            _ => panic!("non-function type does not have a return type")
        }
    }

    pub fn check_assignable(&self, _expr: &GlslAst) {
        /* TODO: With types too
        match expr {
            GlslAst::Global { constant: true, .. } =>
                panic!("cannot assign to global"),
            _ => {}
        }*/
    }

/*
    fn parse_type(&self, ty: &hyp::AstType) -> GlslType {
        match ty {
            hyp::AstType::F32 => GlslType::Float,
            hyp::AstType::I32 => GlslType::Int,
            hyp::AstType::Vec(_subty, dim) => GlslType::Vec(dim), // TODO: ivec
            hyp::AstType::Mat(_subty, dimx, dimy) => GlslType::Mat(dimx, dimy), // TODO: imat
            hyp::AstType::Other(name) if &name[..] == b"sampler2D" =>
                GlslType::Sampler2D,
            hyp::AstType::Any => GlslType::Unknown,
            _ => panic!("unimplemented type {:?}", &ty)
        }
    }
*/

    pub fn ident_to_str(&self, ident: &hyp::Ident) -> String {
        ident.clone()
    }

    fn parse_item(&mut self, item: &hyp::Ast) -> Option<GlslAst> {
        match &item.data {
            hyp::AstData::LetLocal { name, local_index, attr, .. } => {
                let global_list;
                match attr {
                    hyp::Attr::Varying =>
                        global_list = &mut self.module.varyings,
                    hyp::Attr::Attribute =>
                        global_list = &mut self.module.attributes,
                    hyp::Attr::Uniform =>
                        global_list = &mut self.module.uniforms,
                    _ => panic!("unknown global attribute {:?}", &attr)
                }
            
                global_list.push(*local_index);

                None
            }
            //syn::Item::Fn(syn::ItemFn { vis, sig, block, .. }) => {
            hyp::AstData::FnLocal { lambda, local_index, exported, .. } => {
                //let name = self.ident_to_str(name);
                //let fn_type = self.parse_type_from_sig(sig);
                //let id = self.new_local(name.clone(), fn_type);
                //self.add_local(name, GlslLocal::Local(id));
                //let index = self.module.exports.push(id);

                let arg_names: Vec<String> = lambda.params.iter().map(|p| {
                    /*
                    match &p.pat {
                        hyp::Pattern::Ident(name) => self.ident_to_str(&p.name),
                        _ => panic!("pattern not allowed for glsl")
                    }*/
                    self.ident_to_str(&p.name)
                }).collect();

                let expr = GlslAst::Block {
                    stmts: lambda.expr.iter().map(|s| self.parse_stmt(s)).flatten().collect()
                };

                Some(GlslAst::Fn { id: *local_index, exported: *exported, arg_names, expr: Box::new(expr) })
            }
            _ => {
                None
            }
        }
    }

    pub fn assign_unify(&self, to: GlslType, from: GlslType) -> GlslType {
        if to != GlslType::Unknown {
            to
        } else {
            from
        }
    }

/*
    pub fn parse_var_pattern(&mut self, pat: &syn::Pat) -> String {
        match pat {
            syn::Pat::Ident(pat_ident) => {
                pat_ident.ident.to_string()
            },
            _ => panic!("unimplemented var pattern {:?}", pat)
        }
    }
*/
    pub fn parse_stmt(&mut self, stmt: &hyp::Ast) -> Option<GlslAst> {
        match &stmt.data {
            // TODO: Treat this as return when in correct context
            //Stmt::Expr(e) => Some(self.parse_expr(e)),
            hyp::AstData::FnLocal { .. } => panic!("function not allowed here"),
            hyp::AstData::LetLocal { name, ty, init, local_index, attr: _attr } => {
                let name = self.ident_to_str(name);
                //let decl_ty = self.parse_type(t);

                let value = init.as_ref().map(|e| self.parse_expr(e)).unwrap_or(GlslAst::Undef);

                if ty.is_any() {
                    panic!("type of local {} could not be inferred", &name);
                }
                // TODO: Verify ty is not GlslType::Unknown

                let mut locs = Vec::new();
                locs.push((name, value, *local_index)); // TODO: Do we need name here?
                
                Some(GlslAst::Locals { locs })
            }
            _ => Some(self.parse_expr(stmt))
        }
    }

    pub fn parse_expr(&mut self, expr: &hyp::Ast) -> GlslAst {
        
        match &expr.data {
            hyp::AstData::ConstNum { v } => GlslAst::Lit { lit: GlslLit::Int(*v) },
            hyp::AstData::ConstFloat { v } => GlslAst::Lit { lit: GlslLit::Float(*v) },
            hyp::AstData::App {
                fun: box hyp::Ast {
                    data: hyp::AstData::Ident { s }, ..
                }, params, kind: hyp::AppKind::Unary } => {

                GlslAst::Unary {
                    value: Box::new(self.parse_expr(&params[0])),
                    op: self.map_unop(s),
                }
            }
            hyp::AstData::App {
                fun: box hyp::Ast {
                    data: hyp::AstData::Ident { s }, ..
                }, params, kind: hyp::AppKind::Binary } => {

                if &s[..] == ":=" {
                    let left_ast = self.parse_expr(&params[0]);

                    self.check_assignable(&left_ast);
                    
                    GlslAst::Assign {
                        left: Box::new(left_ast),
                        right: Box::new(self.parse_expr(&params[1]))
                    }
                } else {
                    GlslAst::Binary {
                        left: Box::new(self.parse_expr(&params[0])),
                        op: self.map_binop(s),
                        right: Box::new(self.parse_expr(&params[1])),
                    }
                }
            }
            hyp::AstData::Loop { body } => {
                let body_ast = body.expr.iter().map(|s| self.parse_stmt(s)).flatten().collect();
                
                GlslAst::Loop {
                    body: body_ast
                }
            }
            hyp::AstData::If { cond, body, else_body } => {

                let cond_ast = Box::new(self.parse_expr(cond));
                let then_ast = body.expr.iter().map(|s| self.parse_stmt(s)).flatten().collect();

                GlslAst::If {
                    cond: cond_ast,
                    then_branch: then_ast,
                    else_branch: else_body.as_ref().map(|x| Box::new(self.parse_expr(x)))
                }
            }
            hyp::AstData::While { cond, body } => {
                let cond_ast = Box::new(self.parse_expr(cond));
                let body_ast = body.expr.iter().map(|s| self.parse_stmt(s)).flatten().collect();

                GlslAst::While {
                    cond: cond_ast,
                    body: body_ast
                }
            }
            /* TODO?
            Expr::Array(syn::ExprArray { elems, .. }) => {
                JsAst::Array {
                    elems: elems.iter().map(|e| self.parse_expr(e)).collect()
                }
            }
            */
            hyp::AstData::Ident { s } =>
                GlslAst::Path { segments: vec![self.ident_to_str(s)] },
            hyp::AstData::App { fun, params, kind: hyp::AppKind::Normal } =>
                GlslAst::Call {
                    func: Box::new(self.parse_expr(fun)),
                    args: params.iter().map(|x| self.parse_expr(x)).collect()
                },
            /*
            Expr::AssignOp(syn::ExprAssignOp { left, op, right, .. }) => {

                let op_ast = self.map_binop(op);
                let (left_ast, left_ty) = self.parse_expr(left);
                let (right_ast, right_ty) = self.parse_expr(right);

                // TODO: Check types
                self.check_assignable(&left_ast);

                let ast = match (op_ast, right_ast) {
                    (GlslOp::AddEq, GlslAst::Lit { lit: GlslLit::Int(1) }) =>
                        GlslAst::Unary {
                            value: Box::new(left_ast),
                            op: GlslUnop::PreInc
                        },
                    (GlslOp::SubEq, GlslAst::Lit { lit: GlslLit::Int(1) }) =>
                        GlslAst::Unary {
                            value: Box::new(left_ast),
                            op: GlslUnop::PreDec
                        },
                    (_, right_ast) => 
                        GlslAst::Binary {
                            left: Box::new(left_ast),
                            op: op_ast,
                            right: Box::new(right_ast)
                        },
                };

                (ast, left_ty)
            },
            */
            /* TODO
            Expr::MethodCall(syn::ExprMethodCall { receiver, method, args, .. }) => {
                let receiver_ast = self.parse_expr(receiver);
                let args_ast = args.iter().map(|a| self.parse_expr(a)).collect();
                let method_str = method.to_string();

                match receiver_ast {
                    JsAst::ModuleRef { module } =>
                        JsAst::Call {
                            func: Box::new(JsAst::ModuleMember { module, member: method_str }),
                            args: args_ast
                        },
                    r => JsAst::MethodCall {
                        receiver: Box::new(r),
                        method: method.to_string(),
                        args: args_ast
                    }
                }
            },
            */
            /* TODO
            Expr::Index(syn::ExprIndex { expr, index, .. }) => {
                let expr_ast = self.parse_expr(expr);
                let index_ast = self.parse_expr(index);
                GlslAst::Index { 
                    expr: Box::new(expr_ast),
                    index: Box::new(index_ast)
                }
            },
            */
            hyp::AstData::Field { base, member } => {
                let base_ast = self.parse_expr(base);
                
                match &member.data {
                    hyp::AstData::Ident { s } => {
                        let member_str = self.ident_to_str(s);
                        GlslAst::Field {
                            base: Box::new(base_ast),
                            member: member_str
                        }
                    }
                    _ => panic!("invalid field access {:?}", &member.data)
                }
            }
            hyp::AstData::Block { expr } => {

                let b = GlslAst::Block {
                    stmts: expr.iter().map(|s| self.parse_stmt(s)).flatten().collect()
                };

                b
            }
            /* TODO? What to use it for?
            Expr::Closure(syn::ExprClosure { inputs, body, .. }) =>
            {
                let args: Vec<String> = inputs.iter().map(|i| {
                    match i {
                        syn::FnArg::Inferred(syn::Pat::Ident(pat_ident)) => pat_ident.ident.to_string(),
                        _ => panic!("invalid pattern in lambda, {:?}", i)
                    }
                }).collect();

                let local_count = self.begin_scope();

                for arg in &args {
                    self.add_local(arg, JsLocal::Local(arg.clone()));
                }

                let body_ast = self.parse_expr(body);

                self.end_scope(local_count);

                JsAst::Lambda {
                    inputs: args,
                    body: Box::new(body_ast)
                }
            }
            */
            hyp::AstData::Return { value } =>
                GlslAst::Return {
                    value: Box::new(match value {
                        Some(e) => self.parse_expr(e),
                        None => GlslAst::Undef
                    })
                },
            hyp::AstData::Local { local } => {
                match *local {
                    hyp::Local::Builtin { ref name, .. } =>
                        GlslAst::Path { segments: vec![name.clone()] },
                    hyp::Local::Local { index } =>
                        GlslAst::LocalRef { id: index },
                    _ => panic!("unimplemented local {:?}", local)
                }
            }
            _ => panic!("unimplemented expr {:?}", expr)
        }
    }

    pub fn parse_hyp(&mut self, input: &Vec<hyp::Ast>) {
        for item in input {
            if let Some(ast) = self.parse_item(item) {
                self.module.items.push(ast);
            }
        }
    }
}