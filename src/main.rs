#![allow(dead_code)]

use std::collections::HashMap;
use crate::eval::eval;
use crate::parse::{Expr, parser};

mod parse;
mod eval;
mod util_parsers;

fn main() {
    let txt = include_str!("../input.jj");
    let (_,  exprs) = parser(txt).unwrap();
    let mut ctx:HashMap<String, Expr> = HashMap::new();
    for expr in exprs{
        // dbg!( &expr);
        eval(expr, &mut ctx);
    }

    //dbg!(rest, res);
}
