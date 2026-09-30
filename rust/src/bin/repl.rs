use untitled::*;
use std::io::{self, Write};

fn main() {
    let mut expression: String = String::new();
    let mut debug: bool = false;
    println!("OVER\n");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        expression.clear();
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
        let tokens = match lexer::lex(&expression) {
            Ok(tokens) => tokens,
            Err(error) => {println!("{error}\n"); continue}
        };
        let node = match parser::parse(&tokens, &expression) {
            Ok(node) => node,
            Err(error) => {println!("{error}\n"); continue}
        };
        let result = match interpreter::interpret(&node, &expression) {
            Ok(result) => result,
            Err(error) => {println!("{error}\n"); continue}
        };
        if debug {
            println!("\ntokens: {tokens:?}");
            println!("ast: {node:?}");
        }
        println!("{}\n", result.display());
    }
}