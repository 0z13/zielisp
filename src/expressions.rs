use core::fmt;
use std::format;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprE {
    Prim(f64),
    Plus(Box<ExprE>, Box<ExprE>),
    Minus(Box<ExprE>, Box<ExprE>),
    Mult(Box<ExprE>, Box<ExprE>),
    UMinus(Box<ExprE>),
    IfC(Box<ExprE>, Box<ExprE>, Box<ExprE>),
    TrueC,
    FalseC
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Prim(f64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    IfC(Box<Expr>, Box<Expr>, Box<Expr>),
    TrueC,
    FalseC
}


pub enum Val {
    BoolV(bool),
    NumV(f64)
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



impl fmt::Display for Val {
    fn fmt (&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match self {
            &Val::BoolV(b ) => write!(f, "{}", b),
            &Val::NumV(x ) => write!(f, "{}", x)
        }
    }
}