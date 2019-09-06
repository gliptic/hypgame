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

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    Lshr,
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
    BitNot,
    PreInc,
    PreDec,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum JsPattern {
    Array(Vec<JsPattern>),
    Local(u32)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum JsAst {
    Fn { index: u32, args: Vec<JsPattern>, exported: bool, expr: Box<JsAst> },
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
    Lambda { inputs: Vec<JsPattern>, body: Box<JsAst> },
    Return { value: Box<JsAst> },
    Unary { value: Box<JsAst>, op: JsUnop },
    Binary { left: Box<JsAst>, op: JsOp, right: Box<JsAst> },
    ModuleMember { abs_index: u32, local_index: u32 },
    ModuleRef { abs_index: u32 },
    Local { index: u32 },
    If { cond: Box<JsAst>, then_branch: Vec<JsAst>, else_branch: Option<Box<JsAst>> },
    While { cond: Box<JsAst>, body: Vec<JsAst> },
    For { pre: Box<JsAst>, cond: Box<JsAst>, post: Box<JsAst>, body: Vec<JsAst> },
    Break,
    Loop { body: Vec<JsAst> },
    Array { elems: Vec<JsAst> },
    NewObject { assignments: Vec<(String, JsAst)> },
    NewCtor { ctor: Box<JsAst>, params: Vec<JsAst> },
    Undefined
}

impl JsAst {
    pub fn is_expr(&self) -> bool {
        match self {
              JsAst::Loop { .. }
            | JsAst::Locals { .. }
            | JsAst::Global { .. }
            | JsAst::Return { .. }
            | JsAst::While { .. }
            | JsAst::Use { .. }
            | JsAst::Break { .. }
            | JsAst::For { .. }
            | JsAst::Fn { .. } => false,
            JsAst::If { then_branch, else_branch, .. } =>
                else_branch.is_some() &&
                then_branch.len() > 0 && then_branch.iter().all(|x| x.is_expr()) &&
                else_branch.iter().all(|x| x.is_expr()),
            JsAst::Block { stmts } =>
                stmts.len() > 0 && stmts.iter().all(|x| x.is_expr()),
            _ => true
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsModule {
    pub items: Vec<JsAst>
}
