extern crate combine;
extern crate combine_language;

use combine::char::{string, letter, alpha_num};
use combine::{Parser, satisfy, Stream, ParseResult, ParseError, parser, chainl1};
use combine_language::{LanguageEnv, LanguageDef, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Divides(Box<Expr>, Box<Expr>),
}

fn calc_env<'a, I: 'a>() -> LanguageEnv<'a, I> 
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: ["if", "then", "else", "let", "in", "type"].iter()
                                                                .map(|x| (*x).into())
                                                                .collect(),
        },
        op: Identifier {
            start: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            rest: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            reserved: ["+", "-", "*", "/"].iter().map(|x| (*x).into()).collect()
        },
        comment_start: string("/*").map(|_| ()),
        comment_end: string("*/").map(|_| ()),
        comment_line: string("//").map(|_| ()),
    })
}
