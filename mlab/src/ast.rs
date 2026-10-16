//! Parse tree of the mlab language.

use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    LDiv,
    Pow,
    EMul,
    EDiv,
    ELDiv,
    EPow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    AndAnd,
    OrOr,
}

impl BinOp {
    /// Operator text for messages and printing anonymous functions (Octave style: `!=`).
    pub fn text(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::LDiv => "\\",
            BinOp::Pow => "^",
            BinOp::EMul => ".*",
            BinOp::EDiv => "./",
            BinOp::ELDiv => ".\\",
            BinOp::EPow => ".^",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "&",
            BinOp::Or => "|",
            BinOp::AndAnd => "&&",
            BinOp::OrOr => "||",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Plus,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostOp {
    CTranspose,
    Transpose,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Num(f64, String),
    Imag(f64, String),
    Str(String, bool),
    Ident(String),
    /// `:` as the «all» index
    Colon,
    /// `end` inside an index
    End,
    Paren(Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Postfix(PostOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Range(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
    Index(Box<Expr>, Vec<Expr>),
    Matrix(Vec<Vec<Expr>>),
    /// `{a, b}` — only as a list of alternatives in `case` (no cell arrays in v1)
    CellList(Vec<Expr>),
    AnonFn(Vec<String>, Rc<Expr>),
    FuncHandle(String),
    /// `x.name` — struct fields are not in v1; parsed to give a clear error
    Field(Box<Expr>, String),
}

#[derive(Clone, Debug)]
pub enum LValue {
    Var(String),
    Index(String, Vec<Expr>),
    /// `s.name = …` — structs are not in v1; parsed to give a runtime error rather than a parse error
    Field(String, String),
    Tilde,
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub enum StmtKind {
    Expr { expr: Expr, print: bool },
    Assign { lhs: Vec<LValue>, rhs: Expr, print: bool, op: Option<BinOp> },
    If { clauses: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>> },
    For { var: String, iter: Expr, body: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
    DoUntil { body: Vec<Stmt>, cond: Expr },
    Switch { subject: Expr, cases: Vec<(Expr, Vec<Stmt>)>, otherwise: Option<Vec<Stmt>> },
    Try { body: Vec<Stmt>, ident: Option<String>, catch_body: Vec<Stmt> },
    Break,
    Continue,
    Return,
    Command { name: String, args: Vec<String>, print: bool },
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub outputs: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Default)]
pub struct Program {
    pub body: Vec<Stmt>,
    pub functions: Vec<Rc<FunctionDef>>,
}
