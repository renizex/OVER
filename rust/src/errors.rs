use std::fmt;

pub enum InterpretError {
    InvalidVariable(String),
    InvalidOperator(String),
    DivisionByZero(String),
    UnexpectedValue(String),
    ExecutionLimit(String),
    InvalidType(String),
    InvalidCall(String),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            InterpretError::InvalidVariable(msg) => msg,
            InterpretError::InvalidOperator(msg) => msg,
            InterpretError::DivisionByZero(msg) => msg,
            InterpretError::UnexpectedValue(msg) => msg,
            InterpretError::ExecutionLimit(msg) => msg,
            InterpretError::InvalidType(msg) => msg,
            InterpretError::InvalidCall(msg) => msg,
        };
        write!(f, "{error}")
    }
}

pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            ParseError::UnexpectedToken(msg) => msg,
            ParseError::UnexpectedEndOfInput(msg) => msg,
        };
        write!(f, "{error}")
    }
}

pub enum LexError {
    UnexpectedSymbol(String),
    UnknownSymbol(String),
    InvalidNumber(String),
    InvalidString(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            LexError::UnexpectedSymbol(msg) => msg,
            LexError::UnknownSymbol(msg) => msg,
            LexError::InvalidNumber(msg) => msg,
            LexError::InvalidString(msg) => msg,
        };
        write!(f, "{error}")
    }
}

pub fn error(expression: &str, start: usize, end: usize, msg: String) -> String {
    let pointer: String = " ".repeat(start) + &"^".repeat(end - start);
    format!("    ERROR: {msg}\n    {expression}    {pointer}")
}