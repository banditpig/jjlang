#![allow(dead_code)]

use crate::parse::parse_expr;

mod parse;
mod eval;
mod util_parsers;

fn main() {
    let txt = include_str!("../input.jj");
    let (rest, res) = parse_expr(txt).unwrap();
    dbg!(rest, res);
}
