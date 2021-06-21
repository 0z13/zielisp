mod expressions;
mod parser;
use expressions::Val;
use expressions::Env;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use expressions::Expr; 
use expressions::ExprE;

// Todo
// 1. Add conditionals
// 2. Add functions
// Support file reading
// Get factorial to work
// Start working on da compiler.


fn main() {
    let _ex1  = ExprE::IfC(Box::new(ExprE::TrueC), Box::new(ExprE::Prim(2.0)), Box::new(ExprE::Prim(3.0)));
    //  f1 = fdC("double", "x", plusC(idC("x"), idC("x")))
    // ((let add1 (|x| (x+1))
    //     (add1 5))
        
    let ex2 = ExprE::Plus(Box::new(ExprE::Id("x".to_string())), Box::new(ExprE::Id("x".to_string())));
    let ex3 = ExprE::FdC("x".to_string(), Box::new(ex2));
    let tst = ExprE::AppC(Box::new(ex3), Box::new(ExprE::Prim(6.0)));
    let (_, s)= parser::parse_sexpr("((|x| (+ x 1)) (+ 10 10))").unwrap();
    println!("{:?}", s);
    let me_val = parser::parse(s);
    let env:Env = HashMap::new();
    println!("{}", eval(&desugar(&me_val), &env));
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
                let env:Env = HashMap::new();
                let res = eval(&desugared, &env);
                println!("zielisp: {}", res);
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


fn eval(e:&Expr, env:&Env) -> Val {
    let res = match e {
        Expr::Prim(x) => Val::NumV(*x),
        Expr::AppC(fun, arg) => {
            let clos = eval(fun, env);
            let arg = eval(arg, env);
            match clos {
                Val::ClosV(s, local_env) =>  {
                    if let Expr::FdC(arg_name, body) = s {
                        let mut env_copy = local_env.clone();
                        let _ = env_copy.insert(arg_name, arg );
                        eval(&body, &env_copy)
                    } else {
                        panic!("closures are hard")
                    }
                }
                _ => panic!("closure are hard 2")
            }
        },
        Expr::FdC(_, _) => Val::ClosV(e.clone(), env.clone()), // gotta clone em' all
        Expr::Id(s) => {
            let v = env.get(s).unwrap();
            v.clone()
        } 
        //Expr::LetBinding(s, e) => Val::NumV(900.0), // get the let binding working l8er
        Expr::Plus(x,y) => {
            let rh = eval(x, env);
            let lh = eval(y, env);
            Val::NumV(Val::unpack_val(&rh) + Val::unpack_val(&lh))
        }, 
        Expr::Minus(x,y) => {
            let rh = eval(x, env);
            let lh = eval(y, env);
            Val::NumV(Val::unpack_val(&rh) - Val::unpack_val(&lh))
        },                                        
        Expr::Mult(x,y) => {
            let rh = eval(x, env);
            let lh = eval(y, env);
            Val::NumV(Val::unpack_val(&rh) - Val::unpack_val(&lh))
        },
        Expr::IfC(cond,t_val,f_val) => {
            if let Val::BoolV(b) = eval(cond, env) {
                if b { 
                    return eval(t_val, env) 
                } else {
                    return eval(f_val, env)
                }
            } else {
                panic!("Weird ifC")
            }
        },
        Expr::FalseC => Val::BoolV(false),
        Expr::TrueC  => Val::BoolV(true),
        Expr::GT(x, y) => {
            let rh = eval(x, env);
            let lh = eval(y, env);
            if Val::unpack_val(&rh) < Val::unpack_val(&lh) {
                Val::BoolV(true)
            } else {
                Val::BoolV(false)
            }
        },
        Expr::LT(x, y) => {
            let rh = eval(x, env);
            let lh = eval(y, env);
            println!("{}{}", &rh, &lh);
            if Val::unpack_val(&rh) > Val::unpack_val(&lh) {
                Val::BoolV(true)
            } else {
                Val::BoolV(false)
            }
        },
        // Dur ikke, hvooooooorfoorrrrrr
        // Equality ved ikke stoerre end og mindre end?
        Expr::EQ(x, y) => {
            let rh = Val::unpack_val(&eval(x, env));
            let lh = Val::unpack_val(&eval(y, env));
            println!("{}{}", &rh, &lh);
            if rh == lh { 
                Val::BoolV(true)
            } else {
                Val::BoolV(false)
            }
        }
   };
    res
}

fn desugar(e:&ExprE) -> Expr {
    let res:Expr = match e {
        ExprE::Prim(x)                        => Expr::Prim(*x),
        ExprE::Id(x)                        =>        Expr::Id(x.clone()),
        ExprE::Plus(x, y)    => Expr::Plus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Minus(x, y)  => Expr::Mult(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::Mult(x, y)   => Expr::Minus(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::UMinus(x )              => Expr::Mult(Box::new(Expr::Prim(-1.0)), Box::new(desugar(x))), // parser don't support yet
        ExprE::IfC(x,y,z)  => Expr::IfC(Box::new(desugar(x)),Box::new(desugar(y)), Box::new(desugar(z))),
        ExprE::TrueC              => Expr::TrueC,
        ExprE::FalseC             => Expr::FalseC,
        ExprE::LT(x, y)   => Expr::LT(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::EQ(x, y)   => Expr::EQ(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::GT(x, y)   => Expr::GT(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::AppC(x, y)     =>       Expr::AppC(Box::new(desugar(x)), Box::new(desugar(y))),
        ExprE::FdC(x,y)          =>       Expr::FdC(x.clone(), Box::new(desugar(y))),
        ExprE::LetBinding(name, body) => Expr::Prim(10.0)
    };
    res
}


#[test]
fn basic() {
    //assert_eq!(eval(&Expr::Plus(Box::new(Expr::Prim(3.0)),Box::new(Expr::Prim(2.0)))), 5.0);
    //assert_eq!(eval(&Expr::Mult(Box::new(Expr::Prim(1.0)),Box::new(Expr::Prim(0.0)))), 0.0);
}
