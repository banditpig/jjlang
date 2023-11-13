use nom::bytes::complete::{tag, take_until};
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::fmt::{Display, Formatter};
use nom::branch::alt;
use nom::multi::{many0, separated_list0};
use crate::util_parsers::*;

#[derive(Debug, Clone, PartialEq)]

pub enum Atom {
    String(String),
    Name(String),

}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Comment,
    Void,
    Constant(Atom),
    Call(String, Vec<Expr>),
    Let(String, Box<Expr>),
    Closure(Vec<String>, Vec<Expr>),
    Function(String, Vec<String>, Vec<Expr>),
}
impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => { write!(f, "{s}")}
            Atom::Name(n) => { write!(f, "{n}")}
        }
    }
}
pub fn parse_name(input: &str) -> IResult<&str, String>{
    map(alpha1,String::from)(input)
}
pub fn parse_function(input: &str) -> IResult<&str, Expr>{
    /*
    fn <name>((arg,)*){
    (expr)*
    }
     */
    // let parse_func_name = preceded(
    //     ws(tag("fn")),
    //     ws(alpha1)
    // );
   // let parse_name = map(alpha1,String::from);
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","),ws(parse_name)),
        tag(")")
    );
    let parse_body = delimited(
        ws(tag("{")),
        ws(many0(ws(parse_expr))),
        tag("}")
    );
    let parser = preceded(tag("fn"),
                          tuple((ws(parse_name),
                                 ws(parse_args), ws(parse_body))));
        //;
    map(
        parser,
        |(name,args, body)| Expr::Function(name.to_string(), args, body)
    )(input)

}
pub fn parse_constant(input: &str) -> IResult<&str, Expr>{
   map(parse_atom, Expr::Constant)(input)
}
pub fn parse_atom(input: &str) -> IResult<&str, Atom>{
    alt((parse_var_name, parse_string))(input)
}
pub fn parse_var_name(input: &str) -> IResult<&str, Atom>{
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


pub fn parse_comment(input: &str) -> IResult<&str, Expr>{
    map(comment,|_|Expr::Comment)(input)
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
        tag("("),
        separated_list0(tag(","), ws(parse_expr)),
        tag(")")
    );
    let parser = tuple((parse_name, parse_arg));
    map(parser, |(name, arg)| Expr::Call(name.to_string(), arg))(input)
}
pub fn parse_closure(input: &str) -> IResult<&str, Expr> {

    let parse_name= map(alpha1,String::from);
    let parse_args = delimited(
        tag("|"),
        separated_list0(tag(","), parse_name),
        tag("|")
    );
    let parser = tuple(( ws(parse_args), parse_expr));
    map(parser, |(args, expr)| Expr::Closure(args, vec!(expr)))(input)

}
pub fn parse_expr(input: &str) -> IResult<&str, Expr>{
  alt((
      parse_function,
      parse_closure,
      parse_call,
      parse_let,
      parse_constant,
      parse_comment))(input)
}
pub fn parser(input: &str) -> IResult<&str, Vec< Expr>>{
    many0(ws(parse_expr))(input)
}