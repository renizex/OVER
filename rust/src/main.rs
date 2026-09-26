use std::io::{self, Write};
use std::fmt::Write as FmtWrite;
use std::collections::HashMap;

fn plus(a: Value, b: Value) -> Result<Value, InterpretError> {
    match (&a, &b) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::String(a), Value::String(b)) => Ok(Value::String(a.to_owned() + b)),
        _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
    }
}

fn minus(a: f64, b: f64) -> f64 {
    a-b
}

fn multiply(a: f64, b: f64) -> f64 {
    a*b
}

fn divide(a: f64, b: f64) -> Result<f64, InterpretError> {
    if b == 0.0 {
        return Err(InterpretError::DivisionByZero(String::from("ERROR: division by zero.")))
    };
    Ok(a/b)
}

fn unary_minus(a: f64) -> f64 {
    -a
}

fn greater(a: f64, b: f64) -> bool {
    a > b
}

fn less(a: f64, b: f64) -> bool {
    a < b
}

fn equal(a: Value, b: Value) -> Result<bool, InterpretError> {
    match (&a, &b) {
        (Value::Number(a), Value::Number(b)) => Ok(a == b),
        (Value::String(a), Value::String(b)) => Ok(a == b),
        (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
        _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
    }
}

fn greaterequal(a: f64, b: f64) -> bool {
    a >= b
}

fn lessequal(a: f64, b: f64) -> bool {
    a <= b
}

#[derive(Clone)]
#[derive(Debug)]
enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nothing,
    Break,
    Continue,
}

impl Value {
    fn expect_number(&self) -> Result<f64, InterpretError> {
        match self {
            Value::Number(value) => Ok(*value),
            _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected number, got {}.", self.type_name()))),
        }
    }

    fn expect_bool(&self) -> Result<bool, InterpretError> {
        match self {
            Value::Bool(value) => Ok(*value),
            _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected bool, got {}.", self.type_name()))),
        }
    }

    fn option_break(&self) -> bool {
        match self {
            Value::Break => true,
            _ => false
        }
    }

    fn option_continue(&self) -> bool {
        match self {
            Value::Continue => true,
            _ => false
        }
    }

    fn display(&self) -> String {
        match self {
            Value::Number(value) => value.to_string(),
            Value::Bool(value) => value.to_string(),
            Value::String(value) => value.to_string(),
            Value::Nothing => String::from("null(scape)"),
            Value::Break => String::from("break"),
            Value::Continue => String::from("continue"),
        }
    }

    fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Nothing => "nothing",
            Value::Break => "break",
            Value::Continue => "continue",
        }
    }
}

#[derive(Debug)]
enum InterpretError {
    // UnexpectedNode(String),
    InvalidVariable(String),
    InvalidOperator(String),
    DivisionByZero(String),
    UnexpectedValue(String),
    ExecutionLimit(String),
    InvalidType(String),
}

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput(String),
}

#[derive(Debug)]
enum LexError {
    UnexpectedSymbol(String),
    UnknownSymbol(String),
    InvalidNumber(String),
    InvalidString(String),
}

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    Number(f64, usize, usize),
    Identifier(String, usize, usize),
    String(String, usize, usize),
    Bool(bool, usize, usize),
    OpenParenthesis(usize, usize),
    CloseParenthesis(usize, usize),
    OpenBrace(usize, usize),
    CloseBrace(usize, usize),
    Plus(usize, usize),
    Minus(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize),
    Assign(usize, usize),
    Greater(usize, usize),
    Less(usize, usize),
    Equal(usize,usize),
    GreaterEqual(usize, usize),
    LessEqual(usize, usize),
    NotEqual(usize, usize),
    If(usize, usize),
    Else(usize, usize),
    While(usize, usize),
    Loop(usize, usize),
    Break(usize, usize),
    Continue(usize, usize),
}

impl Token {
    fn info(&self) -> (usize, usize) {
        match self {
            Token::Number(_, start, end) => (*start, *end),
            Token::Identifier(_, start, end) => (*start, *end),
            Token::String(_, start, end) => (*start, *end),
            Token::Bool(_, start, end) => (*start, *end),
            Token::OpenParenthesis(start, end) => (*start, *end),
            Token::CloseParenthesis(start, end) => (*start, *end),
            Token::OpenBrace(start, end) => (*start, *end),
            Token::CloseBrace(start, end) => (*start, *end),
            Token::Plus(start, end) => (*start, *end),
            Token::Minus(start, end) => (*start, *end),
            Token::Multiply(start, end) => (*start, *end),
            Token::Divide(start, end) => (*start, *end),
            Token::Assign(start, end) => (*start, *end),
            Token::Greater(start, end) => (*start, *end),
            Token::Less(start, end) => (*start, *end),
            Token::Equal(start, end) => (*start, *end),
            Token::GreaterEqual(start, end) => (*start, *end),
            Token::LessEqual(start, end) => (*start, *end),
            Token::NotEqual(start, end) => (*start, *end),
            Token::If(start, end) => (*start, *end),
            Token::Else(start, end) => (*start, *end),
            Token::While(start, end) => (*start, *end),
            Token::Loop(start, end) => (*start, *end),
            Token::Break(start, end) => (*start, *end),
            Token::Continue(start, end) => (*start, *end),
        }
    }

    fn value(&self) -> String {
        match self {
            Token::Number(value, _ ,_) => value.to_string(),
            Token::Identifier(value, _, _) => value.clone(),
            Token::String(value, _, _) => value.clone(),
            Token::Bool(value, _, _) => value.to_string(),
            Token::OpenParenthesis(_, _) => String::from("("),
            Token::CloseParenthesis(_, _) => String::from(")"),
            Token::OpenBrace(_, _) => String::from("{"),
            Token::CloseBrace(_, _) => String::from("}"),
            Token::Plus(_, _) => String::from("+"),
            Token::Minus(_, _) => String::from("-"),
            Token::Multiply(_, _) => String::from("*"),
            Token::Divide(_, _) => String::from("/"),
            Token::Assign(_, _) => String::from("="),
            Token::Greater(_, _) => String::from(">"),
            Token::Less(_, _) => String::from("<"),
            Token::Equal(_, _) => String::from("=="),
            Token::GreaterEqual(_, _) => String::from(">="),
            Token::LessEqual(_, _) => String::from("<="),
            Token::NotEqual(_, _) => String::from("!="),
            Token::If(_, _) => String::from("if"),
            Token::Else(_, _) => String::from("else"),
            Token::While(_, _) => String::from("while"),
            Token::Loop(_, _) => String::from("loop"),
            Token::Break(_, _) => String::from("break"),
            Token::Continue(_, _) => String::from("continue"),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum Node {
    Number(f64, usize, usize),
    Identifier(String, usize, usize),
    String(String, usize, usize),
    Bool(bool, usize, usize),
    Binary(Box<Node>, Token, Box<Node>),
    UnaryMinus(Box<Node>),
    Assignment(String, Box<Node>, usize, usize),
    If(Box<Node>, Box<Node>, Option<Box<Node>>, usize, usize),
    While(Box<Node>, Box<Node>, Option<Box<Node>>, usize, usize),
    Loop(Box<Node>, usize, usize),
    Break(usize, usize),
    Block(Vec<Node>, usize, usize),
    Continue(usize, usize),
}

impl Node {
    fn info(&self) -> (usize, usize) {
        match self {
            Node::Number(_, start, end) => (*start, *end),
            Node::Identifier(_, start, end) => (*start, *end),
            Node::String(_, start, end) => (*start, *end),
            Node::Bool(_, start, end) => (*start, *end),
            Node::Binary(start_node, _, end_node) => {
                let (start, _) = start_node.info();
                let (_, end) = end_node.info();
                (start, end)
            },
            Node::UnaryMinus(node) => node.info(),
            Node::Assignment(_, _, start, end) => (*start, *end),
            Node::If(_, _, _, start, end) => (*start, *end),
            Node::While(_, _, _, start, end) => (*start, *end),
            Node::Loop(_,  start, end) => (*start, *end),
            Node::Break(start, end) => (*start, *end),
            Node::Block(_, start, end) => (*start, *end),
            Node::Continue(start, end) => (*start, *end),
        }
    }

    fn value(&self) -> String {
        match self {
            Node::Number(value, _, _) => value.to_string(),
            Node::Identifier(value, _, _) => value.clone(),
            Node::String(value, _, _) => value.clone(),
            Node::Bool(value, _, _) => value.to_string(),
            Node::Binary(start_node, operator, end_node) => {let start = start_node.value(); let end = end_node.value(); start + &operator.value() + &end},
            Node::UnaryMinus(node) => "-".to_owned() + &node.value(),
            Node::Assignment(start, end, _, _) => start.to_owned() + "=" + &end.value(),
            Node::If(condition, body, _, _, _) => {"if ".to_owned() + &condition.value() + " {\n" + &body.value() + "\n" + "}"}
            Node::While(condition, body, _, _, _) => {"while ".to_owned() + &condition.value() + " {\n" + &body.value() + "\n" + "}"}
            Node::Loop(body, _, _) => {"loop ".to_owned() + " {\n" + &body.value() + "\n" + "}"}
            Node::Break(_, _) => String::from("break"),
            Node::Continue(_, _) => String::from("continue"),
            Node::Block(nodes, _, _) => {
                let mut result = String::new();
                result.push('{');
                    for node in nodes {
                        result.push_str(&(node.value() + " "))
                    }
                result.push('}');
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
    let mut debug: bool = false;
    println!("OVER\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut expression: String = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        if expression.trim() == "/debug" {
            if !debug {
                println!("debug mode ON\n");
                debug = true;
                continue;
            }
            println!("debug mode OFF\n");
            debug = false;
            continue;
        }
        let tokens = match lex(expression.clone()) {
            Ok(tokens) => tokens,
            Err(LexError::UnknownSymbol(error)) => {println!("{error}\n"); continue}
            Err(LexError::InvalidNumber(error)) => {println!("{error}\n"); continue}
            Err(LexError::UnexpectedSymbol(error)) => {println!("{error}\n"); continue}
            Err(LexError::InvalidString(error)) => {println!("{error}\n"); continue}
        };
        let node = match parse(tokens.clone(), expression.clone()) {
            Ok(node) => node,
            Err(ParseError::UnexpectedToken(error)) => {println!("{error}\n"); continue},
            Err(ParseError::UnexpectedEndOfInput(error)) => {println!("{error}\n"); continue},
        };
        let result = match interpret(&node, expression) {
            Ok(result) => result,
            Err(InterpretError::DivisionByZero(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::InvalidOperator(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::InvalidVariable(error)) => {println!("{error}\n"); continue},
            // Err(InterpretError::UnexpectedNode(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::UnexpectedValue(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::ExecutionLimit(error)) => {println!("{error}\n"); continue},
            Err(InterpretError::InvalidType(error)) => {println!("{error}\n"); continue},
        };
        if debug {
            println!("\ntokens: {tokens:?}");
            println!("ast: {node:?}");
            println!("result: {}\n", result.display());
            continue;
        }
        println!("{}\n", result.display());
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
            '{' => tokens.push(Token::OpenBrace(index, index + 1)),
            '}' => tokens.push(Token::CloseBrace(index, index + 1)),
            '=' => {
                if let Some(&(_, character)) = characters.peek() {
                    if character == '=' {
                        characters.next();
                        tokens.push(Token::Equal(index, index + 2));
                        continue;
                    }
                    tokens.push(Token::Assign(index, index + 1))
                }
            },
            '>' => {
                if let Some(&(_, character)) = characters.peek() {
                    if character == '=' {
                        characters.next();
                        tokens.push(Token::GreaterEqual(index, index + 2));
                        continue
                    }
                    tokens.push(Token::Greater(index, index + 1));
                }
            },
            '<' => {
                if let Some(&(_, character)) = characters.peek() {
                    if character == '=' {
                        characters.next();
                        tokens.push(Token::LessEqual(index, index + 2));
                        continue
                    }
                    tokens.push(Token::Less(index, index + 1));
                }
            }
            '!' => {
                if let Some(&(_, character)) = characters.peek() {
                    if character == '=' {
                        characters.next();
                        tokens.push(Token::NotEqual(index, index + 2));
                        continue;
                    }
                    return Err(LexError::UnexpectedSymbol(error(expression, index, index  + 1, String::from("unexpected '!'."))))
                }
            }
            character if character.is_whitespace() => continue,
            character if character.is_numeric() => {
                let mut number: String = String::new();
                number.push(character);
                while let Some(&(_, character)) = characters.peek() {
                    if character.is_ascii_digit() || character == '.' {
                        number.push(character);
                        characters.next();
                    }
                    else {
                        break;
                    }
                }
                if number.chars().last().unwrap() == '.' {
                    return Err(LexError::InvalidNumber(error(expression, index, index + number.len(), format!("unexpected end of number '{number}'."))));
                }
                let num: f64 = match number.parse() {
                    Ok(number) => number,
                    Err(_) => return Err(LexError::InvalidNumber(error(expression, index, index + number.len(), format!("unexpected '.' in number '{number}'."))))
                };
                tokens.push(Token::Number(num, index, index + number.len()))
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
                let length: usize = index + variable.chars().count();
                match variable.as_str() {
                    "if" => tokens.push(Token::If(index, index + 2)),
                    "else" => tokens.push(Token::Else(index, index + 4)),
                    "while" => tokens.push(Token::While(index, index + 5)),
                    "loop" => tokens.push(Token::Loop(index, index + 4)),
                    "break" => tokens.push(Token::Break(index, index + 5)),
                    "true" => tokens.push(Token::Bool(true, index, index + 4)),
                    "false" => tokens.push(Token::Bool(false, index, index + 5)),
                    "continue" => tokens.push(Token::Continue(index, index + 8)),
                    _ => tokens.push(Token::Identifier(variable, index, length))
                }
            }
            character if character == '"' => {
                let mut string = String::new();
                string.push(character);
                if let Some((_, character)) = characters.next() {
                    string.push(character)
                }
                while let Some(&(_, character)) = characters.peek() {
                    if character.is_numeric() || character.is_alphanumeric() || character == '_' || character == '"' {
                        string.push(character);
                        characters.next();
                    }
                    else {
                        break
                    }
                }
                let (start, end) = (index, index + string.chars().count());
                if string.chars().last() == Some('"') && string.chars().next() == Some('"') {
                    string.pop();
                    string.remove(0);
                }
                else {
                    println!("{string}");
                    return Err(LexError::InvalidString(error(expression, start, end, String::from("invalid string."))))
                }
                tokens.push(Token::String(string, start, end))
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
        nodes.push(parser.parse_statement()?)
    }
    if nodes.is_empty() {
        return Err(ParseError::UnexpectedEndOfInput(String::from("ERROR: empty input.")))
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

    fn optional(&mut self, expected: &[&str]) -> Option<Token> {
        if let Some(token) = self.current() {
            if expected.contains(&&*token.value()) {
                return Some(token.clone())
            }
        }
        None
    }

    fn consume(&mut self, expected: &[&str]) -> Result<Token, ParseError> {
        let mut display: String = String::new();
        for char in expected {
            write!(display, "'{char}', ").unwrap();
        }
        if let Some(token) = self.current().cloned() {
            let value: String = match token {
                Token::Number(value, start, end) => return Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("unexpected number '{value}'.")))),
                Token::Identifier(value, start, end) => return Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("unexpected identifier '{value}'")))),
                _ => token.value()
            };
            if expected.contains(&&*value) {
                self.advance();
                Ok(token)
            }
            else {
                let value = token.value();
                let (start, end) = token.info();
                Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("expected {display}got '{value}'."))))
            }
        }
        else {
            Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), self.expression.chars().count() - 2, self.expression.chars().count() - 1, format!("expected {display}got end of input."))))
        }
    }

    fn parse_if(&mut self) -> Result<Node, ParseError> {
        let start = self.current_index;
        self.advance();
        if self.current().is_none() {
            return Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), start, self.current_index + 2, String::from("expected expression, got nothing.\n    try: if 10 > 5 {10 * 5}"))))
        }
        let condition: Node = self.parse_statement()?;
        if self.current().is_none() {
            let (start, end) = condition.info();
            return Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), start, end + 2, String::from("unclosed brace inside of 'if' statement."))))
        }
        let body: Node = self.parse_block()?;
        if let Some(_) =  self.optional(&["else"]) {
            self.advance();
            let elsebody = self.parse_block()?;
            return Ok(Node::If(Box::new(condition), Box::new(body), Some(Box::new(elsebody)), start, self.current_index))
        }
        Ok(Node::If(Box::new(condition), Box::new(body), None, start, self.current_index))
    }

    fn parse_while(&mut self) -> Result<Node, ParseError> {
        let start: usize = self.current_index;
        self.advance();
        if self.current().is_none() {
            return Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), start, self.current_index + 5, String::from("expected expression, got nothing.\n    try: x = 5 while x < 100 {x = x + 5}"))))
        }
        let condition: Node = self.parse_statement()?;
        if self.current().is_none() {
            let (start, end) = condition.info();
            return Err(ParseError::UnexpectedEndOfInput(error(self.expression.clone(), start, end + 2, String::from("unclosed brace inside of 'while' statement."))))
        }
        let body: Node = self.parse_block()?;
        if let Some(_) =  self.optional(&["else"]) {
            self.advance();
            let elsebody = self.parse_block()?;
            return Ok(Node::While(Box::new(condition), Box::new(body), Some(Box::new(elsebody)), start, self.current_index))
        }
        Ok(Node::While(Box::new(condition), Box::new(body), None, start, self.current_index))
    }

    fn parse_loop(&mut self) -> Result<Node, ParseError> {
        let start = self.current_index;
        self.advance();
        let body = self.parse_block()?;
        Ok(Node::Loop(Box::new(body), start, self.current_index))
    }

    fn parse_block(&mut self) -> Result<Node, ParseError> {
        self.consume(&["{"])?;
        let start = self.current_index;
        let mut nodes = Vec::new();
        while let None = self.optional(&["}"]) {
            nodes.push(self.parse_statement()?);
        }
        self.consume(&["}"])?;
        Ok(Node::Block(nodes, start, self.current_index))
    }

    fn parse_statement(&mut self) -> Result<Node, ParseError> {
        match &self.current() {
            Some(Token::If(_, _)) => self.parse_if(),
            Some(Token::While(_, _)) => self.parse_while(),
            Some(Token::Loop(_, _)) => self.parse_loop(),
            Some(Token::Break(start, end)) => {let (start, end) = (*start, *end); self.advance(); Ok(Node::Break(start, end))},
            Some(Token::Continue(start, end)) => {let (start, end) = (*start, *end); self.advance(); Ok(Node::Continue(start, end))},
            _ => self.parse_assignment()
        }
    }

    fn parse_assignment(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_comparison()?;
        if let Some(_) = self.optional(&["="]) {
            self.advance();
            let right: Node = self.parse_comparison()?;
            let (variable, start) = match left {
                Node::Identifier(value, start, _) => Ok((value, start)),
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

    fn parse_comparison(&mut self) -> Result<Node, ParseError> {
        let left: Node = self.parse_expression()?;
        if let Some(operator) = self.optional(&[">", "<", "==", ">=", "<=", "!="]) {
            self.advance();
            let right: Node = self.parse_expression()?;
                let left: Node = match &left {
                    Node::Identifier(_, _, _) => Ok(left),
                    Node::Number(_, _, _) => Ok(left),
                    Node::Binary(_, _, _) => Ok(left),
                    Node::String(_, _, _) => Ok(left),
                    Node::Bool(_, _, _) => Ok(left),
                _ => {let (value, (start, end)) = (right.value(), right.info()); Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("expected identifier, got '{value}'."))))} }?;
            return Ok(Node::Binary(Box::new(left), operator, Box::new(right)))
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_term()?;
        while let Some(operator) = self.optional(&["+", "-"]) {
            self.advance();
            let right: Node = self.parse_term()?;
            left =  Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Node, ParseError> {
        let mut left: Node = self.parse_unary()?;
        while let Some(operator) = self.optional(&["*", "/"]) {
            self.advance();
            let right: Node = self.parse_unary()?;
            left = Node::Binary(Box::new(left), operator, Box::new(right));
        };
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, ParseError> {
        if let Some(_) = self.optional(&["-"]) {
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
                Token::Identifier(token, start, end) => {let node = Ok(Node::Identifier(token.clone(), *start, *end)); self.advance(); node}
                Token::String(value, start, end) => {let (value, start, end) = (value.clone(), *start, *end); self.advance(); Ok(Node::String(value, start, end))},
                Token::Bool(bool, start, end) => {let (bool, start, end) = (*bool, *start, *end); self.advance(); Ok(Node::Bool(bool, start, end))},
                Token::OpenParenthesis(_, _) => {
                    self.advance();
                    let expression = self.parse_expression()?;
                    //println!("{:?}", self.current());
                    self.consume(&[")"])?;
                    Ok(expression)
                }
                _ => {let (value, (start, end)) = (token.value(), token.info()); Err(ParseError::UnexpectedToken(error(self.expression.clone(), start, end, format!("unexpected '{value}'."))))},
            }
        }
        else {
            Err(ParseError::UnexpectedToken(error(self.expression.clone(), self.current_index + 1, self.current_index + 2, String::from("expected number or variable, got nothing."))))
        }
    }
}

struct Interpret {
    variables: HashMap<String, Value>,
    expression: String,
}

impl Interpret {
    fn binary(&self, left: Value, operator: &Token, right: Value, node: &Node) -> Result<Value, InterpretError> {
        match operator {
            Token::Plus(_, _) => Ok(plus(left, right)?),
            Token::Minus(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Value::Number(minus(a, b)))},
            Token::Multiply(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Value::Number(multiply(a, b)))},
            Token::Divide(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Value::Number(divide(a, b)?))},
            Token::Greater(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Value::Bool(greater(a, b)))},
            Token::Less(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Value::Bool(less(a, b)))},
            Token::Equal(_, _) => {Ok(Value::Bool(equal(left, right)?))},
            Token::GreaterEqual(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?;  Ok(Value::Bool(greaterequal(a, b)))},
            Token::LessEqual(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?;  Ok(Value::Bool(lessequal(a, b)))},
            Token::NotEqual(_, _) => {Ok(Value::Bool(!equal(left, right)?))},
            _ => {let (value, (start, end)) = (&node.value(), &node.info()); Err(InterpretError::InvalidOperator(error(self.expression.clone(), *start, *end, format!("unknown operator '{value}'."))))}
        }
    }

    fn evaluate(&mut self, node: &Node) -> Result<Value, InterpretError> {
        // println!("{:?}", node);
        match node {
            Node::Number(value, _, _) => Ok(Value::Number(*value)),
            Node::Identifier(value, _, _) => Ok(resolve_variable(&self.variables, value)?),
            Node::String(value, _, _) => Ok(Value::String(value.clone())),
            Node::Binary(left, operator, right) => {
                let left: Value = self.evaluate(left)?;
                let right: Value = self.evaluate(right)?;
                self.binary(left, operator, right, node)
            }
            Node::Assignment(variable, expression, _, _) => {
                // println!("variable: {:?}\n expression: {:?}", variable, expression);
                let expression: Value = self.evaluate(expression)?;
                self.variables.insert(variable.clone(), expression.clone());
                Ok(expression)
            }
            Node::If(condition, body, elsebody, _, _) => {
                if self.evaluate(condition)?.expect_bool()? {
                    self.evaluate(body)
                }
                else {
                    if let Some(node) = elsebody {
                        return self.evaluate(node)
                    }
                    Ok(Value::Nothing)
                }
            }
            Node::While(condition, body, elsebody, _, _) => {
                let mut last_result: Value = Value::Nothing;
                let mut count: i32 = 0;
                while self.evaluate(condition)?.expect_bool()? {
                    if count >= 1000000 {
                        return Err(InterpretError::ExecutionLimit(String::from("ERROR: execution limit exceed.")))
                    }
                    let result: Value = self.evaluate(body)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Value::Nothing)
                    }
                    if result.option_continue() {
                        continue;
                    }
                    last_result = result
                }
                if let Some(node) = elsebody {
                    last_result = self.evaluate(node)?
                }
                Ok(last_result)
            }
            Node::Loop(body, _, _) => {
                let mut count: i32 = 0;
                loop {
                    if count >= 1000000 {
                        return Err(InterpretError::ExecutionLimit(String::from("ERROR: execution limit exceed.")))
                    }
                    let result: Value = self.evaluate(body)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Value::Nothing)
                    }
                    if result.option_continue() {
                        continue;
                    }
                }
            }
            Node::Break(_, _) => Ok(Value::Break),
            Node::Continue(_, _) => Ok(Value::Continue),
            Node::Block(nodes, _, _) => {
                let mut result = Value::Nothing;
                // let mut result = Err(InterpretError::UnexpectedNode(error(self.expression.clone(), *start, *end, String::from("unexpected empty input."))));
                    for node in nodes {
                        result = self.evaluate(node)?;
                        if result.option_break() {
                            return Ok(Value::Break)
                        }
                        if result.option_continue() {
                            return Ok(Value::Continue)
                        }
                    };
                Ok(result)
            }
            Node::Bool(bool, _, _) => {let bool = *bool; Ok(Value::Bool(bool))},
            Node::UnaryMinus(node) => {
                let number: f64 = self.evaluate(node)?.expect_number()?;
                Ok(Value::Number(unary_minus(number)))
            }
        }
    }
}

fn interpret(node: &Node, expression: String) -> Result<Value, InterpretError> {
    let variables = HashMap::<String, Value>::new();
    let mut interpret = Interpret{variables, expression};
    interpret.evaluate(node)
}

fn resolve_variable(variables: &HashMap<String, Value>, variable: &str) -> Result<Value, InterpretError> {
    if let Some(value) = variables.get(variable) {
        Ok(value.clone())
    }
    else {
        Err(InterpretError::InvalidVariable(format!("ERROR: variable '{variable}' does not exist.\nif stuck, try to input in one line.\nexample: x = 5 y = x x * y")))
    }
}
