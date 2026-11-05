# OVER
AST-based interpreted programming language implemented in Rust and Python.
Rust implementation is the main and more advanced one.
OVER's main goal is to be simple and understandable for beginners, while also helping me understand how to build a programming language from scratch.

## usage

### clone
- git clone https://github.com/renizex/OVER.git
- cd OVER

### REPL
cargo run --bin repl

### IDE
cargo run --bin ide main.over

## features

### operations
- basic operators: '+', '-', '/', '*',
- assignment: '=',
- comparison operators: '>', '<', "==", "!=", "<=", ">=",
- parentheses.

### variables
- variable assignment, reassignment,
- variables can store function references, input() results, classes objects and every type.

### functions
- buitins: "print()", "input()",
- list methods: "pop()", "append()",
- user defined functions and function calls,
- local scope,
- recursion,
- control flow: "if", "else", "while", "loop", "return", "break", "continue".

### classes
- creating class instances with user defined values,
- attributes,
- object creation and manipulation.

### types
- number
- string
- list
- bool
- object

### custom errors
- syntax errors highlighted with a cursor,
- useful and short explanations.

## REPL examples
- "print(2 + 2)" -> 4,
- "x = 1000 print(x)" -> 1000,
- "x = 100 if x > 50 {print(500)} else {print(666)}" -> 500,
- "function func(x, y) {if x > y {return x + y} else {return x - y}} print(func(10, 5))" -> 15,
- "class Player{health damage resistance} player = Player{100 20 1} print(player.health - 10)" -> 90,
- "x = [1, 2, [3, 4, [4, 5]]] x[2].append(50) print(x[2][2].pop() + 95)" -> 100.

## IDE example
function factorial(n) {
    result = 1
    while n > 1 {
        result = result * n
        n = n - 1
    }
    return result
}

x = 5
print(factorial(x))

-> 120

## planned
- user defined methods,
- more builtin methods like len(),
- russian language support,
- hashmaps (maybe).

## status
active development.

Python implementation is no longer in development, though it still available.
check OVER/python/README.md if you interested.
