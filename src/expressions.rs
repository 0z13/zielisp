
#[derive(Debug, Clone, PartialEq)]
pub enum ExprE {
    Prim(f64),
    Plus(Box<ExprE>, Box<ExprE>),
    Minus(Box<ExprE>, Box<ExprE>),
    Mult(Box<ExprE>, Box<ExprE>),
    UMinus(Box<ExprE>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Prim(f64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}
