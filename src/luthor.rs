//lets write a lexer for zod
//the first version is just a clone of 'Monkey' from the writing an interpreter in Go. 
//I'm doing it in rust and pretending that go is pseudo code. 
enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Equals
    Assignment,

}
enum Token {
    Let,
    Ident(String),
    Int(i32),
    Op(Operator)
    Illegal,
    EOF,





}

struct KeyWord {
    Data: String,
    LineNo: Option<u32>,

}

fn Happy() -> KeyWord  {
    let kw = KeyWord {
      Data: "Let",
      LineNo: None,
    }
}
