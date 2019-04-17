mod luthor;
use std::iter::Peekable;

fn main() {
    println!("hello world");

    let s = "Hello".to_string();
    let s: String = "aslo this".into();
    let sparkle_heart = vec![240, 159, 146, 150];

    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    println!("hearty: {}", sparkle_heart);

    let pangram: &'static str = "the quick brown fox jumped over the lazy dog.";
    println!("{}", pangram);

    println!("words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let astring = "let a = 1;";
    let vc = astring.chars().peekable();

    let mut chars = "226153980,17763347857".chars().peekable();
    let nums = parse_number(&mut chars);
    println!("we parsed: {}", nums);
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
