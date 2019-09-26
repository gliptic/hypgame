pub mod parser;
pub mod resolver;

use std::path::{Path, PathBuf};
use std::collections::{HashMap};
use crate::conflict_tree::ConflictTree;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use num::Integer;

pub type AstRef = Box<Ast>;
pub type LocalRef = u32;
pub type Ident = String;

#[derive(Debug)]
pub struct ParseError(pub Span, pub &'static str);

pub type ParseResult<T> = Result<T, ParseError>;
pub type ModuleResult<T> = Result<T, (Vec<u8>, PathBuf, ParseError)>;

#[derive(Clone, Debug)]
pub struct ParamDef {
    pub pat: Pattern,
    pub ty: AstType,
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Local(u32),
    Array(Vec<Pattern>)
}

#[derive(Clone, Copy, Debug)]
pub enum Builtin {
    Star,
    Vec2Ctor,
    Mat2Ctor
}

impl Builtin {
    fn get_type(&self) -> AstType {
        use Builtin::*;
        match self {
            _ => AstType::Any,
            Vec2Ctor => AstType::Ctor(Box::new(AstType::Vec(Box::new(AstType::F32), 2))),
            Mat2Ctor => AstType::Ctor(Box::new(AstType::Mat(Box::new(AstType::F32), 2, 2))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Local {
    ModuleRef { abs_index: u32 },
    ModuleMember { abs_index: u32, local_index: u32 },
    //Builtin { name: String, ty: AstType },
    Builtin(Builtin),
    //SelfMember { member_index: u32 },
    Local { index: u32, inline: Option<Box<Ast>> }
}

#[derive(Clone, Debug)]
pub struct AstLambda {
    pub params: Vec<ParamDef>,
    pub param_locals: Vec<u32>,
    pub expr: Vec<Ast>,
    pub return_type: AstType
}

impl AstLambda {
    pub fn new_empty() -> AstLambda {
        AstLambda {
            params: vec![],
            param_locals: vec![],
            expr: vec![],
            return_type: AstType::None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AppKind {
    Normal,
    Binary,
    Unary
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attr {
    None,
    Attribute,
    Varying,
    Uniform,
    Binary,
    Inline,
    Debug
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Js,
    Glsl,
    Binary
}

#[derive(Clone, Debug)]
pub struct Use(pub Attr, pub Ident);

#[derive(Clone, Debug)]
pub enum AstData {
    //ConstNum { v: u64 },
    //ConstFloat { v: f64 },
    ConstLit { v: Lit },
    ConstStr { v: String },
    Ident { s: Ident },
    Local { local: Local },
    App { fun: AstRef, params: Vec<Ast>, kind: AppKind },
    Lambda { lambda: AstLambda },
    Block { expr: Vec<Ast> },
    TypeDef { index: u32 },
    LetLocal { name: Ident, ty: AstType, init: Option<AstRef>, local_index: u32, attr: Attr },
    FnLocal { name: Ident, lambda: AstLambda, local_index: u32, exported: bool },
    Field { base: AstRef, member: AstRef },
    For { pat: u32, iter: (AstRef, AstRef), body: AstLambda },
    Index { base: AstRef, index: AstRef }, // TODO: Same as above?
    Array { elems: Vec<Ast> },
    If { cond: AstRef, body: AstLambda, else_body: Option<AstRef> },
    While { cond: AstRef, body: AstLambda },
    Loop { body: AstLambda },
    Break,
    Return { value: Option<AstRef> },
    Use { name: Ident, rel_index: u32 },
    NewObject { assignments: Vec<(Ident, Ast)> },
    NewCtor { ctor: Box<Ast>, params: Vec<Ast> },
    Void
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstType {
    None,
    Any,
    U64,
    I64,
    F64,
    U32,
    I32,
    F32,
    U8,
    Str,
    FixedArr(Box<AstType>, usize),
    Ptr(Box<AstType>),
    MemLoc(Box<AstType>),
    MemStruct(Rc<StructDef>),
    ArrStruct(Rc<ArrayStructDef>),
    Fn(Box<AstType>, Vec<AstType>),
    Ctor(Box<AstType>), // Arbitrary parameters for now
    Vec(Box<AstType>, u32),
    Mat(Box<AstType>, u32, u32),
    Other(Ident)
}

impl AstType {
    fn align_of_size_of(&self) -> (usize, usize) {
        match self {
            AstType::U8 => (1, 1),
            AstType::U32 | AstType::I32 | AstType::F32 => (4, 4),
            AstType::U64 | AstType::I64 | AstType::F64 => (8, 8),
            AstType::FixedArr(ty, count) => {
                let (a, s) = ty.align_of_size_of();
                (a, s * count)
            }
            AstType::MemLoc(_) | AstType::Ptr(_) => (4, 4),
            AstType::MemStruct(def) => def.align_of_size_of(),
            AstType::ArrStruct(_) => panic!("array struct has no fixed size"),
            _ => panic!("type {:?} has no size", &self),
        }
    }

    fn ptr_shift(&self) -> i32 {
        match self {
            AstType::MemStruct(def) => {
                def.ptr_shift()
            }
            other => {
                let (align_of, _) = self.align_of_size_of();
                mult_to_shift(align_of)
            }
        }
    }

    fn ptr_convert_shift(&self, to: &AstType) -> i32 {
        let from_sh = self.ptr_shift();
        let to_sh = to.ptr_shift();

        to_sh - from_sh
    }

    fn arr_offset_mult(&self) -> usize {
        let (_, size_of) = self.align_of_size_of();
        size_of / (1 << self.ptr_shift())
    }

    fn shifted_offset(&self, offset: usize) -> usize {
        let sh = self.ptr_shift();
        assert_eq!((offset >> sh) << sh, offset);
        offset >> sh
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructField {
    name: String,
    ty: AstType,
    offset: Cell<usize>
}

impl StructField {
    pub fn new(name: String, ty: AstType) -> StructField {
        StructField {
            name,
            ty,
            offset: Cell::new(0)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructDef {
    fields: Vec<StructField>,
    computed_offset: Cell<bool>,
    size_of: Cell<usize>,
    align_of: Cell<usize>,
    multiple: Cell<usize>,
}

impl StructDef {
    pub fn new(fields: Vec<StructField>) -> StructDef {
        StructDef {
            fields,
            computed_offset: Cell::new(false),
            size_of: Cell::new(0),
            align_of: Cell::new(0),
            multiple: Cell::new(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayStructField {
    name: String,
    ty: AstType
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayStructDef {
    fields: Vec<ArrayStructField>,
}

fn align_offset(offset: usize, align: usize) -> usize {
    align * ((offset + align - 1) / align)
}

impl StructDef {
    pub fn ensure_offsets_computed(&self) {
        if !self.computed_offset.get() {
            self.computed_offset.set(true);

            let mut offset = 0;
            let mut total_align_of = 1;
            for f in &self.fields {
                let (align_of, size_of) = f.ty.align_of_size_of();
                offset = align_offset(offset, align_of) + size_of;
                total_align_of = total_align_of.lcm(&align_of);
            }

            self.align_of.set(total_align_of);
            self.size_of.set(align_offset(offset, total_align_of));
        }
    }

    pub fn align_of_size_of(&self) -> (usize, usize) {
        self.ensure_offsets_computed();
        (self.align_of.get(), self.size_of.get())
    }

    pub fn ptr_shift(&self) -> i32 {
        self.ensure_offsets_computed();

        let mult = self.multiple.get();
        mult_to_shift(mult)
    }

    pub fn shifted_offset(&self, offset: usize) -> usize {
        let sh = self.ptr_shift();
        assert_eq!((offset >> sh) << sh, offset);
        offset >> sh
    }
}

fn mult_to_shift(mult: usize) -> i32 {
    if mult.is_power_of_two() {
        mult.trailing_zeros() as i32
    } else {
        1
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AstTy(u32);

impl AstTy {
    const fn uninit() -> AstTy {
        AstTy(0)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AstTyParams {
    params: Vec<AstTy>,
    rest: bool
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AstTyInt {
    bits: u32,
    signed: bool
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AstTyData {
    None,
    Any,
    Int(AstTyInt),
    Float(u32),
    Fn(AstTy, AstTyParams),
    Vec(AstTy, u32),
    Mat(AstTy, u32, u32)
}

impl AstType {
    pub fn is_any(&self) -> bool {
        match self {
            AstType::Any => true,
            _ => false
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            AstType::Fn { .. } => true,
            _ => false
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct Ast {
    pub ty: AstType,
    pub loc: Span,
    pub attr: Attr,
    pub data: AstData
}

#[derive(Clone, Debug)]
pub enum Lit {
    Float(f64),
    Int(i64),
}

#[derive(Clone, Debug)]
pub struct LocalDef {
    pub ty: AstType,
    pub name: Ident,
    pub is_mut: bool,
    pub const_value: Option<Lit>,
    pub need_qualification: bool
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: Ident,
    pub ty: AstType,
}

// TODO: ModuleInfo is a very large subset of Module

#[derive(Clone, Debug)]
pub struct Module {
    pub lambda: AstLambda,
    pub src: Vec<u8>,
    pub path: PathBuf,
    pub uses: Vec<Use>,
    pub locals: Vec<LocalDef>,
    pub local_types: Vec<TypeDef>,
    pub exports: Vec<u32>, // indexes into locals
    pub exports_rev: HashMap<Ident, u32>,
    pub language: Language,
}

#[derive(Debug)]
pub struct ModuleInfo {
    pub name: String,
    pub src: Vec<u8>,
    pub path: PathBuf,
    pub locals: Vec<LocalDef>,
    pub local_types: Vec<TypeDef>,
    pub exports: Vec<u32>, // indexes into locals
    pub exports_rev: HashMap<Ident, u32>,
    pub import_map: Vec<u32>,
    pub conflict_tree: ConflictTree<u32>,
    pub language: Language,
    pub external: bool
}


pub fn print_line_at(data: &[u8], err: &ParseError, path: &Path) {
    let mut start = (err.0).0;
    while start > 0 && data[start - 1] != b'\n' {
        start -= 1;
    }
    let mut line = 1;
    let mut col = 1;
    for &d in &data[..(err.0).0] {
        if d == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    let mut stop = (err.0).0;
    while stop + 1 < data.len() && data[stop + 1] != b'\n' {
        stop += 1;
    }

    println!("--> {}:{}:{}", &path.display(), line, col);
    println!(" | {}", std::str::from_utf8(&data[start..stop]).unwrap());
    println!(" {}", err.1);
}

impl ModuleInfo {
    pub fn from_module(module_name: String, import_map: Vec<u32>, module: Module) -> (AstLambda, ModuleInfo) {
        (module.lambda, ModuleInfo {
            name: module_name,
            src: module.src,
            path: module.path,
            locals: module.locals,
            local_types: module.local_types,
            exports: module.exports,
            exports_rev: module.exports_rev,
            conflict_tree: ConflictTree::new(),
            import_map,
            language: module.language,
            external: false
        })
    }

    pub fn from_externals(
        module_name: String,
        locals: Vec<LocalDef>,
        exports: Vec<u32>,
        language: Language) -> (AstLambda, ModuleInfo) {

        let mut exports_rev = HashMap::new();
        for &export in &exports {
            exports_rev.insert(locals[export as usize].name.clone(), export);
        }

        (AstLambda::new_empty(), ModuleInfo {
            name: module_name,
            src: vec![],
            path: PathBuf::new(),
            locals,
            local_types: vec![], // TODO
            exports,
            exports_rev,
            conflict_tree: ConflictTree::new(),
            import_map: vec![],
            language,
            external: true
        })
    }

    pub fn print_line_at(&self, err: &ParseError) {
        let data = &self.src;
        let path = &self.path;
        print_line_at(data, err, path);
    }
}
