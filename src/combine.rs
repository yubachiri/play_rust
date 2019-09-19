 use combine::parser::range::{range, take_while1};
 use combine::parser::repeat::{sep_by};
 use combine::parser::Parser;
 use combine::stream::{RangeStream, state::State};
 use combine::error::ParseError;
 
// Copy the fn header as is, only change ------------╮
//                                                   v
fn tools<'a, I>() -> impl Parser<Input = I, Output = Vec<&'a str>>
    where I: RangeStream<Item = char, Range=&'a str>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let tool = take_while1(|c : char| c.is_alphabetic());
    sep_by(tool, range(", "))
}

fn decode(input : &str) -> Result<Vec<&str>, String> {
    match tools().easy_parse(State::new(input)) {
        Ok((output, _remaining_input)) => Ok(output),
        Err(err) => Err(format!("{} in `{}`", err, input)),
    }
}

fn main() {
    let input = "Hammer, Saw, Drill";
    let output = decode(input).unwrap();
    println!("{:?}", output);
}
