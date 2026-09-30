use crate::tokens::{Token};

#[derive(Clone, Debug)]
pub enum Node {
    Number(f64, usize, usize),
    Identifier(String, usize, usize),
    String(String, usize, usize),
    Bool(bool, usize, usize),
    Binary(Box<Node>, Token, Box<Node>),
    UnaryMinus(Box<Node>),
    Assignment(String, Box<Node>, usize, usize),
    If(Box<Node>, Box<Node>, Option<Box<Node>>, usize, usize),
    While(Box<Node>, Box<Node>, Option<Box<Node>>, usize, usize),
    Loop(Box<Node>, usize, usize),
    Break(usize, usize),
    Block(Vec<Node>, usize, usize),
    Continue(usize, usize),
    Function(Box<Node>, Vec<Node>, Box<Node>, usize, usize),
    Call(Box<Node>, Vec<Node>, usize, usize),
    Return(Option<Box<Node>>, usize, usize),
}

impl Node {
    pub fn info(&self) -> (usize, usize) {
        match self {
            Node::Number(_, start, end) => (*start, *end),
            Node::Identifier(_, start, end) => (*start, *end),
            Node::String(_, start, end) => (*start, *end),
            Node::Bool(_, start, end) => (*start, *end),
            Node::Binary(start_node, _, end_node) => {
                let (start, _) = start_node.info();
                let (_, end) = end_node.info();
                (start, end)
            },
            Node::UnaryMinus(node) => node.info(),
            Node::Assignment(_, _, start, end) => (*start, *end),
            Node::If(_, _, _, start, end) => (*start, *end),
            Node::While(_, _, _, start, end) => (*start, *end),
            Node::Loop(_,  start, end) => (*start, *end),
            Node::Break(start, end) => (*start, *end),
            Node::Block(_, start, end) => (*start, *end),
            Node::Continue(start, end) => (*start, *end),
            Node::Function(_, _, _, start, end) => (*start, *end),
            Node::Call(_, _, start, end) => (*start, *end),
            Node::Return(_, start, end) => (*start, *end),
        }
    }

    pub fn value(&self) -> String {
        match self {
            Node::Number(value, _, _) => value.to_string(),
            Node::Identifier(value, _, _) => value.clone(),
            Node::String(value, _, _) => value.clone(),
            Node::Bool(value, _, _) => value.to_string(),
            Node::Binary(start_node, operator, end_node) => {let start = start_node.value(); let end = end_node.value(); start + &operator.value() + &end},
            Node::UnaryMinus(node) => "-".to_owned() + &node.value(),
            Node::Assignment(start, end, _, _) => start.to_owned() + "=" + &end.value(),
            Node::If(condition, body, _, _, _) => {"if ".to_owned() + &condition.value() + " {\n" + &body.value() + "\n" + "}"}
            Node::While(condition, body, _, _, _) => {"while ".to_owned() + &condition.value() + " {\n" + &body.value() + "\n" + "}"}
            Node::Loop(body, _, _) => {"loop ".to_owned() + " {\n" + &body.value() + "\n" + "}"}
            Node::Break(_, _) => String::from("break"),
            Node::Continue(_, _) => String::from("continue"),
            Node::Block(nodes, _, _) => {
                let mut result = String::new();
                result.push('{');
                for node in nodes {
                    result.push_str(&(node.value() + " "))
                }
                result.push('}');
                result
            }
            Node::Function(identifier, _, body, _, _) => {identifier.value() + "(...)" + " {\n" + &*body.value() + " \n}"}
            Node::Call(identifier, _, _, _) => {identifier.value() + "(...)" + " {\n" + " \n}"}
            Node::Return(_, _, _) => String::from("return")
        }
    }
}