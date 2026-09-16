use std::io::{self, Write};

#[derive(Debug)]
enum LexerError {
    UnknownSymbol(String),
    InvalidNumber(String),
}

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
    Multiply,
    Divide,
}

fn main() {
    println!("OVER\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut expression: String = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        match lex(expression) {
            Ok(tokens) => println!("{:?}\n", tokens),
            Err(LexerError::UnknownSymbol(error)) => println!("{error}\n"),
            Err(LexerError::InvalidNumber(error)) => println!("{error}\n"),
        }
    }
}

fn lex(expression: String) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut characters = expression.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            '*' => tokens.push(Token::Operator(Operator::Multiply)),
            '/' => tokens.push(Token::Operator(Operator::Divide)),
            character if character.is_whitespace() => continue,
            character if character.is_numeric() => {
                let mut number: String = String::new();
                number.push(character);
                while !characters.peek().is_none() {
                    if characters.peek().unwrap().is_numeric() || *characters.peek().unwrap() == '.' {
                        number.push(characters.next().unwrap())
                    }
                    else {
                        break;
                    }
                }
                if number.chars().last().unwrap() == '.' {
                    return Err(LexerError::InvalidNumber(format!("ERROR: unexpected end of number '{number}'.\ntry '{number}0'.")));
                }
                let number: f64 = match number.parse() {
                    Ok(number) => number,
                    Err(_) => return Err(LexerError::InvalidNumber(format!("ERROR: unexpected '.' in number '{number}'.")))
                };
                tokens.push(Token::Number(number))
            }
            character if character.is_alphanumeric() || character == '_' => {
                let mut variable: String = String::new();
                variable.push(character);
                while !characters.peek().is_none() {
                    if characters.peek().unwrap().is_alphanumeric() || *characters.peek().unwrap() == '_' {
                        variable.push(characters.next().unwrap())
                    }
                    else {
                        break;
                    }
                }
                tokens.push(Token::Variable(variable))
            }
            _ => return Err(LexerError::UnknownSymbol(format!("ERROR: '{character}' is invalid.")))
        }
    }
    Ok(tokens)
}
