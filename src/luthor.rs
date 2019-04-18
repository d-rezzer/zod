//lets write a lexer for zod
//the first version is just a clone of 'Monkey' from the writing an interpreter in Go.
//I'm doing it in rust and pretending that go is pseudo code.
use std::error::Error;
use std::iter::Peekable;

struct TokenInfo {
    Type: String,
    Data: String,
    LineNo: Option<u32>,
    Column: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Plus,
    Minus,
    Mult,
    Div,
    Equals,
    Assignment,
    Identifier(String),
    Int(i32),
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Illegal,
    EOF,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        let tok = match c {
            '=' => Token::Assignment,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            'a'...'z' | 'A'...'Z' | '_' => {
                //is we a letter? maybe keyword, maybe identifier.
                //an identifier or a keyword.
                Token::Identifier(String::from("a_var"))
            }
            _ => Token::EOF,
        };

        tok
    }
}

#[derive(Debug, PartialEq)]
struct Lexer {
    input: String,
    position: usize,
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
            self.position = self.read_position;
            self.read_position = self.read_position + 1;
        }
    }

    fn next_token(&mut self) -> Token {
        //let char = String::from_utf8(vec![self.ch.unwrap()]).unwrap();
        //let chref = char.as_ref();
        let curr_char = self.ch.unwrap() as char;

        let tok = match curr_char {
            '=' => Token::Assignment,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            'a'...'z' | 'A'...'Z' | '_' => {
                //is we a letter? maybe keyword, maybe identifier.
                //an identifier or a keyword.
                Token::Identifier(String::from("a_var"))
            }
            _ => Token::EOF,
        };

        self.read_char();

        tok
    }

    //maybe an iterator over chars is a better idea.
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
    fn next_token() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x,y) {
            x +y;    
        };
       
        let result = add(five,ten);
        ";

        let mut lex = Lexer::new(input);
        lex.read_char();
        let r = lex.ch.unwrap();

        assert_eq!('l', r as char);
    }

}
