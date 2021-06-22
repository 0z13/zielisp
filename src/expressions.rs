use core::fmt;
use std::{collections::HashMap, format};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprE {
    Prim(f64),
    Id(String),
    LetBinding(String, Box<ExprE>, Box<ExprE>),
    AppC(Box<ExprE>, Box<ExprE>),
    FdC(String, Box<ExprE>), // Argument, body
    Plus(Box<ExprE>, Box<ExprE>),
    Minus(Box<ExprE>, Box<ExprE>),
    Mult(Box<ExprE>, Box<ExprE>),
    UMinus(Box<ExprE>),
    IfC(Box<ExprE>, Box<ExprE>, Box<ExprE>),
    TrueC,
    FalseC,
    GT(Box<ExprE>, Box<ExprE>),
    LT(Box<ExprE>, Box<ExprE>),
    EQ(Box<ExprE>, Box<ExprE>)
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Prim(f64),
    Id(String),
    AppC(Box<Expr>, Box<Expr>),
    FdC(String, Box<Expr>), // argument, body
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    IfC(Box<Expr>, Box<Expr>, Box<Expr>),
    TrueC,
    FalseC,
    GT(Box<Expr>, Box<Expr>),
    LT(Box<Expr>, Box<Expr>),
    EQ(Box<Expr>, Box<Expr>)
}




pub type Env = HashMap<String, Val>;
#[derive(Clone, Debug)]
pub enum Val {
    BoolV(bool),
    NumV(f64),
    ClosV(Expr, Env)
}

impl Val {
    pub fn unpack_val(&self) -> f64 {
        if let &Val::NumV(x) = self {
            x
        } else {
            panic!("wrong getter")
        }
    }
}

impl Expr {
    fn show(&self)  -> String {
        "closure probably".to_string()
    }
}
impl fmt::Display for Val {
    fn fmt (&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match &self {
            &Val::BoolV(b ) => write!(f, "{}", b),
            &Val::NumV(a ) => write!(f, "{}", a),
            &Val::ClosV(x, _) => write!(f, "{}", x.show()),
        }
    }
}