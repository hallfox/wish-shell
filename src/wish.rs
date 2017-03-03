// AST types for wish

#[derive(Debug,PartialEq,Eq)]
pub enum Wbuiltin {
    Add,
    Sub,
    Mul,
    Div,
    Cd,
}

pub enum Wexpr {
    Wnum(f64), // store floats only now
    Wstring(String),
    Wkeyword(Wbuiltin),
    Wapp(Vec<Wexpr>),
}

pub struct Wish {
    job: String,
    args: Vec<Wexpr>,
}

impl Wish {
    pub fn new(job: String, args: Vec<Wexpr>) -> Self {
        Wish { job: job, args: args }
    }
}

