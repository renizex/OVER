use std::io::{self, Write};
use std::collections::HashMap;
fn plus(a: f64, b: f64) -> Result<f64, InterpretError> {
    Ok(a+b)
}

fn minus(a: f64, b: f64) -> Result<f64, InterpretError> {
    Ok(a-b)
}

fn multiply(a: f64, b: f64) -> Result<f64, InterpretError> {
    Ok(a*b)
}

fn divide(a: f64, b: f64) -> Result<f64, InterpretError> {
    if b == 0.0 {
        Err(InterpretError::DivisionByZero(String::from("ERROR: division by zero.")))
    }
    else {
        Ok(a/b)
    }
}
#[derive(Debug)]
enum InterpretError {
    UnexpectedNode(String),
    InvalidVariable(String),
    InvalidOperator(String),
    DivisionByZero(String),
}

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(String),
}

#[derive(Debug)]
enum LexError {
    UnknownSymbol(String),
    InvalidNumber(String),
}

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    Number(f64),
    Variable(String),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
}

#[derive(Clone)]
#[derive(Debug)]
enum Node {
    Number(f64),
    Variable(String),
    Binary(Box<Node>, char, Box<Node>),
    Assignment(String, Box<Node>),
    Block(Vec<Node>),
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
            Err(LexError::UnknownSymbol(error)) => {println!("{error}\n"); continue}
            Err(LexError::InvalidNumber(error)) => {println!("{error}\n"); continue}
        };
        let node = match parse(tokens.clone()) {
            Ok(node) => node,
            Err(ParseError::UnexpectedToken(error)) => {println!("{error}\n"); continue}
        };
        let result = match interpret(node.clone()) {
            Ok(result) => result,
            Err(InterpretError::DivisionByZero(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::InvalidOperator(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::InvalidVariable(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::UnexpectedNode(error)) => {println!("{error}\n"); continue},
        };
        println!("tokens: {tokens:?}");
        println!("ast: {node:?}");
        println!("result: {result}\n")
    }
}

fn lex(expression: String) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut characters = expression.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),
            '=' => tokens.push(Token::Assign),
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
                    return Err(LexError::InvalidNumber(format!("ERROR: unexpected end of number '{number}'.\ntry '{number}0'.")));
                }
                let number: f64 = match number.parse() {
                    Ok(number) => number,
                    Err(_) => return Err(LexError::InvalidNumber(format!("ERROR: unexpected '.' in number '{number}'.")))
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
            _ => return Err(LexError::UnknownSymbol(format!("ERROR: '{character}' is invalid.")))
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

fn parse(tokens: Vec<Token>) -> Result<Node, ParseError> {
    let mut nodes = Vec::<Node>::new();
    let mut parser = Parser {
        current_index: 0,
        tokens,
    };
    while parser.current_index < parser.tokens.len() {
        nodes.push(parser.parse_assignment()?)
    }
    Ok(Node::Block(nodes))
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
                Token::Assign => '=',
                _ => return None
            };
            if expected.contains(&operator) {
                return Some(operator)
            }
        }
        None
    }

    fn consume(&mut self, expected: &[char]) -> Result<char, ParseError> {
        if let Some(token) = self.current() {
            let operator: char = match token {
                Token::Plus => '+',
                Token::Minus => '-',
                Token::Multiply => '*',
                Token::Divide => '/',
                Token::Assign => '=',
                    _ => return Err(ParseError::UnexpectedToken(format!("ERROR: unknown token '{:?}'.", token)))
            };
            if expected.contains(&operator) {
                self.advance();
                Ok(operator)
            }
            else {
                Err(ParseError::UnexpectedToken(format!("ERROR: expected '{:?}', got '{:?}'.", expected, token)))
            }
        }
        else {
            Err(ParseError::UnexpectedToken(String::from("ERROR: unexpected 'none' type.")))
        }
    }

    fn parse_assignment(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_expression()?;
        if let Some(_) = self.optional(&['=']) {
            self.advance();
            let right: Node = self.parse_expression()?;
            let variable = match left {
                Node::Variable(value) => Ok(value),
                _ => Err(ParseError::UnexpectedToken(format!("ERROR: expected identifier, got '{:?}'.", left))),
            };
            left = Node::Assignment(variable?, Box::new(right));
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_term()?;
        while let Some(operator) = self.optional(&['+', '-']) {
            self.advance();
            let right: Node = self.parse_term()?;
            left =  Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_factor()?;
        while let Some(operator) = self.optional(&['*', '/']) {
            self.advance();
            let right: Node = self.parse_factor()?;
            left = Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Node, ParseError> {
        if let Some(token) = self.current() {
            match token {
                Token::Number(token) => {let node = Ok(Node::Number(*token)); self.advance(); node}
                Token::Variable(token) => {let node = Ok(Node::Variable(token.clone())); self.advance(); node}
                Token::OpenParenthesis => {
                    self.advance();
                    let expression = self.parse_expression();
                    let _ = self.consume(&[')']);
                    expression
                }
                _ => Err(ParseError::UnexpectedToken(format!("ERROR: unexpected token '{:?}'.", token)))
            }
        }
        else {
            Err(ParseError::UnexpectedToken(String::from("ERROR: unexpected nothing.")))
        }
    }
}

struct Interpret {
    variables: HashMap<String, f64>
}

impl Interpret {
    fn evaluate(&mut self, node: Node) -> Result<f64, InterpretError> {
        // println!("{:?}", node);
        match node {
            Node::Number(value) => Ok(value),
            Node::Variable(value) => resolve_variable(self.variables.clone(), value),
            Node::Binary(left, operator, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match operator {
                    '+' => Ok(plus(left, right)?),
                    '-' => Ok(minus(left, right)?),
                    '*' => Ok(multiply(left, right)?),
                    '/' => Ok(divide(left, right)?),
                    _ => Err(InterpretError::InvalidOperator(format!("unknown operator '{operator}'.")))
                }
            }
            Node::Assignment(variable, expression) => {
                // println!("variable: {:?}\n expression: {:?}", variable, expression);
                let expression = self.evaluate(*expression)?;
                self.variables.insert(variable, expression);
                Ok(expression)
            }
            Node::Block(vec) => {
                let mut result= Err(InterpretError::UnexpectedNode(String::from("ERROR: unexpected empty input.")));
                for node in vec {
                    result = self.evaluate(node);
                };
                result
            }
        }
    }
}

fn interpret(node: Node) -> Result<f64, InterpretError> {
    let variables = HashMap::<String, f64>::new();
    let mut interpret = Interpret{variables};
    interpret.evaluate(node)
}

fn resolve_variable(variables: HashMap<String, f64>, variable: String) -> Result<f64, InterpretError> {
    if let Some(value) = variables.get(&variable) {
        Ok(*value)
    }
    else {
        Err(InterpretError::InvalidVariable(format!("ERROR: variable '{variable}' does not exist.\nif stuck, try to input in one line.\nexample: x = 5 y = x x * y")))
    }
}
