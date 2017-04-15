use nom::*;

use std::str;
use std::str::FromStr;

use wish::WishVal;

// Grammar rules:

// number : /-?[0-9]+/
// Add floating points in the future
named!(number <WishVal>,
       map_res!(
           map_res!(
               ws!(digit),
               str::from_utf8
           ),
           |n| {
               FromStr::from_str(n).map(WishVal::Num)
           }
       )
);

// symbol : /[a-zA-Z0-9_+\\-*\\/\\\\=<>!&]+/
static SYMBOL_CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                     abcdefghijklmnopqrstuvwxyz\
                                     0123456789\
                                     _-+*/\\=<>!&";
named!(symbol <WishVal>,
       map!(
           many1!(one_of!(SYMBOL_CHARS)),
           |res: Vec<_>| {
               WishVal::Symbol(res.into_iter().collect())
           }
       )
);

// sexpr : '(' <expr> ')'
named!(sexpr <WishVal>, map!(wexpr, WishVal::Sexpr));

// primitive, will be used for both lists and sexprs
named!(wexpr <Vec<WishVal>>, ws!(
    delimited!(
        tag!("("),
        fold_many0!(ws!(expr), Vec::new(), |mut acc: Vec<_>, wval| {
            acc.push(wval);
            acc
        }),
        tag!(")")
    )
));

// expr : <number> | <symbol> | <sexpr>
named!(expr <WishVal>,
       alt_complete!(number | symbol | sexpr)
);

// wish : /^/ <expr>* /$/
named!(pub wish <WishVal>, map!(many0!(expr), WishVal::Sexpr));
