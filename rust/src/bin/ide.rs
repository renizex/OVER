use untitled::{interpreter, lexer, parser};

fn main() {
    let path: String = match std::env::args().nth(1) {
        Some(path) => path,
        None => {println!("ERROR: path is invalid."); return}
    };
    let expression = match std::fs::read_to_string(path) {
        Ok(expression) => expression,
        Err(_) => {println!("ERROR: failed to read."); return}
    };
    let tokens = match lexer::lex(&expression) {
        Ok(tokens) => tokens,
        Err(error) => {println!("{error}\n"); return}
    };
    let node = match parser::parse(&tokens, &expression) {
        Ok(node) => node,
        Err(error) => {println!("{error}\n"); return}
    };
    let result = match interpreter::interpret(&node, &expression) {
        Ok(result) => result,
        Err(error) => {println!("{error}\n"); return}
    };
    println!("{}\n", result.display());
}