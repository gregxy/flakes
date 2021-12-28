#[derive(PartialEq)]
enum Token {
    Number(i64),
    ParenL,
    ParenR,
    Add,
    Sub,
    Mul,
    Div,
    End,
    Unknown,
}

fn scan(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    let mut c: char;

    loop {
        let maybe_c = chars.next();
        if maybe_c.is_none() {
            return Token::End;
        }

        c = maybe_c.unwrap();
        if c.is_ascii_whitespace() {
            continue;
        }
        break;
    }

    match c {
        '*' => return Token::Mul,
        '/' => return Token::Div,
        '+' => return Token::Add,
        '-' => return Token::Sub,
        '(' => return Token::ParenL,
        ')' => return Token::ParenR,
        _ => {
            if !c.is_ascii_digit() {
                return Token::Unknown;
            }

            let mut v: i64 = c.to_digit(10).unwrap() as i64;
            loop {
                let p = chars.peek();
                if p.is_none() || !p.unwrap().is_ascii_digit() {
                    return Token::Number(v);
                }

                v = v * 10 + (chars.next().unwrap().to_digit(10).unwrap() as i64);
            }
        }
    }
}

fn eval_secion(nums: &mut Vec<i64>, ops: &mut Vec<Token>) {
    loop {
        if ops.is_empty() {
            return;
        }

        let token = ops.pop().unwrap();
        if token == Token::ParenL {
            return;
        }

        let rhs = nums.pop().unwrap();
        let lhs = nums.pop().unwrap();
        match token {
            Token::Add => nums.push(lhs + rhs),
            Token::Sub => nums.push(lhs - rhs),
            Token::Mul => nums.push(lhs * rhs),
            Token::Div => nums.push(lhs / rhs),
            _ => (),
        }
    }
}

fn maybe_eval_once(nums: &mut Vec<i64>, ops: &mut Vec<Token>, next_op: &Token) {
    if ops.is_empty() {
        return;
    }

    let op = ops.pop().unwrap();
    if op == Token::ParenL {
        return;
    } 

    if (*next_op == Token::Mul || *next_op == Token::Div) &&
        (op == Token::Add || op == Token::Sub) {
        ops.push(op);
        return;
    }

    let rhs = nums.pop().unwrap();
    let lhs = nums.pop().unwrap();
    match op {
        Token::Add => nums.push(lhs + rhs),
        Token::Sub => nums.push(lhs - rhs),
        Token::Mul => nums.push(lhs * rhs),
        Token::Div => nums.push(lhs / rhs),
        _ => (),
    }
}

pub fn calc(src: &'static str) -> i64 {
    let mut chars = src.chars().peekable();
    let mut nums: Vec<i64> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    loop {
        let token = scan(&mut chars);

        match token {
            Token::Number(n) => nums.push(n),
            Token::ParenL => ops.push(token),
            Token::Add | Token::Sub | Token::Mul | Token::Div => {              
                maybe_eval_once(&mut nums, &mut ops, &token);
                ops.push(token);
            },
            Token::ParenR => eval_secion(&mut nums, &mut ops),
            Token::End => break,
            Token::Unknown => continue,
        }
    }

    eval_secion(&mut nums, &mut ops);

    return nums.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(calc("2*2"), 4);
        assert_eq!(calc("2+2"), 4);
        assert_eq!(calc("2 + 2*2 / 4 + 2"), 5);
        assert_eq!(calc("3 + 4 * 5"), 23);
        assert_eq!(calc("(3 + 4) * 5"), 35);
        assert_eq!(calc("3 - 4 + 5 * 2 + 6"), 15);
        assert_eq!(calc("3 - (4 + 5 * 2) + 6"), -5);
        assert_eq!(calc("3 - ((4 + 5) * 2) + 6"), -9);
        assert_eq!(calc("3 - 4 + 5 * (2 + 6)"), 39);
    }
}
