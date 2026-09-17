use std::io::{self, Write};


#[derive(Debug)]
enum ParserError {
    IndexOutOfRange(String),
    Shit(String),
}

#[derive(Debug)]
enum LexerError {
    UnknownSymbol(String),
    InvalidNumber(String),
}

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone)]
#[derive(Debug)]
enum Node {
    Number(f64),
    Variable(String),
    Binary(Box<Node>, char, Box<Node>)
}

fn main() {
    println!("OVER\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut expression: String = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        let tokens = match lex(expression) {
            Ok(tokens) => tokens,
            Err(LexerError::UnknownSymbol(error)) => {println!("{error}\n"); continue}
            Err(LexerError::InvalidNumber(error)) => {println!("{error}\n"); continue}
        };
        let node = match parse(tokens.clone()) {
            Ok(node) => node,
            Err(ParserError::IndexOutOfRange(error)) => {println!("{error}\n"); continue}
            Err(ParserError::Shit(error)) => {println!("{error}\n"); continue}
        };
        println!("tokens: {tokens:?}");
        println!("ast: {node:?}");
    }
}

fn lex(expression: String) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut characters = expression.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            character if character.is_whitespace() => continue,
            character if character.is_numeric() => {
                let mut number: String = String::new();
                number.push(character);
                while let Some(&character) = characters.peek() {
                    if character.is_numeric() || character == '.' {
                        number.push(character);
                        characters.next();
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
                while let Some(&character) = characters.peek() {
                    if character.is_alphanumeric() || character == '_' {
                        variable.push(character);
                        characters.next();
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

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,

}

fn parse(tokens: Vec<Token>) -> Result<Node, ParserError> {
    let mut parser = Parser {
        current_index: 0,
        tokens
    };
    parser.parse_expression()
}

impl Parser {
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }

    fn advance(&mut self) {
        self.current_index += 1
    }

    fn optional(&mut self, expected: &[char]) -> Option<char> {
        if let Some(token) = self.current() {
            let operator: char = match token {
                Token::Plus => '+',
                Token::Minus => '-',
                Token::Multiply => '*',
                Token::Divide => '/',
                _ => return None
            };
            if expected.contains(&operator) {
                return Some(operator)
            }
        }
        None
    }

    fn consume(&mut self, expected: &[char]) -> Result<char, ParserError> {
        if let Some(token) = self.current() {
            let operator: char = match token {
                Token::Plus => '+',
                Token::Minus => '-',
                Token::Multiply => '*',
                Token::Divide => '/',
                _ => return Err(ParserError::Shit(String::from("долбаеб тут нужен оператор")))
            };
            if expected.contains(&operator) {
                self.advance();
                Ok(operator)
            }
            else {
                Err(ParserError::Shit(String::from("сожрать не вышло")))
            }
        }
        else {
            Err(ParserError::Shit(String::from("что ты за хуйню мне подкинул")))
        }
    }

    fn parse_expression(&mut self) -> Result<Node, ParserError> {
        let mut left: Node = self.parse_term()?;
        while let Some(operator) = self.optional(&['+', '-']) {
            self.advance();
            let right: Node = self.parse_term()?;
            left =  Node::Binary(Box::new(left.clone()), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Node, ParserError> {
        let mut left: Node = self.parse_factor()?;
        while let Some(operator) = self.optional(&['*', '/']) {
            self.advance();
            let right: Node = self.parse_factor()?;
            left = Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Node, ParserError> {
        if let Some(token) = self.current() {
            match token {
                Token::Number(token) => {let node = Ok(Node::Number(*token)); self.advance(); node}
                Token::Variable(token) => Ok(Node::Variable(token.clone())),
                _ => Err(ParserError::Shit(format!("ERROR: unexpected token '{:?}'", token)))
            }
        }
        else {
            Err(ParserError::Shit(String::from("ти обисрався")))
        }
    }
}
