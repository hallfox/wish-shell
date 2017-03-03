#[macro_use]
extern crate nom;

use nom::{IResult, digit};

use std::str;
use std::str::FromString;

mod wish;
use wish::*;

// Grammar rules:
// wish ::= <job> <expr>*
// job ::= <string>
// expr ::= <constant>
//        | <keyword>
//        | <app>
// constant ::= <number> | <string>
// app ::= (<expr>+)
//
// number ::= <sign> <real>
// real ::= <sign> <ureal>
// ureal ::= <uinteger> | <decimal>
// uinteger ::= <digit>+
// sign ::= <empty> | + | -
//
// string ::= "<strchr>*"
// strchr ::= <not (\ | ")>

// wish command, a binary/builtin with expressions
named!(wish<Wish>, do_parse!(
    cmd: job >>
    args: many0!(wexpr) >>
    Wish::new(cmd, args)
));

// wish expressions, lispy an prefixed
named!(wexpr<Wexpr>, ws!(alt_complete!(
    wconstant |
    wkeyword |
    wapp => { |res: Vec<Wexpr>| Wexpr::Wapp(res) }
)));

// wish constants
named!(wconstant, alt_complete!(
    wnum => { |res: f64| Wexpr::Wnum(res) } |
    wstr => { |res: &[u8]| Wexpr::Wstring(res.to_owned()) }
));

// number parsing
named!(wnum<f64>, map_res!(
    map_res!(
        ws!(digit),
        str::from_utf8
    ),
    FromStr::from_str
));

// string literals
named!(wstr<&[u8]>, delimited!(tag!("\""), take_until!("\""), tag!("\"")));

// keywords and builtin fns
named!(wkeyword<Wbuiltin>, alt_complete!(
    tag!("+") => { |_| Wbuiltin::Add }
    | tag!("-") => { |_| Wbuiltin::Sub }
    | tag!("*") => { |_| Wbuiltin::Mul }
    | tag!("/") => { |_| Wbuiltin::Div }
    | tag!("cd") => { |_| Wbuiltin::Cd }
));

// Applied function
named!(wapp<Vec<Wexpr> >, many1!(wexpr));

pub fn interpret_wish(wish: &[u8]) {
    try_parse!(wish)
}
