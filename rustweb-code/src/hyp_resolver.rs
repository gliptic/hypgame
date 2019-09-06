use crate::hyp_parser::*;
use crate::conflict_tree::ConflictTree;
use std::collections::{HashMap, hash_map};
use std::cell::Cell;

type LocalList = Vec<(AstType, Ident)>;

type LocalConflictTree = ConflictTree<u32>;

pub struct Resolver<'a> {
    pub scope_locals: HashMap<Ident, (Local, Cell<usize>)>,
    pub scope_local_list: Vec<Ident>,
    pub current_module: usize,
    pub module_infos: &'a mut Vec<ModuleInfo>,
    pub debug: bool,
    pub conflict_tree_stack: Vec<LocalConflictTree>,
    pub errors: Vec<ParseError>,
}

impl<'a> Resolver<'a> {
    pub fn new(
        current_module: usize,
        module_infos: &'a mut Vec<ModuleInfo>,
        debug: bool) -> Resolver {

        let mut res = Resolver {
            scope_locals: HashMap::new(),
            scope_local_list: Vec::new(),
            conflict_tree_stack: vec![ConflictTree::new()],
            current_module,
            module_infos,
            debug,
            errors: Vec::new()
        };

        match res.module_infos[current_module].language {
            Language::Js => {},
            Language::Glsl => {
                res.add_local("vec2".to_owned(),
                    Local::Builtin {
                        name: "vec2".to_owned(),
                        ty: AstType::Ctor(Box::new(AstType::Vec(Box::new(AstType::F32), 2)))
                    });
                res.add_local("mat2".to_owned(),
                    Local::Builtin {
                        name: "mat2".to_owned(),
                        ty: AstType::Ctor(Box::new(AstType::Mat(Box::new(AstType::F32), 2, 2)))
                    });
            }
            Language::Binary => {}
        }

        res
    }

    pub fn add_error(&mut self, ast: &Ast, s: &'static str) {
        self.errors.push(ParseError(ast.loc, s));
    }

    pub fn begin_scope(&mut self) -> usize {
        self.conflict_tree_stack.push(ConflictTree::new());
        self.scope_local_list.len()
    }

    pub fn add_local(&mut self, name: Ident, local: Local) {
        
        // TODO: Ugh, get rid of .clone()
        match self.scope_locals.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => panic!("it's not allowed to shadow the name {:?}", &name),
            hash_map::Entry::Vacant(v) => {
                v.insert((local, Cell::new(1)));
            },
        }
        
        self.scope_local_list.push(name);
    }

    pub fn find_local(&self, s: &str) -> Option<&Local> {
        let r = self.scope_locals.get(s);

        if let Some((l, count)) = r {
            count.set(count.get() + 1);
            Some(l)
        } else {
            None
        }
    }

    pub fn end_scope(&mut self, local_count: usize) {

        self.flush_scope_locals(local_count);

        let child = self.conflict_tree_stack.pop().unwrap();
        self.conflict_tree_stack.last_mut().unwrap().add_child(child);
    }

    pub fn flush_scope_locals(&mut self, local_count: usize) {
        while self.scope_local_list.len() > local_count {
            let name = self.scope_local_list.pop().unwrap();
            let local = self.scope_locals.remove(&name);

            if let Some((Local::Local { index, inline }, count)) = local {
                if inline.is_none() {
                    // TODO: Gather use statistics instead of 1
                    self.conflict_tree_stack.last_mut().unwrap().add_item(index, count.get());
                }
            }
        }
    }

    fn resolve_type(&mut self, ty: &mut AstType) {
        let lang = self.module_infos[self.current_module].language;

        match lang {
            Language::Glsl => {
                match ty {
                    AstType::Other(s) =>
                        match &s[..] {
                            "float" => *ty = AstType::F32,
                            "int" => *ty = AstType::I32,
                            "vec2" => *ty = AstType::Vec(Box::new(AstType::F32), 2),
                            "vec3" => *ty = AstType::Vec(Box::new(AstType::F32), 3),
                            "vec4" => *ty = AstType::Vec(Box::new(AstType::F32), 4),
                            "mat2" | "mat2x2" => *ty = AstType::Mat(Box::new(AstType::F32), 2, 2),
                            "mat2x3" => *ty = AstType::Mat(Box::new(AstType::F32), 2, 3),
                            "mat2x4" => *ty = AstType::Mat(Box::new(AstType::F32), 2, 4),
                            "mat3x2" => *ty = AstType::Mat(Box::new(AstType::F32), 3, 2),
                            "mat3" | "mat3x3" => *ty = AstType::Mat(Box::new(AstType::F32), 3, 3),
                            "mat3x4" => *ty = AstType::Mat(Box::new(AstType::F32), 3, 4),
                            "mat4x2" => *ty = AstType::Mat(Box::new(AstType::F32), 4, 2),
                            "mat4x3" => *ty = AstType::Mat(Box::new(AstType::F32), 4, 3),
                            "mat4" | "mat4x4" => *ty = AstType::Mat(Box::new(AstType::F32), 4, 4),
                            //b"sampler2D" => GlslType::Sampler2D,
                            _ => {}
                        },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn resolve(&mut self, root_lambda: &mut AstLambda) {
        self.resolve_lambda(root_lambda);
        self.flush_scope_locals(0);

        let module = &mut self.module_infos[self.current_module];
        module.conflict_tree = self.conflict_tree_stack.remove(0);
        module.conflict_tree.compute_sum_of_max();
    }

    pub fn resolve_lambda(&mut self, lambda: &mut AstLambda) -> AstType {
        for ParamDef { ty, .. } in &mut lambda.params {
            self.resolve_type(ty);
        }

        for &local_index in &lambda.param_locals {
            let name = &self.module_infos[self.current_module].locals[local_index as usize].1;
            let cloned_name = name.clone();
            self.add_local(cloned_name, Local::Local { index: local_index, inline: None });
        }

        self.resolve_type(&mut lambda.return_type);

        let fn_type = AstType::Fn(
            Box::new(lambda.return_type.clone()),
            lambda.params.iter().map(|p| p.ty.clone()).collect());

        lambda.expr.drain_filter(|e| {
            self.resolve_ast(e);
            match e.data {
                AstData::Void => true,
                _ => false
            }
        });

        fn_type
    }

    fn resolve_ast(&mut self, ast: &mut Ast) {
        if ast.attr == Attr::Debug && !self.debug {
            let loc = ast.loc;
            *ast = Ast { loc, attr: Attr::None, ty: AstType::None, data: AstData::Void };
            return;
        }

        match &mut ast.data {
            AstData::Void => {}
            AstData::ConstNum { .. } => {
                if self.module_infos[self.current_module].language == Language::Glsl {
                    // GLSL ints are I32
                    ast.ty = AstType::I32;
                }
            }
            AstData::ConstStr { .. } => { /* Type already set */ }
            AstData::ConstFloat { .. } => {
                if self.module_infos[self.current_module].language == Language::Glsl {
                    // GLSL floats are F32
                    ast.ty = AstType::F32;
                }
            }
            AstData::Break => {}
            AstData::Loop { body } => {
                let local_count = self.begin_scope();
                self.resolve_lambda(body);
                self.end_scope(local_count);
            }
            AstData::For { name, pat, body, iter: (from, to), .. } => {
                let local_count = self.begin_scope();
                self.resolve_ast(from);
                self.resolve_ast(to);
                self.add_local(name.clone(), Local::Local { index: *pat, inline: None });
                self.resolve_lambda(body);
                self.end_scope(local_count);
            }
            AstData::While { cond, body } => {
                self.resolve_ast(cond);
                let local_count = self.begin_scope();
                self.resolve_lambda(body);
                self.end_scope(local_count);
            }
            AstData::FnLocal { name, lambda, local_index, .. } => {
                self.add_local(name.clone(), Local::Local { index: *local_index, inline: None });
                let local_count = self.begin_scope();

                let fn_type = self.resolve_lambda(lambda);
                self.module_infos[self.current_module].locals[*local_index as usize].0 = fn_type;

                self.end_scope(local_count);
            }
            AstData::LetLocal { name, init, local_index, ty: let_ty, attr, .. } => {
                
                let ty;
                if let Some(i) = init {
                    self.resolve_ast(i);
                    //dbg!(&i.ty);
                    ty = i.ty.clone();
                } else {
                    ty = AstType::None;
                }

                let mut local_type = std::mem::replace(&mut self.module_infos[self.current_module].locals[*local_index as usize].0, AstType::None);

                if let AstType::Any = &local_type {
                    // No declared type, use init expression type
                    local_type = ty;
                } else {
                    self.resolve_type(&mut local_type);
                }

                //dbg!(&local_type);
                *let_ty = local_type.clone(); // TODO: This is temporary, backend should look at the type stored in 'locals'
                std::mem::replace(&mut self.module_infos[self.current_module].locals[*local_index as usize].0, local_type);

                if attr == &Attr::Inline {
                    self.add_local(name.clone(), Local::Local {
                        index: *local_index,
                        inline: Some(init.as_ref().unwrap().clone())
                    });
                    
                    let loc = ast.loc;
                    *ast = Ast { loc, attr: Attr::None, ty: AstType::None, data: AstData::Void };
                } else {
                    self.add_local(name.clone(), Local::Local { index: *local_index, inline: None });
                }
            }
            AstData::NewObject { assignments } => {
                for (_, value) in assignments {
                    self.resolve_ast(value);
                }
            }
            AstData::NewCtor { box ctor, params } => {
                self.resolve_ast(ctor);
                for p in params {
                    self.resolve_ast(p);
                }
            }
            AstData::Use { name, rel_index } => {
                let abs_index = self.module_infos[self.current_module].import_map[*rel_index as usize];
                self.add_local(name.clone(), Local::ModuleRef { abs_index });
            }
            AstData::Ident { s } => {

                if let Some(local) = self.find_local(s) {
                    match local {
                        Local::Builtin { ty, .. } => {
                            //dbg!(ty);
                            ast.ty = ty.clone();
                            ast.data = AstData::Local { local: local.clone() };
                        }
                        Local::Local { index, inline } => {
                            if let Some(box inline) = inline {
                                *ast = inline.clone();
                            } else {
                                ast.ty = self.module_infos[self.current_module].locals[*index as usize].0.clone();
                                ast.data = AstData::Local { local: local.clone() };
                            }
                        }
                        _ => {
                            ast.data = AstData::Local { local: local.clone() };
                        }
                    }
                    
                }
            }
            AstData::Return { value } => {
                if let Some(v) = value {
                    self.resolve_ast(v);
                }
            }
            AstData::While { cond, body } => {
                self.resolve_ast(cond);
                let local_count = self.begin_scope();
                self.resolve_lambda(body);
                self.end_scope(local_count);
            }
            AstData::Block { expr } => {
                let local_count = self.begin_scope();
                for e in expr {
                    self.resolve_ast(e);
                }
                self.end_scope(local_count);
            }
            AstData::If { cond, body, else_body } => {
                self.resolve_ast(cond);
                let local_count = self.begin_scope();
                self.resolve_lambda(body);
                self.end_scope(local_count);

                if let Some(else_branch) = else_body {
                    self.resolve_ast(else_branch);
                }
            }
            AstData::Lambda { lambda } => {

                let local_count = self.begin_scope();
                let fn_type = self.resolve_lambda(lambda);
                self.end_scope(local_count);

                ast.ty = fn_type;
            }
            AstData::Array { elems } => {
                for e in elems {
                    self.resolve_ast(e);
                }

                // TODO: ast.ty = AstType::DynamicArray ..
            }
            AstData::Index { base, index } => {
                self.resolve_ast(base);
                self.resolve_ast(index);
                
                // TODO: ast.ty = base[index] type
            }
            AstData::Field { base, member } => {
                self.resolve_ast(base);
                match &member.data {
                    AstData::Ident { s } => {
                        match &base.data {
                            AstData::Local { local: Local::ModuleRef { abs_index } } => {
                                
                                //dbg!(&self.module_infos[*abs_index as usize].name);
                                //dbg!(self.module_infos[*abs_index as usize].exports_rev.len());
                                //dbg!(s);

                                if let Some(&module_local_index) = self.module_infos[*abs_index as usize].exports_rev.get(s) {
                                    
                                    *ast = Ast {
                                        loc: ast.loc,
                                        attr: Attr::None,
                                        ty: AstType::Any,
                                        data: AstData::Local {
                                            local: Local::ModuleMember {
                                                abs_index: *abs_index,
                                                local_index: module_local_index
                                            }
                                        }
                                    };
                                } else {
                                    // TODO: Could not find this member in the module
                                }
                            }
                            _ => {
                                // TODO: ast.ty = base.member type
                            }
                        }
                    }
                    _ => {
                        self.add_error(&ast, "invalid field access");
                        //panic!("invalid field access {:?}", &member.data)
                    }
                }
            }
            AstData::App { fun, params, .. } => {
                self.resolve_ast(fun);
                for e in params {
                    self.resolve_ast(e);
                }

                match &fun.ty {
                    AstType::Ctor(box ret_ty) => {
                        //dbg!(ret_ty);
                        ast.ty = ret_ty.clone();
                    }
                    _ => {}
                }
                // TODO: ast.ty = return type from fun type
            }
            AstData::Local { .. } => {
                
            }
        }
    }
}