use nom::bytes::complete::{tag, take_until};
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::fmt::{Display, Formatter};
use std::os::unix::raw::ino_t;
use nom::branch::alt;
use nom::multi::many0;
use crate::util_parsers::*;

#[derive(Debug,Clone)]
#[derive(PartialEq)]
pub enum Atom {
    String(String),
    Name(String),

}
#[derive(Debug, Clone)]
pub enum Expr {
    Void,
    Constant(Atom),
    Call(String, Box<Expr>),
    Let(String, Box<Expr>)
}
impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => { write!(f, "{s}")}
            Atom::Name(n) => { write!(f, "{n}")}
        }
    }
}
pub fn parse_constant(input: &str) -> IResult<&str, Expr>{
   map(parse_atom, Expr::Constant)(input)
}
pub fn parse_atom(input: &str) -> IResult<&str, Atom>{
    alt((parse_name, parse_string))(input)
}
pub fn parse_name(input: &str) -> IResult<&str, Atom>{
   //any letters
    map(alpha1, |string: &str| Atom::Name(string.to_string()))(input)
}
pub fn parse_let(input: &str) ->    IResult<&str, Expr>{
    //let <string> = atom

    let parse_name = preceded(
        ws(tag("let")),
        ws(alpha1)
    );
    let parse_equals = preceded(
        tag("="),
        ws(parse_expr)
    );
    let parser = tuple((parse_name, parse_equals));
    map(parser,|(name,arg)|Expr::Let(name.to_string(), Box::new(arg)))(input)

}
pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    //" text "
    let parser = delimited(
        tag("\""),
        take_until("\""),
        tag("\"")
    );
    map(parser, |string: &str| Atom::String(string.to_string()))(input)

}
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    //name ( atom )
    let parse_name = ws(alpha1);
    let parse_arg = delimited(
        ws(tag("(")),
        parse_expr,
        tag(")")
    );
    let parser = tuple((parse_name, parse_arg));
    map(parser, |(name, arg)| Expr::Call(name.to_string(), Box::new(arg)))(input)
}
pub fn parse_expr(input: &str) -> IResult<&str, Expr>{
  alt((parse_let, parse_call, parse_constant))(input)
}
pub fn parser(input: &str) -> IResult<&str, Vec< Expr>>{
    many0(ws(parse_expr))(input)
}