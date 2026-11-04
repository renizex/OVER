use std::collections::HashMap;
use crate::tokens::Token;
use crate::nodes::Node;
use crate::flow::{Value, Flow};
use crate::errors::{InterpretError, error};
use crate::flow::Value::Nothing;
use crate::operations::*;

pub fn interpret(node: &Node, expression: &str) -> Result<Flow, InterpretError> {
    let scope = Scope{ local: HashMap::new(), parent: HashMap::new()};
    let functions = HashMap::<String, Node>::new();
    let classes = HashMap::<String, Node>::new();
    let mut interpret = Interpret{variables: scope.clone(), classes, functions, expression: expression.to_string()};
    interpret.evaluate(node)
}

struct Interpret {
    variables: Scope,
    classes: HashMap<String, Node>,
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
            _ => {let (value, (start, end)) = (&node.value(), &node.info()); Err(InterpretError::InvalidOperator(error(&self.expression, start, end, format!("unknown operator '{value}'."))))}
        }
    }

    fn evaluate(&mut self, node: &Node) -> Result<Flow, InterpretError> {
        match node {
            Node::Number(value, _, _) => Ok(Flow::Value(Value::Number(*value))),
            Node::Identifier(value, _, _) => Ok(resolve_variable(&self.variables, value)?),
            Node::String(value, _, _) => Ok(Flow::Value(Value::String(value.clone()))),
            Node::Binary(left, operator, right) => {
                let left: Flow = self.evaluate(left)?;
                let right: Flow = self.evaluate(right)?;
                if left.option_return() {
                    return Ok(left)
                }
                if right.option_return() {
                    return Ok(right)
                }
                self.binary(left, operator, right, node)
            }
            Node::Assignment(variable, expression, _, _) => {
                let expression: Flow = self.evaluate(expression)?;
                self.variables.set(variable.clone(), expression.clone());
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
                    Ok(Flow::Value(Nothing))
                }
            }
            Node::While(condition, body, elsebody, _, _) => {
                let mut last_result: Flow = Flow::Value(Nothing);
                let mut count: i32 = 0;
                while self.evaluate(condition)?.expect_bool()? {
                    if count >= 1000000 {
                        return Err(InterpretError::ExecutionLimit(String::from("ERROR: execution limit exceed.")))
                    }
                    let result: Flow = self.evaluate(body)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Flow::Value(Nothing))
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
                    let result: Flow = self.evaluate(body)?;
                    count += 1;
                    if result.option_break() {
                        return Ok(Flow::Value(Nothing))
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
                let mut result = Flow::Value(Nothing);
                for node in nodes {
                    result = self.evaluate(node)?;
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
                let number: f64 = self.evaluate(node)?.expect_number()?;
                Ok(Flow::Value(Value::Number(unary_minus(number))))
            }
            Node::Function(identifier, _, _, _, _) => {
                self.functions.insert(identifier.value(), node.clone());
                Ok(Flow::Value(Nothing))
            }
            Node::FunctionCall(identifier, arguments, start, end) => {
                if &identifier.value() == "append" {
                    if arguments.len() != 2 {
                        let (start, end) = identifier.info(); return Err(InterpretError::InvalidCall(error(&self.expression, &start, &end, format!("expected 1 arguments, got {}.", arguments.len() - 1))))
                    }
                    let mut god = arguments.clone();
                    let argument = god.pop().unwrap();
                    let (list, list_identifier) = match god.pop() {
                        Some(node) => {
                            match node.clone() {
                                Node::Identifier(identifier, _, _) => {
                                    let mut list = match self.evaluate(&node)? {
                                        Flow::Value(Value::List(list)) => convert(list),
                                        _ => unreachable!()
                                    };
                                    list.push(argument);
                                    (list, identifier)
                                },
                                Node::Index(identifier, index, _, _) => {
                                    let mut indices = vec![*index];
                                    self.recursive_append(&identifier, &mut indices, argument)?
                                },
                                _=> unreachable!()
                            }
                        }
                        _=> unreachable!()
                    };
                    self.evaluate(&Node::Assignment(list_identifier, Box::new(Node::List(list, 0, 0)), 0, 0))?;
                    return Ok(Flow::Value(Nothing))
                }
                if &identifier.value() == "pop" {
                    if arguments.len() > 1 {
                        let (start, end) = identifier.info(); return Err(InterpretError::InvalidCall(error(&self.expression, &start, &end, format!("expected 0 arguments, got {}.", arguments.len() - 1))))
                    }
                    let ((popped, list), identifier) =  {
                        self.recursive_pop(arguments[0].clone(), &mut Vec::new())?
                    };
                    let popped = match popped {
                        Some(some) => self.evaluate(&some)?,
                        None => Flow::Value(Nothing)
                    };
                    self.evaluate(&Node::Assignment(identifier, Box::new(Node::List(list, 0, 0)), 0, 0))?;
                    return Ok(popped)
                }
                match self.functions.get(&identifier.value()).cloned() {
                    Some(value) => match value {
                        Node::Function(_, parameters, body, _, _) => {
                            if arguments.len() != parameters.len() {
                                return Err(InterpretError::InvalidCall(error(&self.expression, start, end, format!("expected {} arguments, got {}.", parameters.len(), arguments.len()))))
                            }
                            let mut local = HashMap::new();
                            for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
                                let parameter = parameter.value();
                                let argument = self.evaluate(argument)?;
                                local.insert(parameter, argument);
                            }
                            self.variables = Scope{local, parent: self.variables.parent.clone()};
                            match self.evaluate(&body.clone())? {
                                Flow::Return(expression) => {
                                    if expression.is_some() {
                                        return self.evaluate(&expression.unwrap())
                                    }
                                    Ok(Flow::Value(Nothing))
                                }
                                _ => Ok(Flow::Value(Nothing))
                            }
                        }
                        _ => unreachable!()
                    },
                    None => {let (start, end) = identifier.info(); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, format!("function '{}' does not exist.", identifier.value()))))}
                }
            }
            Node::Return(expression, _, _) => {
                let expression = if expression.is_some() {Some(*expression.clone().unwrap())} else {None};
                Ok(Flow::Return(expression))
            },
            Node::List(list, _, _) => {
                let mut result = Vec::new();
                for node in list {
                    result.push(self.evaluate(node)?)
                }
                Ok(Flow::Value(Value::List(result)))
            }
            Node::Index(identifier, index, start, end) => {
                let flow = self.evaluate(identifier)?;
                let index = self.evaluate(index)?;
                match &flow {
                    Flow::Value(Value::List(list)) => match index {
                        Flow::Value(Value::Number(index)) => {
                            if index.fract() != 0.0 {
                                {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, format!("index '{index}' is invalid. try '0'."))))}
                            }
                            else if index < 0.0 {
                                {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, String::from("index can not be negative."))))}
                            }
                            else {
                                let index: usize = index as usize;
                                match list.get(index) {
                                    Some(value) => Ok(value.clone()),
                                    None => {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, format!("index '{index}' out of range."))))}
                                }
                            }
                        }
                        _ => {let (start, end) = (*start, *end); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, String::from("index can only be integer."))))}
                    },
                    _ => {let (start, end) = (start - flow.display().len(), *start); Err(InterpretError::InvalidVariable(error(&self.expression, &start, &end, format!("'{}' is not a list.", flow.display()))))}
                }
            },
            Node::Class(identifier, _, _, _) => {
                self.classes.insert(identifier.value(), node.clone());
                Ok(Flow::Value(Nothing))
            }
            Node::ClassCall(identifier, fields, start, end) => {
                let mut objects = HashMap::new();
                match self.classes.get(&identifier.value()) {
                    Some(class) => match class.clone() {
                        Node::Class(_, arguments, _, _) => {
                            if fields.len() != arguments.len() {
                                return Err(InterpretError::InvalidCall(error(&self.expression, start, end, format!("expected {} arguments, got {}.\n{:?}", arguments.len(), fields.len(), fields))))
                            }
                            for (arguments, field) in arguments.iter().zip(fields.iter()) {
                                objects.insert(arguments.value(), field.clone());
                            }
                            self.variables.set(identifier.value(), Flow::Value(Value::Object(objects.clone())));
                        },
                        _=> unreachable!()
                    },
                    None => {let (start, end) = identifier.info(); return Err(InterpretError::InvalidCall(error(&self.expression, &start, &end, format!("class {} does not exist.", identifier.value()))))}
                };
                Ok(Flow::Value(Value::Object(objects)))
            }
            Node::Object(identifier, attribute, _, _) => {
                let evaluated = self.evaluate(identifier)?;
                match evaluated {
                    Flow::Value(Value::Object(attributes)) => match attributes.get(&attribute.value()) {
                        Some(node) => Ok(self.evaluate(node)?),
                        None => {let (start, end) = attribute.info(); Err(InterpretError::InvalidCall(error(&self.expression, &start, &end,format!("'{}' does not have attribute '{}'.", identifier.value(), attribute.value()))))}
                    },
                    _=> {let (start, end) = identifier.info(); Err(InterpretError::InvalidCall(error(&self.expression, &start, &end, format!("{} does not have attributes.", evaluated.type_name()))))}
                }
            }
        }
    }

    fn recursive_append(&mut self, identifier: &Node, indices: &mut Vec<Node>, victim: Node) -> Result<(Vec<Node>, String), InterpretError> {
        match &identifier {
            Node::Index(identifier, index, _, _) => {
                indices.push(*index.clone());
                self.recursive_append(identifier, indices, victim)
            },
            Node::Identifier(string_identifier, _, _) => {
                let original = match self.evaluate(&identifier)? {
                    Flow::Value(Value::List(list)) => convert(list),
                    _=> unreachable!()
                };
                indices.reverse();
                Ok((self.append_original_vec(original, indices, victim)?, string_identifier.clone()))
            },
            _=> unreachable!()
        }
    }
    fn recursive_pop(&mut self, identifier: Node, indices: &mut Vec<Node>) -> Result<((Option<Node>, Vec<Node>), String), InterpretError> {
        match &identifier {
            Node::Index(identifier, index, _, _) => {
                indices.push(*index.clone());
                self.recursive_pop(*identifier.clone(), indices)
            },
            Node::Identifier(string_identifier, _, _) => {
                let original = match self.evaluate(&identifier)? {
                    Flow::Value(Value::List(list)) => convert(list),
                    _=> unreachable!()
                };
                indices.reverse();
                Ok((self.pop_original_vec(original, indices)?, string_identifier.clone()))
            },
            _=> unreachable!()
        }
    }
    fn pop_original_vec(&mut self, mut original: Vec<Node>, indices: &Vec<Node>) -> Result<(Option<Node>, Vec<Node>), InterpretError> {
        let mut indexed = &mut original;
        for index in indices {
            let usize_index = self.get_usize_index(index)?;
            indexed = match indexed.get_mut(usize_index) {
                Some(some) => match some {
                    Node::List(list, _, _) => list,
                    _=> unreachable!()
                },
                None => {let (start, end) = &index.info(); return Err(InterpretError::InvalidCall(error(&self.expression, &(start - 1), &(end + 1), format!("index {} is out of range. maximum index: {}.", usize_index, original.len() - 1))))}
            }
        }
        let popped = indexed.pop();
        Ok((popped, original))
    }

    fn append_original_vec(&mut self, mut original: Vec<Node>, indices: &Vec<Node>, victim: Node) -> Result<Vec<Node>, InterpretError> {
        let mut indexed = &mut original;
        let (last_index, indices) = indices.split_last().unwrap();
        let last_usize_index = self.get_usize_index(last_index)?;
        for index in indices {
            let usize_index = self.get_usize_index(index)?;
            let fallback = indexed[usize_index].clone();
            indexed = match &mut indexed[usize_index] {
                Node::List(list, _, _) => list,
                _=> {let (start, end) = &last_index.info(); return Err(InterpretError::InvalidCall(error(&self.expression.clone(), &(start - 1), &(end + 1), format!("you can not index from {}.", self.evaluate(&fallback)?.type_name()))))}
            };
        }
        let mut true_victim: Vec<Node> = match indexed.get(last_usize_index) {
            Some(some) => match some {
                Node::List(list, _, _) => list.clone(),
                _=> unreachable!()
            },
            None => {let (start, end) = &last_index.info(); return Err(InterpretError::InvalidCall(error(&self.expression, &(start - 1), &(end + 1), format!("index {} is out of range. maximum index: {}.", last_usize_index, original.len() - 1))))}
        };
        true_victim.push(victim);
        indexed[last_usize_index] = Node::List(true_victim.clone(), 0, 0);
        Ok(original)
    }

    fn get_usize_index(&mut self, last_index: &Node) -> Result<usize, InterpretError> {
        match self.evaluate(&last_index)? {
            Flow::Value(Value::Number(number)) => {let (start, end) = &last_index.info();
                Ok(format_to_usize(number, &self.expression, start, end)?)
            },
            _=> unreachable!()
        }
    }

}

fn format_to_usize(victim: f64, expression: &str, start: &usize, end: &usize) -> Result<usize, InterpretError> {
    if victim.fract() != 0.0 {
        {
            Err(InterpretError::InvalidVariable(error(expression, &start, end, format!("float indexes like '{victim}' are not allowed."))))
        }
    } else if victim < 0.0 {
        {
            Err(InterpretError::InvalidVariable(error(expression, start, end, String::from("indexes can not be negative."))))
        }
    } else {
        return Ok(victim as usize)
    }
}

fn convert(list: Vec<Flow>) -> Vec<Node> {
    let mut converted = Vec::new();
    for flow in list {
        match flow {
            Flow::Value(value) => match value {
                Value::Number(number) => converted.push(Node::Number(number, 0, 0)),
                Value::Bool(bool) => converted.push(Node::Bool(bool, 0, 0)),
                Value::String(string) => converted.push(Node::String(string, 0, 0)),
                Value::List(list) => converted.push(Node::List(convert(list), 0, 0)),
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }
    converted
}

fn resolve_variable(variables: &Scope, variable: &str) -> Result<Flow, InterpretError> {
    if let Some(value) = variables.get(variable.parse().unwrap()) {
        Ok(value.clone())
    }
    else {
        Err(InterpretError::InvalidVariable(format!("ERROR: variable '{variable}' does not exist.\nif stuck, try to input in one line.\nexample: x = 5 y = x x * y")))
    }
}

#[derive(Debug)]
#[derive(Clone)]
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