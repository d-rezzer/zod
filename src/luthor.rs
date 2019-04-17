//lets write a lexer for zod
//the first version is just a clone of 'Monkey' from the writing an interpreter in Go.
//I'm doing it in rust and pretending that go is pseudo code.
use std::error::Error;
use std::iter::Peekable;
#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Equals,
    Assignment,
    Nop,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        let t = match s {
            "=" => Operator::Assignment,
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Mult,
            "/" => Operator::Div,
            "==" => Operator::Equals,
            _ => Operator::Nop, //i don't like this.
        };

        t
    }
}
#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Identifier(String),
    Int(i32),
    Op(Operator),
    Terminator,
    LParen,
    RParen,
    Comma,
    LBrace,
    RBrace,
    Illegal,

    EOF,
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token::Identifier(s)
    }
}

struct Lexer {
    input: String,
    position: i32,
    read_position: usize,
    ch: Option<u8>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: None,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.as_bytes()[self.read_position]);
        }
    }

    //maybe an iterator over chars is a better idea.
}

struct TokenInfo {
    Data: String,
    LineNo: Option<u32>,
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    //let mut n = 0;
    let mut tok = String::new();
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                tok.push_str(&r.to_string());
                //n = n + r;
                //n = n * 10 + r.to_digit(10).unwrap();
                //println!("peeking: {}", n);
            }
            _ => return tok,
        }

        tokens.next();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn from_string_ident() {
        let id = String::from("my_var");

        let tk = Token::from(id);

        assert_eq!(Token::Identifier(String::from("my_var")), tk);
    }

    #[test]
    fn from_string_operator() {
        let tk = Token::Op(Operator::from("="));

        assert_eq!(Token::Op(Operator::from("=")), tk);
    }

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let mut lex = Lexer::new(input);
        let c1 = lex.read_char();
        assert_eq!(1, 1);
    }

    #[test]
    fn check_operator() {
        let ass = Operator::Assignment;
        let pl = Operator::Plus;
        let tok = Token::Op(ass);

        let x = match tok {
            Token::Op(t) => {
                println!("I have a token : {:?}", t);
                t
            }

            _ => {
                println!("I have nothing");
                Operator::Nop
            }
        };
    }

}
