use std::collections::HashMap;
use crate::parse::{Atom, Expr};

pub fn eval(expr: Expr, ctx: &mut HashMap<String, Expr>) -> Expr{
    match expr{
        Expr::Call(name,args ) => {
            if name == "println"{
                for arg in args{
                    print!("{}", eval(arg, ctx));
                }
                println!()

            }else {
                match ctx.get(&name){
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = ctx.clone();
                        for (parameter, arg) in parameters.into_iter().zip(args.into_iter()) {
                            let expr = eval(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }
                        for expr in body{
                           if let Expr::Return(expr) = eval(expr.clone(), &mut scope){
                               return *expr;
                           }
                        }
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
            let expr = eval(*value, ctx);
            ctx.insert(name, expr);
            Expr::Void
        }
        Expr::Void | Expr::Closure(_, _) =>  expr,
        Expr::Comment => expr,

        Expr::Function(name , args, body) => {
            ctx.insert(name, Expr::Closure(args, body));
            Expr::Void
        }
        Expr::Return(expr) => Expr::Return( Box::new(eval(*expr, ctx)))
    }
}