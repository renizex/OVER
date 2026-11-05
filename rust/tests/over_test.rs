use untitled::lexer::lex;
use untitled::parser::parse;
use untitled::interpreter::interpret;

fn run(expression: &str) -> String {
    let tokens = match lex(expression) {
        Ok(tokens) => tokens,
        Err(_) => {panic!("ты обосрался") }
    };
    let nodes = match parse(&tokens, expression) {
        Ok(nodes) => nodes,
        Err(_) => { panic!("ты обосрался") }
    };
    match interpret(&nodes, expression) {
        Ok(flow) => flow.display(),
        Err(_) => { panic!("ты обосрался") }
    }
}

#[test]
fn tests() {
    assert_eq!(run("2+2"), "4");
    assert_eq!(run("(2 + 3 * 4) / 7 - 0"), "2");
    assert_eq!(run("x = 5 if x > 5 {10} else {20}"), "20");
    assert_eq!(run("x = 0 while x < 1000 {x = x + 1} x"), "1000");
    assert_eq!(run("x = 5 loop{x = x + 1 if x > 99 {break}} x"), "100");
    assert_eq!(run("function func(a, b) {if a > b {return func(a - 500, b)} else {return 1000}} func(5, 1)"), "1000");
    assert_eq!(run("x = \"всем привет ребята\" y = \"всем пока ребята\" if x != y {1000} else {500}"), "1000");
    assert_eq!(run("x = true y = 100 while x == true {if y > 200 {x = false} else {y = y + 1}} else {12345}"), "12345");
    assert_eq!(run("x = [1, 2, 3] x.append(4) x"), "[1, 2, 3, 4]");
    assert_eq!(run("x = [1, \"shit\", false] x.pop() x"), "[1, \"shit\"]");
    assert_eq!(run("x = [1, 2, false] pop(x) pop(x) x"), "[1]");
    assert_eq!(run("x = [1, 2, 3] pop(x) pop(x) append(x, 4) pop(x) append(x, 5) x"), "[1, 5]");
}