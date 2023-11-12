use std::collections::HashMap;
use crate::parse::{Atom, Expr};

pub fn eval(expr: Expr, ctx: &mut HashMap<String, Expr>) -> Expr{
    match expr{
        Expr::Call(name,args ) => {
            if name == "println"{
                for arg in args{
                    print!("{:?}", eval(arg, ctx));
                }
                println!()

            }else {
                match ctx.get(&name){
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = ctx.clone();
                        for (paramter, arg) in parameters.into_iter().zip(args.into_iter()){
                            let expr = eval(arg, &mut scope);
                            scope.insert(paramter.clone(), expr);
                            for expr in body{
                                eval(expr.clone(), &mut scope);
                            }
                        }

                       // eval(args,ctx
                    }
                    _ => {panic!("Expected closure")}
                }
            }
            Expr::Void
        },
        Expr::Constant(ref atom) => match atom{
            Atom::Name(ref name) =>  ctx.get(name).unwrap().clone(),
            _ => expr
        },
        Expr::Let(name, value) => {
            ctx.insert(name, *value);
            Expr::Void
        }
        Expr::Void | Expr::Closure(_, _) =>  expr,
        Expr::Comment => expr,

    }
}