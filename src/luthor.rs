//lets write a lexer for zod
//the first version is just a clone of 'Monkey' from the writing an interpreter in Go.
//I'm doing it in rust and pretending that go is pseudo code.
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

impl From<String> for Operator {
    fn from(s: String) -> Self {
        let ass = String::from("=");
        let eq = String::from("==");
        let pl = String::from("+");
        let min = String::from("-");
        let mult = String::from("*");
        let div = String::from("/");

        let t = match s {
            ass => Operator::Assignment,
            pl => Operator::Plus,
            min => Operator::Minus,
            mult => Operator::Mult,
            div => Operator::Div,
            eq => Operator::Equals,
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

struct TokenInfo {
    Data: String,
    LineNo: Option<u32>,
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
        let op = String::from("=");
        let tk = Token::Op(Operator::from(op));

        assert_eq!(Token::Op(Operator::from(String::from("="))), tk);
    }

    #[test]
    fn next_token() {
        let input = "=+(){},;";
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
