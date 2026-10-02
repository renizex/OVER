use crate::errors::InterpretError;
use crate::nodes::Node;

#[derive(Clone)]
pub enum Flow {
    Value(Value),
    Break,
    Continue,
    Return(Option<Node>),
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    List(Vec<Flow>),
    Nothing,

}

impl Flow {
    pub fn expect_number(&self) -> Result<f64, InterpretError> {
        match self {
            Flow::Value(value) => match value {
                Value::Number(value) => Ok(*value),
                _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected number, got {}.", self.type_name()))),
            }
            _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected number, got {}.", self.type_name()))),
        }
    }

    pub fn expect_bool(&self) -> Result<bool, InterpretError> {
        match self {
            Flow::Value(value) => match value {
                Value::Bool(value) => Ok(*value),
                _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected number, got {}.", self.type_name()))),
            }
            _ => Err(InterpretError::UnexpectedValue(format!("ERROR: expected number, got {}.", self.type_name()))),
        }
    }

    pub fn option_break(&self) -> bool {
        match self {
            Flow::Break => true,
            _ => false
        }
    }

    pub fn option_continue(&self) -> bool {
        match self {
            Flow::Continue => true,
            _ => false
        }
    }

    pub fn option_return(&self) -> bool {
        match self {
            Flow::Return(_) => true,
            _ => false
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Flow::Value(_) => "value",
            Flow::Break => "break",
            Flow::Continue => "continue",
            Flow::Return(_) => "return",
        }
    }

    pub fn display(&self) -> String {
        match self {
            Flow::Value(value) => value.display(),
            Flow::Break => String::from("break"),
            Flow::Continue => String::from("continue"),
            Flow::Return(_) => String::from("return"),
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Nothing => "nothing",
            Value::List(_) => "list",
        }
    }

    pub fn display(&self) -> String {
        match self {
            Value::Number(value) => value.to_string(),
            Value::Bool(value) => value.to_string(),
            Value::String(value) => value.clone(),
            Value::Nothing => String::from("nothing"),
            Value::List(list) => {
                let list: Vec<String> = list.iter().map(|x|
                    match x {
                        Flow::Value(Value::String(_)) => format!("\"{}\"", x.display()),
                        _ => x.display()
                    }).collect();
                format!("[{}]", list.join(", "))
            }
        }
    }
}