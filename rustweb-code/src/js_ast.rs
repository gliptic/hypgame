use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub enum JsLocal {
    ModuleRef(u32),
    ModuleMember(u32, String),
    Builtin(JsBuiltin),
    SelfMember(u32),
    Local(String)
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum JsBuiltin {
    Glsl,
    Wasm
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsLit {
    Int(u64),
    Str(String),
    Bool(bool),
    Float(f64)
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum JsOp {
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
    BitOr,
    BitAnd,
    BitXor,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum JsUnop {
    Plus,
    Minus,
    Not,
    PreInc,
    PreDec,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsAst {
    Fn { index: u32, args: Vec<u32>, exported: bool, expr: Box<JsAst> },
    Use { name: String, rel_index: u32 },
    Path { path: Vec<String> },
    Builtin { builtin: JsBuiltin },
    Call { func: Box<JsAst>, args: Vec<JsAst> },
    MethodCall { receiver: Box<JsAst>, method: String, args: Vec<JsAst> },
    Lit { lit: JsLit },
    Block { stmts: Vec<JsAst> },
    Global { index: u32, constant: bool, value: Box<JsAst> },
    Locals { local_indexes: Vec<u32>, values: Vec<JsAst> },
    Assign { left: Box<JsAst>, right: Box<JsAst> },
    Field { base: Box<JsAst>, member: String },
    Index { expr: Box<JsAst>, index: Box<JsAst> },
    Lambda { inputs: Vec<String>, body: Box<JsAst> },
    Return { value: Box<JsAst> },
    Unary { value: Box<JsAst>, op: JsUnop },
    Binary { left: Box<JsAst>, op: JsOp, right: Box<JsAst> },
    ModuleMember { abs_index: u32, local_index: u32 },
    ModuleRef { abs_index: u32 },
    Local { index: u32 },
    If { cond: Box<JsAst>, then_branch: Vec<JsAst>, else_branch: Option<Box<JsAst>> },
    While { cond: Box<JsAst>, body: Vec<JsAst> },
    Loop { body: Vec<JsAst> },
    Array { elems: Vec<JsAst> },
    Undefined
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsModule {
    //pub name: String,
    //pub imports: Vec<String>, // TODO: deprecated
    //pub import_map: Vec<u32>,
    //pub exports: Vec<String>,
    pub items: Vec<JsAst>
}
