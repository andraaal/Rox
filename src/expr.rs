use crate::scanner::Location;

#[derive(Debug)]
pub enum Expr {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
}

pub struct LocExpr {
    expr: Expr,
    start: Location,
    end: Location,
}