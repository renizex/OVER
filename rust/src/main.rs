use std::io::{self, Write};

#[derive(Debug)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(Operator),
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut expression: String = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        let lexed: Vec<Token> = lex(expression);
        println!("{:?}", lexed)
    }
}

fn lex(expression: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut characters = expression.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            character if character.is_whitespace() => continue,
            character if character.is_numeric() => {
                let mut raw: Vec<char> = Vec::new();
                let mut number: Vec<f64> = Vec::new();
                raw.push(character);
                while !characters.peek().is_none() {
                    if characters.peek().unwrap().is_numeric() {
                        raw.push(characters.next().unwrap());
                    }
                    else {
                        break;
                    }
                }
                for char in raw {
                    number.push(char.to_digit(10).unwrap() as f64);
                }
                let mut result: f64 = 0.0;
                for num in number {
                    result = result * 10.0 + num
                }
                tokens.push(Token::Number(result));
            }
            character if character.is_alphanumeric() => {
                let mut result: String = String::new();
                let mut chars: Vec<char> = Vec::new();
                chars.push(character);
                while !characters.peek().is_none() {
                    if characters.peek().unwrap().is_alphanumeric() {
                        chars.push(characters.next().unwrap());
                    } else {
                        break;
                    }
                }
                for char in &chars {
                    result += char.to_string().as_str();
                    }
                tokens.push(Token::Variable(result))
            }
            _ => println!("not implemented yet.")
        }
    }
    tokens
}