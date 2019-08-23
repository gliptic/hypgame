use serde::{Serialize, Deserialize};
use syn::{Item, Visibility, Expr, Stmt};
use std::collections::{HashMap, hash_map};
use crate::js_ast::{JsLocal, JsLit, JsBuiltin, JsOp, JsUnop, JsAst, JsModule};

pub struct JsEnc {
    pub locals: HashMap<String, JsLocal>,
    pub local_list: Vec<String>,
    pub module: JsModule,
    pub exported_decls: Vec<(String, String, syn::ItemFn)>,
    pub arr: Vec<u8>
}


impl JsEnc {
    pub fn new(name: String) -> JsEnc {
        let mut enc = JsEnc {
            locals: HashMap::new(),
            local_list: Vec::new(),
            exported_decls: Vec::new(),
            module: JsModule {
                name,
                import_map: Vec::new(),
                imports: Vec::new(),
                exports: Vec::new(),
                items: Vec::new()
            },
            arr: Vec::new()
        };

        enc.add_local("glsl", JsLocal::Builtin(JsBuiltin::Glsl));
        enc.add_local("wasm", JsLocal::Builtin(JsBuiltin::Wasm));

        enc
    }

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

    pub fn map_unop(&self, op: &syn::UnOp) -> JsUnop {
        match op {
            syn::UnOp::Neg(_) => JsUnop::Minus,
            syn::UnOp::Not(_) => JsUnop::Not,
            _ => panic!("unimplemented unary op {:?}", op)
        }
    }

    pub fn map_binop(&self, op: &syn::BinOp) -> JsOp {
        match op {
            syn::BinOp::Mul(_) => JsOp::Mul,
            syn::BinOp::Div(_) => JsOp::Div,
            syn::BinOp::Add(_) => JsOp::Add,
            syn::BinOp::Sub(_) => JsOp::Sub,
            syn::BinOp::Rem(_) => JsOp::Rem,
            syn::BinOp::MulEq(_) => JsOp::MulEq,
            syn::BinOp::DivEq(_) => JsOp::DivEq,
            syn::BinOp::AddEq(_) => JsOp::AddEq,
            syn::BinOp::SubEq(_) => JsOp::SubEq,
            syn::BinOp::RemEq(_) => JsOp::RemEq,
            syn::BinOp::BitOr(_) => JsOp::BitOr,
            syn::BinOp::BitAnd(_) => JsOp::BitAnd,
            syn::BinOp::BitXor(_) => JsOp::BitXor,
            syn::BinOp::And(_) => JsOp::AndAnd,
            syn::BinOp::Or(_) => JsOp::OrOr,
            syn::BinOp::Shl(_) => JsOp::Shl,
            syn::BinOp::Shr(_) => JsOp::Shr,
            syn::BinOp::Eq(_) => JsOp::Eq,
            syn::BinOp::Lt(_) => JsOp::Lt,
            syn::BinOp::Le(_) => JsOp::Le,
            syn::BinOp::Ne(_) => JsOp::Ne,
            syn::BinOp::Gt(_) => JsOp::Gt,
            syn::BinOp::Ge(_) => JsOp::Ge,
            _ => panic!("unimplemented binary op {:?}", op)
        }
    }

    pub fn begin_scope(&mut self) -> usize {
        self.local_list.len()
    }

    pub fn end_scope(&mut self, local_count: usize) {
        while self.local_list.len() > local_count {
            let name = self.local_list.pop().unwrap();
            self.locals.remove(&name);
        }
    }

    pub fn check_assignable(&self, expr: &JsAst) {
        match expr {
            JsAst::Global { constant: true, .. } =>
                panic!("cannot assign to global"),
            _ => {}
        }
    }

    pub fn parse_expr(&mut self, expr: &syn::Expr) -> JsAst {
        
        match expr {
            Expr::Lit(syn::ExprLit { lit, .. }) =>
                match lit {
                    syn::Lit::Int(lit_int) => JsAst::Lit { lit: JsLit::Int(lit_int.base10_parse().unwrap()) },
                    syn::Lit::Str(lit_str) => JsAst::Lit { lit: JsLit::Str(lit_str.value()) },
                    syn::Lit::Float(lit_float) => JsAst::Lit { lit: JsLit::Float(lit_float.base10_parse().unwrap()) },
                    syn::Lit::Bool(lit_bool) => JsAst::Lit { lit: JsLit::Bool(lit_bool.value) },
                    _ => panic!("unimplemented literal {:?}", lit)
                },
            Expr::Paren(syn::ExprParen { expr, .. }) =>
                self.parse_expr(expr),
            Expr::Unary(syn::ExprUnary { expr, op, .. }) =>
                JsAst::Unary {
                    value: Box::new(self.parse_expr(expr)),
                    op: self.map_unop(op),
                },
            Expr::Binary(syn::ExprBinary { left, op, right, .. }) =>
                JsAst::Binary {
                    left: Box::new(self.parse_expr(left)),
                    op: self.map_binop(op),
                    right: Box::new(self.parse_expr(right)),
                },
            Expr::Loop(syn::ExprLoop { body, .. }) => {
                let body_ast = body.stmts.iter().map(|s| self.parse_stmt(s)).collect();
                JsAst::Loop {
                    body: body_ast
                }
            },
            Expr::If(syn::ExprIf { cond, then_branch, else_branch, .. }) => {

                let then_ast = then_branch.stmts.iter().map(|s| self.parse_stmt(s)).collect();

                JsAst::If {
                    cond: Box::new(self.parse_expr(cond)),
                    then_branch: then_ast,
                    else_branch: else_branch.as_ref().map(|(_, x)| Box::new(self.parse_expr(x)))
                }
            },
            Expr::While(syn::ExprWhile { cond, body, .. }) => {
                let body_ast = body.stmts.iter().map(|s| self.parse_stmt(s)).collect();
                JsAst::While {
                    cond: Box::new(self.parse_expr(cond)),
                    body: body_ast
                }
            },
            Expr::Array(syn::ExprArray { elems, .. }) => {
                JsAst::Array {
                    elems: elems.iter().map(|e| self.parse_expr(e)).collect()
                }
            },
            Expr::Path(syn::ExprPath { path, .. }) => {
                    let mut segments: Vec<_> = path.segments.iter().map(|x| x.ident.to_string()).collect();
                    if segments.len() == 1 {
                        match self.find_local(&segments[0]) {
                            Some(local) => self.local_to_ast(local),
                            None => JsAst::Path { path: segments }
                        }
                    } else if segments.len() == 2 {
                        match self.find_local(&segments[0]) {
                            Some(local) => {
                                match local {
                                    JsLocal::ModuleRef(module) =>
                                        JsAst::ModuleMember { module, member: segments.remove(1) },
                                    _ => JsAst::Path { path: segments }
                                }
                            },
                            None => JsAst::Path { path: segments }
                        }
                    } else {
                        JsAst::Path { path: segments }
                    }
                },
            Expr::Call(syn::ExprCall { func, args, .. }) =>
                JsAst::Call {
                    func: Box::new(self.parse_expr(func)),
                    args: args.iter().map(|x| self.parse_expr(x)).collect()
                },
            Expr::Assign(syn::ExprAssign { left, right, .. }) => {
                let left_ast = self.parse_expr(left);

                self.check_assignable(&left_ast);
                
                JsAst::Assign {
                    left: Box::new(left_ast),
                    right: Box::new(self.parse_expr(right))
                }
            },
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
            Expr::Index(syn::ExprIndex { expr, index, .. }) => {
                let expr_ast = self.parse_expr(expr);
                let index_ast = self.parse_expr(index);
                JsAst::Index { 
                    expr: Box::new(expr_ast),
                    index: Box::new(index_ast)
                }
            },
            Expr::Field(syn::ExprField { base, member, .. }) => {
                let base_ast = self.parse_expr(base);
                let member_str = match member {
                    syn::Member::Named(id) => id.to_string(),
                    syn::Member::Unnamed(id) => id.index.to_string(),
                };

                match base_ast {
                    JsAst::ModuleRef { module } => JsAst::ModuleMember { module, member: member_str },
                    b => JsAst::Field {
                        base: Box::new(b),
                        member: member_str
                    }
                }
            },
            Expr::Block(syn::ExprBlock { block, .. }) => {
                let local_count = self.begin_scope();

                let b = JsAst::Block {
                    stmts: block.stmts.iter().map(|s| self.parse_stmt(s)).collect()
                };

                self.end_scope(local_count);

                b
            },
            Expr::Closure(syn::ExprClosure { inputs, body, .. }) =>
            {
                let args: Vec<String> = inputs.iter().map(|i| {
                    match i {
                        //syn::FnArg::Inferred(syn::Pat::Ident(pat_ident)) => pat_ident.ident.to_string(),
                        //syn::FnArg::Typed(syn::PatType { pat: syn::Pat::Ident(pat_ident), .. }) =>
                        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
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
            },
            Expr::Return(syn::ExprReturn { expr, .. }) =>
                JsAst::Return {
                    value: Box::new(match expr {
                        Some(e) => self.parse_expr(e),
                        None => JsAst::Undefined
                    })
                },
            _ => panic!("unimplemented expr {:?}", expr)
        }
    }

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

    pub fn parse_stmt(&mut self, stmt: &syn::Stmt) -> JsAst {
        match stmt {
            // TODO: Treat this as return when in correct context
            Stmt::Expr(e) => self.parse_expr(e),
            Stmt::Local(syn::Local { pat, init, .. }) => {
                let names = self.parse_var_pattern(pat);
                for name in &names {
                    self.add_local(name, JsLocal::Local(name.clone()));
                }

                let values = match init {
                    Some((_, e)) => self.parse_var_values(e),
                    None => Vec::new()
                };
                JsAst::Locals { names, values }
            },
            Stmt::Item(item) => self.parse_item(item),
            Stmt::Semi(e, _) => self.parse_expr(e),
            //_ => panic!("unimplemented stmt {:?}", stmt)
        }
    }

    pub fn parse_block(&mut self, block: &syn::Block) -> JsAst {
        JsAst::Block {
            stmts: block.stmts.iter().map(|s| self.parse_stmt(s)).collect()
        }
    }

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

    pub fn parse_rs(&mut self, input: &Vec<syn::Stmt>) {

        for stmt in input {
            let ast = self.parse_stmt(stmt);
            self.module.items.push(ast);
        }

        bincode::serialize_into(&mut self.arr, &self.module).unwrap();
    }
}