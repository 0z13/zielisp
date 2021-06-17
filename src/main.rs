mod expressions;
mod parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use parser::SExpr;
use parser::testing;

use expressions::Expr; 
use expressions::ExprE;

fn main() {
    println!("zielispy testing repl:\n");
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
                let (_, x) = parser::parse_sexpr("(+ 3 3)").unwrap();
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





fn eval(e:&Expr) -> f64 {
    let res = match e {
        Expr::Prim(x) => *x,
        Expr::Plus(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            rh + lh
        }, 
         Expr::Minus(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            return rh - lh
        },                                        
         Expr::Mult(x,y) => {
            let rh = eval(x);
            let lh = eval(y);
            return rh * lh
    },
   };
    res
}

fn desugar(e:&ExprE) -> Expr{
    let res:Expr = match e {
        ExprE::Prim(x) => Expr::Prim(*x),
        ExprE::Plus(x, y)   => Expr::Plus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Minus(x, y)  => Expr::Mult(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Mult(x, y)   => Expr::Minus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::UMinus(x )              => Expr::Mult(Box::new(Expr::Prim(-1.0)), Box::new(desugar(x))),
    };
    res
}


#[test]
fn basic() {
    assert_eq!(eval(&Expr::Plus(Box::new(Expr::Prim(3.0)),Box::new(Expr::Prim(2.0)))), 5.0);
    assert_eq!(eval(&Expr::Mult(Box::new(Expr::Prim(1.0)),Box::new(Expr::Prim(0.0)))), 0.0);
}
