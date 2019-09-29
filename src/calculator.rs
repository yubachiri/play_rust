extern crate combine;
extern crate combine_language;

use combine::char::{string, letter, spaces};
use combine::{Parser, many};
use combine_language::{Assoc, Fixity, expression_parser};
//TODO unignore once its stops ICEing
use self::Expr::*;

#[derive(PartialEq, Debug)]
enum Expr {
    Id(String),
    Op(Box<Expr>, &'static str, Box<Expr>)
}
fn op(l: Expr, o: &'static str, r: Expr) -> Expr {
    Op(Box::new(l), o, Box::new(r))
}
fn id(s: &str) -> Expr {
    Id(String::from(s))
}

fn main(){
    let op_parser = string("+").or(string("*"))
        .map(|op| {
            let prec = match op {
                "+" => 6,
                "*" => 7,
                _ => unreachable!()
            };
            (op, Assoc { precedence: prec, fixity: Fixity::Left })
        })
        .skip(spaces());
    let term = many(letter())
        .map(Id)
        .skip(spaces());
    let mut parser = expression_parser(term, op_parser, op);
    let result = parser.parse("a + b * c + d");
    assert_eq!(result, Ok((op(op(id("a"), "+", op(id("b"), "*", id("c"))), "+", id("d")), "")));
}
