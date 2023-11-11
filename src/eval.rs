use std::collections::HashMap;
use crate::parse::{Atom, Expr};

pub fn eval(expr: Expr, ctx: &mut HashMap<String, Expr>) -> Expr{
    match expr{
        Expr::Call(name,arg ) => {
            if name == "println"{
                let arg = eval(*arg, ctx);
                println!("{:?}", arg);


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
        Expr::Void =>  expr


    }

}