use std::collections::HashMap;
use crate::tokens::Token;
use crate::nodes::Node;
use crate::flow::{Value, Flow};
use crate::errors::{InterpretError, error};
use crate::operations::*;

pub fn interpret(node: &Node, expression: &str) -> Result<Flow, InterpretError> {
    let mut scope = Scope{ local: HashMap::new(), parent: HashMap::new()};
    let functions = HashMap::<String, Node>::new();
    let mut interpret = Interpret{functions, expression: expression.to_string()};
    interpret.evaluate(node, &mut scope)
}

struct Interpret {
    functions: HashMap<String, Node>,
    expression: String,
}

impl Interpret {
    fn binary(&self, left: Flow, operator: &Token, right: Flow, node: &Node) -> Result<Flow, InterpretError> {
        match operator {
            Token::Plus(_, _) => Ok(Flow::Value(plus(left, right)?)),
            Token::Minus(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Flow::Value(Value::Number(minus(a, b))))},
            Token::Multiply(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Flow::Value(Value::Number(multiply(a, b))))},
            Token::Divide(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Flow::Value(Value::Number(divide(a, b)?)))},
            Token::Greater(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Flow::Value(Value::Bool(greater(a, b))))},
            Token::Less(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?; Ok(Flow::Value(Value::Bool(less(a, b))))},
            Token::Equal(_, _) => {Ok(Flow::Value(Value::Bool(equal(left, right)?)))},
            Token::GreaterEqual(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?;  Ok(Flow::Value(Value::Bool(greaterequal(a, b))))},
            Token::LessEqual(_, _) => {let a: f64 = left.expect_number()?; let b: f64 = right.expect_number()?;  Ok(Flow::Value(Value::Bool(lessequal(a, b))))},
            Token::NotEqual(_, _) => {Ok(Flow::Value(Value::Bool(!equal(left, right)?)))},
            _ => {let (value, (start, end)) = (&node.value(), &node.info()); Err(InterpretError::InvalidOperator(error(&self.expression, *start, *end, format!("unknown operator '{value}'."))))}
        }
    }

    fn evaluate(&mut self, node: &Node, variables: &mut Scope) -> Result<Flow, InterpretError> {
        // println!("{:?}", node);
        match node {
            Node::Number(value, _, _) => Ok(Flow::Value(Value::Number(*value))),
            Node::Identifier(value, _, _) => Ok(resolve_variable(variables, value)?),
            Node::String(value, _, _) => Ok(Flow::Value(Value::String(value.clone()))),
            Node::Binary(left, operator, right) => {
                let left: Flow = self.evaluate(left, variables)?;
                let right: Flow = self.evaluate(right, variables)?;
                if left.option_return() {
                    return Ok(left)
                }
                if right.option_return() {
                    return Ok(right)
                }
                self.binary(left, operator, right, node)
            }
            Node::Assignment(variable, expression, _, _) => {
                let expression: Flow = self.evaluate(expression, variables)?;
                variables.set(variable.clone(), expression.clone());
                Ok(expression)
            }
            Node::If(condition, body, elsebody, _, _) => {
                if self.evaluate(condition, variables)?.expect_bool()? {
                    self.evaluate(body, variables)
                }
                else {
                    if let Some(node) = elsebody {
                        return self.evaluate(node, variables)
                    }
                    Ok(Flow::Value(Value::Nothing))
                }
            }
            Node::While(condition, body, elsebody, _, _) => {
                let mut last_result: Flow = Flow::Value(Value::Nothing);
                let mut count: i32 = 0;
                while self.evaluate(condition, variables)?.expect_bool()? {
                    if count >= 1000000 {
                        return Err(InterpretError::ExecutionLimit(String::from("ERROR: execution limit exceed.")))
                    }
                    let result: Flow = self.evaluate(body, variables)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Flow::Value(Value::Nothing))
                    }
                    if result.option_continue() {
                        continue;
                    }
                    if result.option_return() {
                        return Ok(result)
                    }
                    last_result = result
                }
                if let Some(node) = elsebody {
                    last_result = self.evaluate(node, variables)?
                }
                Ok(last_result)
            }
            Node::Loop(body, _, _) => {
                let mut count: i32 = 0;
                loop {
                    if count >= 1000000 {
                        return Err(InterpretError::ExecutionLimit(String::from("ERROR: execution limit exceed.")))
                    }
                    let result: Flow = self.evaluate(body, variables)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Flow::Value(Value::Nothing))
                    }
                    if result.option_continue() {
                        continue;
                    }
                    if result.option_return() {
                        return Ok(result)
                    }
                }
            }
            Node::Break(_, _) => Ok(Flow::Break),
            Node::Continue(_, _) => Ok(Flow::Continue),
            Node::Block(nodes, _, _) => {
                let mut result = Flow::Value(Value::Nothing);
                for node in nodes {
                    result = self.evaluate(node, variables)?;
                    if result.option_break() {
                        return Ok(Flow::Break)
                    }
                    if result.option_continue() {
                        return Ok(Flow::Continue)
                    }
                    if result.option_return() {
                        return Ok(result)
                    }
                };
                Ok(result)
            }
            Node::Bool(bool, _, _) => {let bool = *bool; Ok(Flow::Value(Value::Bool(bool)))},
            Node::UnaryMinus(node) => {
                let number: f64 = self.evaluate(node, variables)?.expect_number()?;
                Ok(Flow::Value(Value::Number(unary_minus(number))))
            }
            Node::Function(identifier, _, _, _, _) => {
                self.functions.insert(identifier.value(), node.clone());
                Ok(Flow::Value(Value::Nothing))
            }
            Node::Call(identifier, arguments, start, end) => {
                match self.functions.get(&*identifier.value()).cloned() {
                    Some(value) => match value {
                        Node::Function(_, parameters, body, _, _) => {
                            if arguments.len() != parameters.len() {
                                return Err(InterpretError::InvalidCall(error(&self.expression, *start, *end, format!("expected {} arguments, got {}.", parameters.len(), arguments.len()))))
                            }
                            let mut local = HashMap::new();
                            for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
                                let parameter = parameter.value();
                                let argument = self.evaluate(argument, variables)?;
                                local.insert(parameter, argument);
                            }
                            let mut scope = Scope{local, parent: variables.parent.clone()};
                            match self.evaluate(&*body.clone(), &mut scope)? {
                                Flow::Return(expression) => {
                                    if expression.is_some() {
                                        return self.evaluate(&expression.unwrap(), &mut scope)
                                    }
                                    Ok(Flow::Value(Value::Nothing))
                                }
                                _ => Ok(Flow::Value(Value::Nothing))
                            }
                        }
                        _ => unreachable!()
                    },
                    None => Err(InterpretError::InvalidVariable(error(&self.expression, *start, *end, format!("function '{}' does not exist.", identifier.value()))))
                }
            }
            Node::Return(expression, _, _) => {
                let expression = if expression.is_some() {Some(*expression.clone().unwrap())} else {None};
                Ok(Flow::Return(expression))
            },
            Node::List(list, _, _) => {
                let mut result = Vec::new();
                for node in list {
                    result.push(self.evaluate(node, variables)?)
                }
                Ok(Flow::Value(Value::List(result)))
            }
            Node::Index(identifier, index, start, end) => {
                let flow = self.evaluate(identifier, variables)?;
                let index = self.evaluate(index, variables)?;
                match &flow {
                    Flow::Value(Value::List(list)) => match index {
                        Flow::Value(Value::Number(index)) => {
                            if index.fract() != 0.0 {
                                {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, start, end, format!("index '{index}' is invalid. try '0'."))))}
                            }
                            else if index < 0.0 {
                                {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, start, end, String::from("index can not be negative."))))}
                            }
                            else {
                                let index: usize = index as usize;
                                match list.get(index) {
                                    Some(shit) => Ok(shit.clone()),
                                    None => {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, start, end, format!("index '{index}' out of range."))))}
                                }
                            }
                        }
                        _ => {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, start, end, String::from("index can only be integer."))))}
                    },
                    _ => {let (start, end) = (start - flow.display().len(), *start); Err(InterpretError::InvalidVariable(error(&self.expression, start, end, format!("'{}' is not a list.", flow.display()))))}
                }
            },
        }
    }
}

fn resolve_variable(variables: &Scope, variable: &str) -> Result<Flow, InterpretError> {
    if let Some(value) = variables.get(variable.parse().unwrap()) {
        Ok(value.clone())
    }
    else {
        Err(InterpretError::InvalidVariable(format!("ERROR: variable '{variable}' does not exist.\nif stuck, try to input in one line.\nexample: x = 5 y = x x * y")))
    }
}

struct Scope {
    local: HashMap<String, Flow>,
    parent: HashMap<String, Flow>,
}

impl Scope {
    fn get(&self, target: String) -> Option<Flow> {
        match self.local.get(&target) {
            Some(value) => Some(value.clone()),
            None => match self.parent.get(&target) {
                Some(value) => Some(value.clone()),
                None => None
            }
        }
    }

    fn set(&mut self, target: String, value: Flow) {
        self.local.insert(target, value);
    }
}