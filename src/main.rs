mod expressions;
mod parser;
use expressions::Val;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use parser::SExpr;
use parser::testing;

use expressions::Expr; 
use expressions::ExprE;

// Todo
// 1. Add conditionals
// 2. Add functions
// Support file reading
// Get factorial to work
// Start working on da compiler.


fn main() {
    let ex1  = ExprE::IfC(Box::new(ExprE::TrueC), Box::new(ExprE::Prim(2.0)), Box::new(ExprE::Prim(3.0)));
    println!("{}", eval(&desugar(&ex1)));
    parser::testing();
    run_repl();
    
}

fn run_repl() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {

        println!("No previous history.");
    }    
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let (_, x) = parser::parse_sexpr(&line).unwrap();
                let parsed = parser::parse(x);
                let desugared = desugar(&parsed);
                let res = eval(&desugared);
                println!("zielispy: {}", res);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}


fn eval(e:&Expr) -> Val {
    let res = match e {
        Expr::Prim(x) => Val::NumV(*x),
        Expr::Plus(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            Val::NumV(Val::unpack_val(&rh) + Val::unpack_val(&lh))
        }, 
        Expr::Minus(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            Val::NumV(Val::unpack_val(&rh) - Val::unpack_val(&lh))
        },                                        
        Expr::Mult(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            Val::NumV(Val::unpack_val(&rh) - Val::unpack_val(&lh))
        },
        Expr::IfC(cond,tVal,fVal) => {
            if let Val::BoolV(b) = eval(cond) {
                if b { 
                    return eval(tVal) 
                } else {
                    return eval(fVal)
                }
            } else {
                panic!("fkk")
            }
        },
        Expr::FalseC => Val::BoolV(false),
        Expr::TrueC  => Val::BoolV(true)
   };
    res
}

fn desugar(e:&ExprE) -> Expr {
    let res:Expr = match e {
        ExprE::Prim(x) => Expr::Prim(*x),
        ExprE::Plus(x, y)   => Expr::Plus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Minus(x, y)  => Expr::Mult(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Mult(x, y)   => Expr::Minus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::UMinus(x )              => Expr::Mult(Box::new(Expr::Prim(-1.0)), Box::new(desugar(x))), // parser don't support yet
        ExprE::IfC(x,y,z)  => Expr::IfC(Box::new(desugar(x)),Box::new(desugar(y)), Box::new(desugar(z))),
        ExprE::TrueC              => Expr::TrueC,
        ExprE::FalseC             => Expr::FalseC
    };
    res
}


#[test]
fn basic() {
    //assert_eq!(eval(&Expr::Plus(Box::new(Expr::Prim(3.0)),Box::new(Expr::Prim(2.0)))), 5.0);
    //assert_eq!(eval(&Expr::Mult(Box::new(Expr::Prim(1.0)),Box::new(Expr::Prim(0.0)))), 0.0);
}
