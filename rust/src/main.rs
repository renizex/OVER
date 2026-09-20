use std::io::{self, Write};
use std::collections::HashMap;
fn plus(a: f64, b: f64) -> f64 {
    a+b
}

fn minus(a: f64, b: f64) -> f64 {
    a-b
}

fn multiply(a: f64, b: f64) -> f64 {
    a*b
}

fn divide(a: f64, b: f64) -> Result<f64, InterpretError> {
    if b == 0.0 {
        Err(InterpretError::DivisionByZero(String::from("ERROR: division by zero.")))
    }
    else {
        Ok(a/b)
    }
}

fn unary_minus(a: f64) -> f64 {
    -a
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
    UnexpectedEndOfInput(String),
}

#[derive(Debug)]
enum LexError {
    UnknownSymbol(String),
    InvalidNumber(String),
}

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    Number(f64, usize, usize),
    Variable(String, usize, usize),
    OpenParenthesis(usize, usize),
    CloseParenthesis(usize, usize),
    Plus(usize, usize),
    Minus(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize),
    Assign(usize, usize),
}

impl Token {
    fn info(&self) -> (usize, usize) {
        match self {
            Token::Number(_, start, end) => (*start, *end),
            Token::Variable(_, start, end) => (*start, *end),
            Token::OpenParenthesis(start, end) => (*start, *end),
            Token::CloseParenthesis(start, end) => (*start, *end),
            Token::Plus(start, end) => (*start, *end),
            Token::Minus(start, end) => (*start, *end),
            Token::Multiply(start, end) => (*start, *end),
            Token::Divide(start, end) => (*start, *end),
            Token::Assign(start, end) => (*start, *end),
        }
    }

    fn value(&self) -> String {
        match self {
            Token::Number(value, _ ,_) => value.to_string(),
            Token::Variable(value, _, _) => value.clone(),
            Token::OpenParenthesis(_, _) => String::from("("),
            Token::CloseParenthesis(_, _) => String::from(")"),
            Token::Plus(_, _) => String::from("+"),
            Token::Minus(_, _) => String::from("-"),
            Token::Multiply(_, _) => String::from("*"),
            Token::Divide(_, _) => String::from("/"),
            Token::Assign(_, _) => String::from("="),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum Node {
    Number(f64, usize, usize),
    Variable(String, usize, usize),
    Binary(Box<Node>, char, Box<Node>),
    UnaryMinus(Box<Node>),
    Assignment(String, Box<Node>, usize, usize),
    Block(Vec<Node>, usize, usize),
}

impl Node {
    fn info(&self) -> (usize, usize) {
        match self {
            Node::Number(_, start, end) => (*start, *end),
            Node::Variable(_, start, end) => (*start, *end),
            Node::Binary(start_node, _, end_node) => {
                let (start, _) = start_node.info();
                let (_, end) = end_node.info();
                (start, end)
            },
            Node::UnaryMinus(node) => node.info(),
            Node::Assignment(_, _, start, end) => (*start, *end),
            Node::Block(_, start, end) => (*start, *end),
        }
    }

    fn value(&self) -> String {
        match self {
            Node::Number(value, _, _) => value.to_string(),
            Node::Variable(value, _, _) => value.clone(),
            Node::Binary(start_node, operator, end_node) => {let start = start_node.value(); let end = end_node.value(); start + &operator.to_string() + &end},
            Node::UnaryMinus(node) => "-".to_owned() + &node.value(),
            Node::Assignment(start, end, _, _) => start.to_owned() + "=" + &end.value(),
            Node::Block(nodes, _, _) => {
                let mut result = String::new();
                for node in nodes {
                    result.push_str(&(node.value() + " "))
                }
                result
            }
        }
    }
}

fn error(expression: String, start: usize, end: usize, msg: String) -> String {
    let pointer: String = " ".repeat(start) + &"^".repeat(end - start);
    format!("    ERROR: {msg}\n    {expression}    {pointer}")
}

fn main() {
    println!("OVER\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut expression: String = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        let tokens = match lex(expression.clone()) {
            Ok(tokens) => tokens,
            Err(LexError::UnknownSymbol(error)) => {println!("{error}\n"); continue}
            Err(LexError::InvalidNumber(error)) => {println!("{error}\n"); continue}
        };
        let node = match parse(tokens.clone(), expression.clone()) {
            Ok(node) => node,
            Err(ParseError::UnexpectedToken(error)) => {println!("{error}\n"); continue},
            Err(ParseError::UnexpectedEndOfInput(error)) => {println!("{error}\n"); continue},
        };
        let result = match interpret(&node.clone(), expression) {
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
    let mut characters = expression.chars().enumerate().peekable();
    while let Some((index, character)) = characters.next() {
        match character {
            '+' => tokens.push(Token::Plus(index, index + 1)),
            '-' => tokens.push(Token::Minus(index, index + 1)),
            '*' => tokens.push(Token::Multiply(index, index + 1)),
            '/' => tokens.push(Token::Divide(index, index + 1)),
            '(' => tokens.push(Token::OpenParenthesis(index, index + 1)),
            ')' => tokens.push(Token::CloseParenthesis(index, index + 1)),
            '=' => tokens.push(Token::Assign(index, index + 1)),
            character if character.is_whitespace() => continue,
            character if character.is_numeric() => {
                let mut number: String = String::new();
                number.push(character);
                while let Some(&(_, character)) = characters.peek() {
                    if character.is_numeric() || character == '.' {
                        number.push(character);
                        characters.next();
                    }
                    else {
                        break;
                    }
                }
                if number.chars().last().unwrap() == '.' {
                    return Err(LexError::InvalidNumber(error(expression, index, number.chars().count(), format!("unexpected end of number '{number}'. try '{number}0'."))));
                }
                let number: f64 = match number.parse() {
                    Ok(number) => number,
                    Err(_) => return Err(LexError::InvalidNumber(error(expression, index, number.chars().count(), format!("unexpected '.' in number '{number}'."))))
                };
                tokens.push(Token::Number(number, index, index + number.to_string().chars().count()))
            }
            character if character.is_alphanumeric() || character == '_' => {
                let mut variable: String = String::new();
                variable.push(character);
                while let Some(&(_, character)) = characters.peek() {
                    if character.is_alphanumeric() || character == '_' {
                        variable.push(character);
                        characters.next();
                    }
                    else {
                        break;
                    }
                }
                tokens.push(Token::Variable(variable.clone(), index, index + variable.chars().count()))
            }
            _ => return Err(LexError::UnknownSymbol(error(expression, index, index + 1, format!("'{character}' is invalid."))))
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    expression: String,
}

fn parse(tokens: Vec<Token>, expression: String) -> Result<Node, ParseError> {
    let mut nodes = Vec::<Node>::new();
    let mut parser = Parser {
        current_index: 0,
        tokens,
        expression,
    };
    while parser.current_index < parser.tokens.len() {
        nodes.push(parser.parse_assignment()?)
    }
    if nodes.is_empty() {
        return Err(ParseError::UnexpectedEndOfInput(String::from("ERROR: unexpected empty input.")))
    }
    let (start, end) = nodes.last().unwrap().info();
    Ok(Node::Block(nodes, start, end))
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
                Token::Plus(_, _) => '+',
                Token::Minus(_, _) => '-',
                Token::Multiply(_, _) => '*',
                Token::Divide(_, _) => '/',
                Token::Assign(_, _) => '=',
                Token::OpenParenthesis(_, _) => '(',
                Token::CloseParenthesis(_, _) => ')',
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
                Token::Plus(_, _) => '+',
                Token::Minus(_, _) => '-',
                Token::Multiply(_, _) => '*',
                Token::Divide(_, _) => '/',
                Token::Assign(_, _) => '=',
                Token::OpenParenthesis(_, _) => '(',
                Token::CloseParenthesis(_, _) => ')',
                Token::Number(value, start, end) => return Err(ParseError::UnexpectedToken(error(self.expression.clone(), *start, *end, format!("unexpected number '{value}'.")))),
                Token::Variable(value, start, end) => return Err(ParseError::UnexpectedToken(error(self.expression.clone(), *start, *end, format!("unexpected identifier '{value}'")))),
            };
            if expected.contains(&operator) {
                self.advance();
                Ok(operator)
            }
            else {
                match token {
                    Token::Number(value, start, end) => Err(ParseError::UnexpectedToken(error(self.expression.clone(), *start, *end, format!("expected '{:?}', got '{value}'.", expected)))),
                    Token::Variable(value, start, end) => Err(ParseError::UnexpectedToken(error(self.expression.clone(), *start, *end, format!("expected '{:?}', got '{value}'.", expected)))),
                    _ => {let value = token.value(); let (start, end) = token.info(); Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("expected '{:?}', got '{value}'.", expected))))},
                }
            }
        }
        else {
            Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), self.expression.chars().count() - 2, self.expression.chars().count() - 1, String::from("expected ')', got end of input."))))
        }
    }

    fn parse_assignment(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_expression()?;
        if let Some(_) = self.optional(&['=']) {
            self.advance();
            let right: Node = self.parse_expression()?;
            let (variable, start) = match left {
                Node::Variable(value, start, _) => Ok((value, start)),
                Node::Number(value, start, end) => Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("expected identifier, got '{value}'.")))),
                _ => {
                    let value: String = left.value();
                    let (start, end) = left.info();
                    Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("expected identifier, got '{value}'."))))
                },
            }?;
        let (_, end) = right.info();
        left = Node::Assignment(variable, Box::new(right), start, end);
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
        let mut left: Node = self.parse_unary()?;
        while let Some(operator) = self.optional(&['*', '/']) {
            self.advance();
            let right: Node = self.parse_unary()?;
            left = Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, ParseError> {
        if let Some(_) = self.optional(&['-']) {
            self.advance();
            let expression: Node = self.parse_unary()?;
            return Ok(Node::UnaryMinus(Box::new(expression)))
        }
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Result<Node, ParseError> {
        if let Some(token) = self.current() {
            match token {
                Token::Number(token, start, end) => {let node = Ok(Node::Number(*token, *start, *end)); self.advance(); node}
                Token::Variable(token, start, end) => {let node = Ok(Node::Variable(token.clone(), *start, *end)); self.advance(); node}
                Token::OpenParenthesis(_, _) => {
                    self.advance();
                    let expression = self.parse_expression()?;
                    //println!("{:?}", self.current());
                    self.consume(&[')'])?;
                    Ok(expression)
                }
                _ => {let (value, (start, end)) = (token.value(), token.info()); Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("unexpected '{value}'."))))},
            }
        }
        else {
            Err(ParseError::UnexpectedToken(error(self.expression.clone(), self.current_index + 1, self.current_index + 2, String::from("expected number, got nothing."))))
        }
    }
}

struct Interpret {
    variables: HashMap<String, f64>,
    expression: String,
}

impl Interpret {
    fn evaluate(&mut self, node: &Node) -> Result<f64, InterpretError> {
        // println!("{:?}", node);
        match &node {
            Node::Number(value, _, _) => Ok(*value),
            Node::Variable(value, _, _) => resolve_variable(self.variables.clone(), value.clone()),
            Node::Binary(left, operator, right) => {
                let left = self.evaluate(&**left)?;
                let right = self.evaluate(&**right)?;
                match operator {
                    '+' => Ok(plus(left, right)),
                    '-' => Ok(minus(left, right)),
                    '*' => Ok(multiply(left, right)),
                    '/' => Ok(divide(left, right)?),
                    _ => {let (value, (start, end)) = (&node.value(), &node.info()); Err(InterpretError::InvalidOperator(error(self.expression.clone(), *start, *end, format!("unknown operator '{value}'."))))}
                }
            }
            Node::Assignment(variable, expression, _, _) => {
                // println!("variable: {:?}\n expression: {:?}", variable, expression);
                let expression = self.evaluate(&**expression)?;
                self.variables.insert(variable.clone(), expression);
                Ok(expression)
            }
            Node::Block(vec, start, end) => {
                let mut result= Err(InterpretError::UnexpectedNode(error(self.expression.clone(), *start, *end, String::from("unexpected empty input."))));
                for node in vec {
                    result = self.evaluate(&*node);
                };
                result
            }
            Node::UnaryMinus(node) => {
                let number = self.evaluate(&**node)?;
                Ok(unary_minus(number))
            }
        }
    }
}

fn interpret(node: &Node, expression: String) -> Result<f64, InterpretError> {
    let variables = HashMap::<String, f64>::new();
    let mut interpret = Interpret{variables, expression};
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
