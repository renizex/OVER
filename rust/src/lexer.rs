use crate::errors::{LexError, error};
use crate::tokens::Token;

pub fn lex(expression: &str) -> Result<Vec<Token>, LexError> {
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
            '[' => tokens.push(Token::OpenBracket(index, index + 1)),
            ']' => tokens.push(Token::CloseBracket(index, index + 1)),
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
            ',' => tokens.push(Token::Comma(index, index + 1)),
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
                    "function" => tokens.push(Token::Function(index, index + 8)),
                    "return" => tokens.push(Token::Return(index, index + 6)),
                    _ => tokens.push(Token::Identifier(variable, index, length))
                }
            }
            character if character == '"' => {
                let mut string: String = String::new();
                string.push(character);
                if let Some((_, character)) = characters.next() {
                    string.push(character)
                }
                while let Some(&(_, character)) = characters.peek() {
                    if character != '"' {
                        string.push(character);
                        characters.next();
                    }
                    else {
                        string.push('"');
                        characters.next();
                        break;
                    }
                }
                let (start, end) = (index, index + string.chars().count());
                if string.chars().last() == Some('"') && string.chars().next() == Some('"') {
                    string.pop();
                    string.remove(0);
                }
                else {
                    return Err(LexError::InvalidString(error(expression, start, end, String::from("invalid string."))))
                }
                tokens.push(Token::String(string, start, end))
            }
            _ => return Err(LexError::UnknownSymbol(error(expression, index, index + 1, format!("'{character}' is invalid."))))
        }
    }
    Ok(tokens)
}