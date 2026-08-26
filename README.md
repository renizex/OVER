# OVER
AST-based interpreted programming language written in Python.

## requirements
Python 3.11+

## usage

### clone
git clone https://github.com/renizex/OVER.git
cd OVER

### CLI
python -m over.main

### IDE
python IDE/ide.py

## features

### operations
- basic operators: '+', '-', '*', '/',
- power: '^', assign: '=', modulo: '%',
- comparison operators: '>', '<', '==', '<=', '>=',
- parentheses.

### variables
- variable assignment, reassignment,
- variables can store function references.

### functions
- user-defined functions,
- function calls,
- recursion,
- if, else, while, return statements,
- local scope.

### custom errors
- syntax errors are being highlighted with a cursor,
- useful explanations.

## CLI examples
'print(2 + 2)' -> 4,
'print(6 + 3 ^ 2 / 3)' -> 9,
'print(2+3*4)' -> 14,
'print((2+3) * 4)' -> 20,
'x = 500' -> memory: x: 500,
'x = 1000 if x > 100 {print(x + 100)}' -> 1100,
'function func(x, y) {if x > y {return x + y} else {return x - y}} print(func(10, 5))' -> 15.

## IDE examples
"""
число = 500

функция удвоить(число) {
return число * 2
}
""" -> 1000

"""
x = 5
y = 3

function power(x, y) {
return x ^ y
}

z = power(x, y)
return z
""" -> 125

## IDE
a small custom IDE made on Tkinter.

### features
- saving files with custom extension '.over',
- opening them,
- custom keybinds.

### keybinds
- ctrl c - copy,
- ctrl v - paste,
- ctrl x - cut text,
- ctrl r - run,
- ctrl a - select all,
- ctrl d - delete all,
- ctrl s - save file,
- ctrl o - open new file.

## planned
- improving error UX,
- lists [],
- string type.

## status
active development.
