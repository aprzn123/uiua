use enum_iterator::Sequence;

use crate::{
    function::FunctionId,
    lex::{Sp, Span},
    Ident,
};

#[derive(Debug, Clone)]
pub enum Item {
    FunctionDef(FunctionDef),
    Expr(Sp<Expr>),
    Let(Let),
    Const(Const),
}

impl Item {
    pub fn span(&self) -> Span {
        match self {
            Item::FunctionDef(func) => func
                .name
                .span
                .clone()
                .merge(func.func.body.expr.span.clone()),
            Item::Expr(expr) => expr.span.clone(),
            Item::Let(r#let) => r#let.pattern.span.clone().merge(r#let.expr.span.clone()),
            Item::Const(r#const) => r#const.name.span.clone().merge(r#const.expr.span.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Let {
    pub pattern: Sp<Pattern>,
    pub expr: Sp<Expr>,
}

#[derive(Debug, Clone)]
pub struct Const {
    pub name: Sp<Ident>,
    pub expr: Sp<Expr>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub doc: Option<Sp<String>>,
    pub name: Sp<Ident>,
    pub func: Func,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub id: FunctionId,
    pub params: Vec<Sp<Ident>>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<Item>,
    pub expr: Sp<Expr>,
}

impl From<Sp<Expr>> for Block {
    fn from(expr: Sp<Expr>) -> Self {
        Self {
            items: Vec::new(),
            expr,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Expr {
    #[default]
    Unit,
    If(Box<IfExpr>),
    Call(Box<CallExpr>),
    Bin(Box<BinExpr>),
    Pipe(Box<PipeExpr>),
    Real(String),
    Char(char),
    String(String),
    FormatString(Vec<String>),
    Ident(Ident),
    Placeholder,
    List(Vec<Sp<Expr>>),
    Array(Vec<Sp<Expr>>),
    Parened(Box<Sp<Expr>>),
    Func(Box<Func>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(Ident),
    List(Vec<Sp<Pattern>>),
    Discard,
}

#[derive(Debug, Clone)]
pub struct PipeExpr {
    pub left: Sp<Expr>,
    pub op: Sp<PipeOp>,
    pub right: Sp<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub cond: Sp<Expr>,
    pub if_true: Block,
    pub if_false: Block,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub func: Sp<Expr>,
    pub arg: Sp<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinExpr {
    pub left: Sp<Expr>,
    pub op: Sp<BinOp>,
    pub right: Sp<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Sequence)]
pub enum BinOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Compose,
    BlackBird,
    LeftThen,
    RightThen,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipeOp {
    Forward,
    Backward,
}
