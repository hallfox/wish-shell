// AST types for wish

#[derive(Debug)]
pub enum WishVal {
    Num(i64),
    Symbol(String),
    Sexpr(Vec<WishVal>),
}

type WishResult = Result<WishVal, String>;

impl WishVal {
    pub fn eval(self) -> WishResult {
        match self {
            WishVal::Num(_) => Ok(self),
            // No environment lookup yet, one day maybe
            WishVal::Symbol(_) => Ok(self),
            WishVal::Sexpr(s) => {
                // Eval every child
                let mut body = Vec::with_capacity(s.len());
                for b in s {
                    body.push(b.eval()?);
                }

                // Now check how to eval
                match body.split_first() {
                    // Empty expr, maybe make one instance?
                    None => Ok(Vec::new()).map(WishVal::Sexpr),
                    Some((&WishVal::Symbol(ref func), args)) =>
                        builtin_op(func, args),
                    _ => Err("Operator is not a symbol".to_string())
                }
            }
        }
    }
}

fn builtin_op(func: &str, args: &[WishVal]) -> WishResult {
    match func {
        "+" => args.iter()
            .fold(Ok(0), |acc, v|
                  match (acc, v) {
                      (Ok(a), &WishVal::Num(x)) => Ok(a + x),
                      _ => Err("Arguments to operator + are not numbers".to_string()),
                  }
            )
            .map(WishVal::Num),
        "-" => args.iter()
            .fold(Ok(0), |acc, v|
                  match (acc, v) {
                      (Ok(a), &WishVal::Num(x)) => Ok(a - x),
                      _ => Err("Arguments to operator - are not numbers".to_string()),
                  }
            )
            .map(WishVal::Num),
        "*" => args.iter()
            .fold(Ok(1), |acc, v|
                  match (acc, v) {
                      (Ok(a), &WishVal::Num(x)) => Ok(a * x),
                      _ => Err("Arguments to operator * are not numbers".to_string()),
                  }
            )
            .map(WishVal::Num),
        "/" =>
            match args.split_first() {
                None => Err("Too few args passed to /".to_string()),
                Some((&WishVal::Num(f), rst)) =>
                    if rst.len() == 0 {
                        1i64.checked_div(f)
                            .map(WishVal::Num)
                            .ok_or("Division by 0.".to_string())
                    } else {
                        rst.iter()
                            .fold(Ok(f), |acc, v|
                                  match (acc, v) {
                                      (Ok(_), &WishVal::Num(0)) => Err("Division by 0".to_string()),
                                      (Ok(a), &WishVal::Num(x)) => Ok(a / x),
                                      _ => Err("Arguments to operator / are not numbers".to_string()),
                                  }
                            )
                            .map(WishVal::Num)
                    },
                _ => Err("Arguments to operator / are not numbers".to_string())
            },
        _ => Err("Invalid operator".to_string()),
    }
}
