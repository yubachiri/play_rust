extern crate combine;
extern crate combine_language;

use combine::char::{string, spaces, letter, alpha_num};
use combine::{Parser, satisfy, Stream, ParseResult, ParseError, many1};
use combine_language::{LanguageEnv, LanguageDef, Identifier};

// #[derive(Debug, Clone, PartialEq, Eq)]
// enum Expr {
//     Number(i64),
//     Plus(Box<Expr>, Box<Expr>),
//     Minus(Box<Expr>, Box<Expr>),
//     Times(Box<Expr>, Box<Expr>),
//     Divides(Box<Expr>, Box<Expr>),
// }

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

// fn numbers<I>(input: I) -> ParseResult<Box<Expr>, I>
//     where
//         I: Stream<Item=char>,
//         I: combine::StreamOnce,
//         <I as combine::StreamOnce>::Error: combine::ParseError<char, <I as combine::StreamOnce>::Range, <I as combine::StreamOnce>::Position>
// {
//     let env = calc_env();
//     let number = env.integer().map(|x| Box::new(Expr::Number(x)));
//     // let parenthesized = env.parens(parser(expr));
//     number.parse_stream(input)
// }

// fn integer_literal<I: 'static + Stream<Item=char>>() -> Box<Parser<Input = I, Output = i64>> {
//     let sign = optional(token('+').or(token('-')));

//     let number = many1::<Vec<char>, _>(digit())
//         .map(|ds| {
//             ds.into_iter()
//                 .map(|c| c.to_digit(10).unwrap() as i64)
//                 .fold(0, |acc, x| acc * 10 + x)
//         });

//     let signed_number = (sign, number)
//         .map(|(s, num)| {
//             match s {
//                 None | Some('+') => num,
//                 Some('-') => -num,
//                 _ => unreachable!(),
//             }
//         });

//     Box::new(signed_number)
// }

fn main() {
    let env = calc_env();
    // let id = env.identifier();//An identifier parser
    let mut integer = many(env.integer());//An integer parser
    // let result = (id, integer).easy_parse("this /* Skips comments */ 42");
    let result = integer.easy_parse("42 55 12");
    assert_eq!(result, Ok((42, "")));
    // assert_eq!(result, Ok(((String::from("this"), 42), "")));
}
