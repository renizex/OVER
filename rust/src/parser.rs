use std::fmt::Write as FmtWrite;
use crate::tokens::Token;
use crate::nodes::Node;
use crate::errors::{error, ParseError};

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    expression: String,
}

pub fn parse(tokens: &Vec<Token>, expression: &str) -> Result<Node, ParseError> {
    let mut nodes = Vec::<Node>::new();
    let mut parser = Parser {
        current_index: 0,
        tokens: tokens.clone(),
        expression: expression.to_string(),
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
                Token::Number(value, start, end) => return Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("unexpected number '{value}'.")))),
                Token::Identifier(value, start, end) => return Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("unexpected identifier '{value}'")))),
                _ => token.value()
            };
            if expected.contains(&&*value) {
                self.advance();
                Ok(token)
            }
            else {
                let value = token.value();
                let (start, end) = token.info();
                Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("expected {display}got '{value}'."))))
            }
        }
        else {
            Err(ParseError::UnexpectedEndOfInput(error(&self.expression, &(self.expression.chars().count() - 2), &(self.expression.chars().count() - 1), format!("expected {display}got end of input."))))
        }
    }

    fn parse_if(&mut self) -> Result<Node, ParseError> {
        let start = self.current_index;
        self.advance();
        if self.current().is_none() {
            return Err(ParseError::UnexpectedEndOfInput(error(&self.expression, &start, &(self.current_index + 2), String::from("expected expression, got nothing.\n    try: if 10 > 5 {10 * 5}"))))
        }
        let condition: Node = self.parse_statement()?;
        if self.current().is_none() {
            let (start, end) = condition.info();
            return Err(ParseError::UnexpectedEndOfInput(error(&self.expression, &start, &(end + 2), String::from("unclosed brace inside of 'if' statement."))))
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
            return Err(ParseError::UnexpectedEndOfInput(error(&self.expression, &start, &(self.current_index + 5), String::from("expected expression, got nothing.\n    try: x = 5 while x < 100 {x = x + 5}"))))
        }
        let condition: Node = self.parse_statement()?;
        if self.current().is_none() {
            let (start, end) = condition.info();
            return Err(ParseError::UnexpectedEndOfInput(error(&self.expression, &start, &(end + 2), String::from("unclosed brace inside of 'while' statement."))))
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

    fn parse_function(&mut self) -> Result<Node, ParseError> {
        let start: usize = self.current_index;
        self.advance();
        let identifier: Node = self.parse_factor()?;
        if !matches!(identifier, Node::Identifier(_, _, _)) {
            let (start, end) = identifier.info();
            return Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("expected identifier, got {}.", identifier.value()))))
        }
        self.consume(&["("])?;
        let mut arguments = Vec::new();
        if self.optional(&[")"]).is_some() {
            self.consume(&[")"])?;
            let body = self.parse_block()?;
            let end = self.current_index;
            return Ok(Node::Function(Box::new(identifier), arguments, Box::new(body), start, end))
        }
        else {
            arguments.push(self.parse_expression()?);
        }
        while self.optional(&[")"]).is_none() {
            self.consume(&[","])?;
            arguments.push(self.parse_expression()?);
        }
        self.consume(&[")"])?;
        let body = self.parse_block()?;
        let end = self.current_index;
        Ok(Node::Function(Box::new(identifier), arguments, Box::new(body), start, end))
    }

    fn parse_class(&mut self) -> Result<Node, ParseError> {
        let (start, _) = self.current().unwrap().info();
        self.advance();
        let identifier = self.parse_factor()?;
        self.consume(&["{"])?;
        let mut fields = Vec::new();
        while self.optional(&["}"]).is_none() {
            let field = self.parse_factor()?;
            match field {
                Node::Identifier(_, _, _) => fields.push(field),
                _=> {let (start, end) = field.info(); return Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("'{}' can not be attribute.", field.value()))))}
            };
        }
        if self.optional(&["}"]).is_none() {
            self.consume(&["}"])?;
        }
        let (_, end) = self.current().unwrap().info();
        self.consume(&["}"])?;
        Ok(Node::Class(Box::new(identifier), fields, start, end))
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
            Some(Token::Function(_, _)) => self.parse_function(),
            Some(Token::Return(start, end)) => {let (start, end) = (*start, *end); self.advance(); if self.optional(&["}"]).is_some() {Ok(Node::Return(None, start, end))} else {let expression = self.parse_expression()?; Ok(Node::Return(Some(Box::new(expression)), start, end))}},
            Some(Token::Class(_, _)) => self.parse_class(),
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
                    Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("expected identifier, got '{value}'."))))
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
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Node, ParseError> {
        let mut prefix: Node = self.parse_factor()?;
        while let Some(postfix) = self.optional(&["[", ".", "(", "{"]) {
            let (start, _) = self.current().unwrap().info();
            if postfix.value() == "[" {
                self.advance();
                let index = self.parse_factor()?;
                if !self.optional(&["]"]).is_some() {
                    self.consume(&["]"])?;
                }
                let (_, end) = self.current().unwrap().info();
                self.consume(&["]"])?;
                prefix = Node::Index(Box::new(prefix.clone()), Box::new(index), end, start);
            }
            else if postfix.value() == "(" {
                self.advance();
                let mut arguments = Vec::new();
                if let Some(_) = self.optional(&[")"]) {
                    let (_, end) = self.current().unwrap().info();
                    self.advance();
                    return Ok(Node::FunctionCall(Box::new(prefix), arguments, start, end))
                }
                arguments.push(self.parse_expression()?);
                while let Some(_) = self.optional(&[","]) {
                    self.advance();
                    arguments.push(self.parse_expression()?);
                }
                if !self.optional(&[")"]).is_some() {
                    self.consume(&[")"])?;
                }
                let (_, end) = self.current().unwrap().info();
                self.consume(&[")"])?;
                return Ok(Node::FunctionCall(Box::new(prefix), arguments, start, end))
            }
            else if postfix.value() == "{" {
                if !matches!(prefix.clone(), Node::Identifier(identifier, _, _) if identifier.chars().next().unwrap().is_uppercase()) {return Ok(prefix)}
                self.advance();
                let mut fields = Vec::new();
                while self.optional(&["}"]).is_none() {
                    fields.push(self.parse_expression()?);
                }
                if self.optional(&["}"]).is_none() {
                    self.consume(&["}"])?;
                }
                let (_, end) = self.current().unwrap().info();
                self.consume(&["}"])?;
                prefix = Node::ClassCall(Box::new(prefix), fields, start, end)
            }
            while self.optional(&["."]).is_some() {
                self.advance();
                let mut object = Vec::new();
                let postfix = self.parse_factor()?;
                if self.optional(&["("]).is_some() {
                    self.advance();
                    let mut arguments = Vec::new();
                    if let Some(_) = self.optional(&[")"]) {
                        let (_, end) = self.current().unwrap().info();
                        self.advance();
                        return Ok(Node::FunctionCall(Box::new(postfix), vec![prefix], start, end))
                    }
                    arguments.push(self.parse_expression()?);
                    while let Some(_) = self.optional(&[","]) {
                        self.advance();
                        arguments.push(self.parse_expression()?);
                    }
                    if !self.optional(&[")"]).is_some() {
                        self.consume(&[")"])?;
                    }
                    let (_, end) = self.current().unwrap().info();
                    self.consume(&[")"])?;
                    object.push(prefix.clone());
                    for _ in 0..arguments.len() {
                        object.push(arguments[arguments.len() - 1].clone());
                    }
                    prefix = Node::FunctionCall(Box::new(postfix), object, start, end)
                }
                else {
                    let end = self.current_index;
                    prefix = Node::Object(Box::new(prefix), Box::new(postfix), start, end);
                }
            }
        }
        Ok(prefix)
    }

    fn parse_factor(&mut self) -> Result<Node, ParseError> {
        if let Some(token) = self.current() {
            match token {
                Token::Number(token, start, end) => {let node = Ok(Node::Number(*token, *start, *end)); self.advance(); node}
                Token::Identifier(token, start, end) => {let node = Ok(Node::Identifier(token.clone(), *start, *end)); self.advance(); node}
                Token::String(value, start, end) => {let (value, start, end) = (value.clone(), *start, *end); self.advance(); Ok(Node::String(value, start, end))},
                Token::OpenBracket(start, _) => {
                    let start = *start;
                    let mut list = Vec::new();
                    self.advance();
                    if self.optional(&["]"]).is_some() {
                        self.advance();
                        return Ok(Node::List(list, start, start + 1))
                    }
                    list.push(self.parse_factor()?);
                    while self.optional(&[","]).is_some() {
                        self.advance();
                        list.push(self.parse_factor()?)
                    }
                    let (_, end) = self.current().unwrap().info();
                    self.consume(&["]"])?;
                    Ok(Node::List(list, start, end))
                },
                Token::Bool(bool, start, end) => {let (bool, start, end) = (*bool, *start, *end); self.advance(); Ok(Node::Bool(bool, start, end))},
                Token::OpenParenthesis(_, _) => {
                    self.advance();
                    let expression = self.parse_expression()?;
                    self.consume(&[")"])?;
                    Ok(expression)
                }
                _ => {let (value, (start, end)) = (token.value(), token.info()); Err(ParseError::UnexpectedToken(error(&self.expression, &start, &end, format!("unexpected '{value}'."))))},
            }
        }
        else {
            Err(ParseError::UnexpectedToken(error(&self.expression, &(self.current_index + 1), &(self.current_index + 2), String::from("expected number or variable, got nothing."))))
        }
    }
}