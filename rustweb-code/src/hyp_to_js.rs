//use serde::{Serialize, Deserialize};
//use std::collections::{HashMap, hash_map};
use crate::js_ast::{JsLit, JsOp, JsUnop, JsAst, JsModule};
use crate::hyp_parser::{self as hyp};

pub struct JsEnc {
    //pub locals: HashMap<String, JsLocal>,
    //pub local_list: Vec<String>,
    pub module: JsModule,
    //pub exported_decls: Vec<(String, String, syn::ItemFn)>,
}

impl JsEnc {
    pub fn new() -> JsEnc {
        let mut enc = JsEnc {
            //locals: HashMap::new(),
            //local_list: Vec::new(),
            //exported_decls: Vec::new(),
            module: JsModule {
                //name,
                items: Vec::new()
            }
        };

        //enc.add_local("glsl", JsLocal::Builtin(JsBuiltin::Glsl));
        //enc.add_local("wasm", JsLocal::Builtin(JsBuiltin::Wasm));

        enc
    }

/*
    pub fn local_to_ast(&self, local: JsLocal) -> JsAst {
        match local {
            //JsLocal::ModuleRef(module) => JsAst::Path { path: vec![self.module.imports[module as usize].clone()] },
            JsLocal::ModuleRef(module) => JsAst::ModuleRef { module },
            JsLocal::ModuleMember(module, member) => JsAst::ModuleMember { module, member },
            JsLocal::SelfMember(index) => JsAst::SelfMember { index },
            JsLocal::Builtin(builtin) => JsAst::Builtin { builtin },
            JsLocal::Local(name) => JsAst::Path { path: vec![name] },
        }
    }
*/

    pub fn map_unop(&self, op: &hyp::Ident) -> JsUnop {
        match &op[..] {
            "-" => JsUnop::Minus,
            "!" => JsUnop::Not,
            _ => panic!("unimplemented unary op {}", &self.ident_to_str(op))
        }
    }

    pub fn map_binop(&self, op: &hyp::Ident) -> JsOp {
        match &op[..] {
            "*" => JsOp::Mul,
            "/" => JsOp::Div,
            "+" => JsOp::Add,
            "-" => JsOp::Sub,
            "%" => JsOp::Rem,
            "*=" => JsOp::MulEq,
            "/=" => JsOp::DivEq,
            "+=" => JsOp::AddEq,
            "-=" => JsOp::SubEq,
            "%=" => JsOp::RemEq,
            "|" => JsOp::BitOr,
            "&" => JsOp::BitAnd,
            "^" => JsOp::BitXor,
            "&&" => JsOp::AndAnd,
            "||" => JsOp::OrOr,
            "<<" => JsOp::Shl,
            ">>" => JsOp::Shr,
            "==" => JsOp::Eq,
            "<" => JsOp::Lt,
            "<=" => JsOp::Le,
            "!=" => JsOp::Ne,
            ">" => JsOp::Gt,
            ">=" => JsOp::Ge,
            _ => panic!("unimplemented binary op {}", &self.ident_to_str(op))
        }
    }

/*
    pub fn begin_scope(&mut self) -> usize {
        self.local_list.len()
    }

    pub fn end_scope(&mut self, local_count: usize) {
        while self.local_list.len() > local_count {
            let name = self.local_list.pop().unwrap();
            self.locals.remove(&name);
        }
    }
*/

    pub fn check_assignable(&self, expr: &JsAst) {
        match expr {
            JsAst::Global { constant: true, .. } =>
                panic!("cannot assign to global"),
            _ => {}
        }
    }

    pub fn parse_expr(&mut self, expr: &hyp::Ast) -> JsAst {
        
        match &expr.data {
            hyp::AstData::ConstNum { v } => JsAst::Lit { lit: JsLit::Int(*v) },
            hyp::AstData::ConstFloat { v } => JsAst::Lit { lit: JsLit::Float(*v) },
            hyp::AstData::ConstStr { v } => JsAst::Lit { lit: JsLit::Str(v.clone()) },
            hyp::AstData::Loop { body } => {
                //let local_count = self.begin_scope();
                let body_ast = body.expr.iter().map(|s| self.parse_stmt(s)).collect();
                //self.end_scope(local_count);
                
                JsAst::Loop {
                    body: body_ast
                }
            }
            hyp::AstData::App {
                fun: box hyp::Ast {
                    data: hyp::AstData::Ident { s }, ..
                }, params, kind: hyp::AppKind::Unary } => {

                JsAst::Unary {
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
                    
                    JsAst::Assign {
                        left: Box::new(left_ast),
                        right: Box::new(self.parse_expr(&params[1]))
                    }
                } else {
                    JsAst::Binary {
                        left: Box::new(self.parse_expr(&params[0])),
                        op: self.map_binop(s),
                        right: Box::new(self.parse_expr(&params[1])),
                    }
                }
            }
            hyp::AstData::App { fun, params, kind: hyp::AppKind::Normal } =>
                JsAst::Call {
                    func: Box::new(self.parse_expr(fun)),
                    args: params.iter().map(|x| self.parse_expr(x)).collect()
                },
            hyp::AstData::Ident { s } =>
                JsAst::Path { path: vec![self.ident_to_str(s)] },
            hyp::AstData::Field { base, member } => {
                let base_ast = self.parse_expr(base);
                
                match &member.data {
                    hyp::AstData::Ident { s } => {
                        let member_str = self.ident_to_str(s);
                        JsAst::Field {
                            base: Box::new(base_ast),
                            member: member_str
                        }
                    }
                    _ => panic!("invalid field access {:?}", &member.data)
                }
            },
            hyp::AstData::Index { base, index } => {
                let base_ast = self.parse_expr(base);
                let index_ast = self.parse_expr(index);
                JsAst::Index { 
                    expr: Box::new(base_ast),
                    index: Box::new(index_ast)
                }
            }
            hyp::AstData::Array { elems } => {
                JsAst::Array {
                    elems: elems.iter().map(|e| self.parse_expr(e)).collect()
                }
            }
            hyp::AstData::Lambda { lambda } => {
                let args: Vec<String> = lambda.params.iter().map(|p| {
                    self.ident_to_str(&p.name)
                }).collect();

                let body_ast = lambda.expr.iter().map(|e| self.parse_stmt(e)).collect();

                JsAst::Lambda {
                    inputs: args,
                    body: Box::new(JsAst::Block { stmts: body_ast })
                }
            }
            hyp::AstData::If { cond, body, else_body } => {

                let cond_ast = Box::new(self.parse_expr(cond));
                let then_ast = body.expr.iter().map(|s| self.parse_stmt(s)).collect();

                JsAst::If {
                    cond: cond_ast,
                    then_branch: then_ast,
                    else_branch: else_body.as_ref().map(|x| Box::new(self.parse_expr(x)))
                }
            }
            hyp::AstData::Block { expr } => {

                let b = JsAst::Block {
                    stmts: expr.iter().map(|s| self.parse_stmt(s)).collect()
                };

                b
            }
            hyp::AstData::While { cond, body } => {
                let cond_ast = Box::new(self.parse_expr(cond));
                let body_ast = body.expr.iter().map(|s| self.parse_stmt(s)).collect();

                JsAst::While {
                    cond: cond_ast,
                    body: body_ast
                }
            }
            hyp::AstData::Return { value } =>
                JsAst::Return {
                    value: Box::new(match value {
                        Some(e) => self.parse_expr(e),
                        None => JsAst::Undefined
                    })
                },
            hyp::AstData::Local { local } => {
                match *local {
                    hyp::Local::ModuleRef { abs_index } =>
                        JsAst::ModuleRef { abs_index },
                    hyp::Local::ModuleMember { abs_index, local_index } =>
                        JsAst::ModuleMember { abs_index, local_index },
                    hyp::Local::Builtin { ref name, .. } =>
                        JsAst::Path { path: vec![name.clone()] },
                    hyp::Local::Local { index } =>
                        JsAst::Local { index }
                }
            }
            /*
            Expr::AssignOp(syn::ExprAssignOp { left, op, right, .. }) => {

                let op_ast = self.map_binop(op);
                let left_ast = self.parse_expr(left);
                let right_ast = self.parse_expr(right);

                self.check_assignable(&left_ast);

                match (op_ast, right_ast) {
                    (JsOp::AddEq, JsAst::Lit { lit: JsLit::Int(1) }) =>
                        JsAst::Unary {
                            value: Box::new(left_ast),
                            op: JsUnop::PreInc
                        },
                    (JsOp::SubEq, JsAst::Lit { lit: JsLit::Int(1) }) =>
                        JsAst::Unary {
                            value: Box::new(left_ast),
                            op: JsUnop::PreDec
                        },
                    (_, right_ast) => 
                        JsAst::Binary {
                            left: Box::new(left_ast),
                            op: op_ast,
                            right: Box::new(right_ast)
                        },
                }
            },
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
            _ => panic!("unimplemented expr {:?}", expr)
        }
    }

/*
    pub fn parse_var_pattern(&mut self, pat: &syn::Pat) -> Vec<String> {
        match pat {
            syn::Pat::Tuple(pat_tuple) => {
                pat_tuple.elems.iter().map(|x| match x {
                    syn::Pat::Ident(pat_ident) => {
                        pat_ident.ident.to_string()
                    },
                    _ => panic!("only idents allowed in tuple pattern, not {:?}", x)
                }).collect()
            },
            syn::Pat::Ident(pat_ident) => {
                vec![pat_ident.ident.to_string()]
            },
            _ => panic!("unimplemented var pattern {:?}", pat)
        }
    }

    pub fn parse_var_values(&mut self, expr: &syn::Expr) -> Vec<JsAst> {
        match expr {
            Expr::Tuple(expr_tuple) => {
                expr_tuple.elems.iter().map(|x| self.parse_expr(x)).collect()
            },
            e => {
                vec![self.parse_expr(e)]
            }
        }
    }
*/

/*
    pub fn find_local(&mut self, name: &str) -> Option<JsLocal> {
        self.locals.get(name).map(|v| v.clone())
    }

    pub fn add_local(&mut self, name: &str, local: JsLocal) {
        // TODO: Ugh, get rid of .to_owned()
        match self.locals.entry(name.to_owned()) {
            hash_map::Entry::Occupied(_) => panic!("it's not allowed to shadow the name {}", &name),
            hash_map::Entry::Vacant(v) => {
                v.insert(local);
            },
        }

        self.local_list.push(name.to_owned());
    }
*/

    pub fn ident_to_str(&self, ident: &hyp::Ident) -> String {
        ident.clone()
    }

    pub fn parse_stmt(&mut self, stmt: &hyp::Ast) -> JsAst {
        match &stmt.data {
            hyp::AstData::Use { name, rel_index } => {
                let name = self.ident_to_str(&name);
                //let abs_index = self.module.import_map[*rel_index as usize];
                
                //self.add_local(&name, JsLocal::ModuleRef(abs_index));
                JsAst::Use { name, rel_index: *rel_index }
            }
            hyp::AstData::FnLocal { name, lambda, local_index, .. } => {
                let name = self.ident_to_str(name);

                //self.add_local(&name, JsLocal::SelfMember(index));

/* TODO
                let exported = match item_fn.vis {
                    Visibility::Public(_) => true,
                    _ => false
                };
                let exported = false;

                if exported {
                    let exported_name = format!("{}_{}", self.module.name, &name);
                    // TODO? self.exported_decls.push((name.clone(), exported_name, item_fn.clone()));
                }

                self.module.exports.push(name);
                */

                let exported = false; // TODO

                let args: Vec<u32> = lambda.params.iter().map(|p| {
                    p.local_index
                }).collect();

                let expr = JsAst::Block {
                    stmts: lambda.expr.iter().map(|s| self.parse_stmt(s)).collect()
                };

                //let expr = self.parse_block(&item_fn.block);

                JsAst::Fn { index: *local_index, exported, args, expr: Box::new(expr) }
            }
            hyp::AstData::LetLocal { name, ty, init, local_index, attr: _attr } => {
                //let name = self.ident_to_str(name);
                
                let values = init.iter().map(|i| self.parse_expr(i)).collect();;
                JsAst::Locals { local_indexes: vec![*local_index], values }
            }
            /*
            Stmt::Expr(e) => self.parse_expr(e),
            
            Stmt::Item(item) => self.parse_item(item),
            Stmt::Semi(e, _) => self.parse_expr(e),
            */
            _ => self.parse_expr(stmt)
            //_ => panic!("unimplemented stmt {:?}", stmt)
        }
    }

    pub fn parse_block(&mut self, block: &hyp::AstLambda) -> JsAst {
        JsAst::Block {
            stmts: block.expr.iter().map(|s| self.parse_stmt(s)).collect()
        }
    }

/*
    pub fn parse_use(&mut self, use_tree: &syn::UseTree) -> JsAst {
        match use_tree {
            syn::UseTree::Name(syn::UseName { ident }) => {
                let name = ident.to_string();
                let index = self.module.imports.len() as u32;
                println!("assigning {} to {}", index, &name);
                self.module.imports.push(name.clone());
                self.add_local(&name, JsLocal::ModuleRef(index));
                JsAst::Use { name }
            },
            _ => panic!("unimplemented use")
        }
    }

    pub fn parse_item(&mut self, item: &syn::Item) -> JsAst {
        match item {
            Item::Fn(item_fn) => {
                let name = item_fn.sig.ident.to_string();
                let index = self.module.exports.len() as u32;

                self.add_local(&name, JsLocal::SelfMember(index));

                let exported = match item_fn.vis {
                    Visibility::Public(_) => true,
                    _ => false
                };

                if exported {
                    let exported_name = format!("{}_{}", self.module.name, &name);
                    self.exported_decls.push((name.clone(), exported_name, item_fn.clone()));
                }

                self.module.exports.push(name);

                let args: Vec<String> = item_fn.sig.inputs.iter().map(|i| {
                    match i {
                        //syn::FnArg::Inferred(syn::Pat::Ident(pat_ident)) =>
                        syn::FnArg::Typed(syn::PatType { pat: box syn::Pat::Ident(pat_ident), .. }) =>
                            pat_ident.ident.to_string(),
                        //syn::FnArg::Captured(syn::ArgCaptured { pat: syn::Pat::Ident(pat_ident), .. }) =>
                        //    pat_ident.ident.to_string(),
                        //syn::FnArg::Ignored(syn::Type::Path(type_path)) => {
                        //    type_path.path.segments.iter().nth(0).unwrap().ident.to_string() },
                        _ => panic!("invalid pattern in function, {:?}", i)
                    }
                }).collect();

                let local_count = self.begin_scope();

                for arg in &args {
                    self.add_local(arg, JsLocal::Local(arg.clone()));
                }

                let expr = JsAst::Block {
                    stmts: item_fn.block.stmts.iter().map(|s| self.parse_stmt(s)).collect()
                };

                self.end_scope(local_count);

                //let expr = self.parse_block(&item_fn.block);

                JsAst::Fn { index, exported, args, expr: Box::new(expr) }
            },
            Item::Const(item_const) => {
                let name = item_const.ident.to_string();
                let index = self.module.exports.len() as u32;

                self.add_local(&name, JsLocal::SelfMember(index));

                let exported = match item_const.vis {
                    Visibility::Public(_) => true,
                    _ => false
                };

                self.module.exports.push(name);

                let value = self.parse_expr(&item_const.expr);

                JsAst::Global { index, constant: true, value: Box::new(value) }
            },
            Item::Static(item_static) => {
                // syn::ItemStatic { vis, ident, expr, .. }
                let name = item_static.ident.to_string();
                let index = self.module.exports.len() as u32;

                self.add_local(&name, JsLocal::SelfMember(index));

                let exported = match item_static.vis {
                    Visibility::Public(_) => true,
                    _ => false
                };

                self.module.exports.push(name);

                let value = self.parse_expr(&item_static.expr);

                JsAst::Global { index, constant: false, value: Box::new(value) }
            },
            Item::Use(item_use) => {
                let use_ = self.parse_use(&item_use.tree);
                use_
            },
            _ => panic!("unimplemented item {:?}", item)
        }
    }
*/

    pub fn parse_hyp(&mut self, input: &Vec<hyp::Ast>) {

        for stmt in input {
            let ast = self.parse_stmt(stmt);
            self.module.items.push(ast);
        }

        //bincode::serialize_into(&mut self.arr, &self.module).unwrap();
    }
}