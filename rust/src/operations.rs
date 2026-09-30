use crate::flow::{Value, Flow};
use crate::errors::{InterpretError};

pub fn plus(a: Flow, b: Flow) -> Result<Value, InterpretError> {
    match (&a, &b) {
        (Flow::Value(a), Flow::Value(b)) => match (a, b){
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.to_owned() + b)),
            _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
        }
        _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
    }
}

pub fn minus(a: f64, b: f64) -> f64 {
    a-b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a*b
}

pub fn divide(a: f64, b: f64) -> Result<f64, InterpretError> {
    if b == 0.0 {
        return Err(InterpretError::DivisionByZero(String::from("ERROR: division by zero.")))
    };
    Ok(a/b)
}

pub fn unary_minus(a: f64) -> f64 {
    -a
}

pub fn greater(a: f64, b: f64) -> bool {
    a > b
}

pub fn less(a: f64, b: f64) -> bool {
    a < b
}

pub fn equal(a: Flow, b: Flow) -> Result<bool, InterpretError> {
    match (&a, &b) {
        (Flow::Value(a), Flow::Value(b)) => match (a, b) {
            (Value::Number(a), Value::Number(b)) => Ok(a == b),
            (Value::String(a), Value::String(b)) => Ok(a == b),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
        }
        _ => Err(InterpretError::InvalidType(format!("ERROR: expected {}, got {}.", a.type_name(), b.type_name())))
    }
}

pub fn greaterequal(a: f64, b: f64) -> bool {
    a >= b
}

pub fn lessequal(a: f64, b: f64) -> bool {
    a <= b
}