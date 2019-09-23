extern crate combine;
extern crate combine_language;

use combine::char::{string, letter, alpha_num};
use combine::{Parser, satisfy, Stream, ParseResult, ParseError, parser, chainl1};
use combine_language::{LanguageEnv, LanguageDef, Identifier};

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

fn main() {
    let env = calc_env();
    let id = env.identifier();//An identifier parser
    let integer = env.integer();//An integer parser
    let result = (id, integer).easy_parse("this /* Skips comments */ 42");
    assert_eq!(result, Ok(((String::from("this"), 42), "")));
}
