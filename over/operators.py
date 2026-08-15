from over.typing_utils import Number
from over.exceptions import DivisionByZeroError

def plus(a: Number, b: Number) -> Number:
    return a+b

def minus(a: Number, b: Number) -> Number:
    return a-b

def multiply(a: Number, b: Number) -> Number:
    return a*b

def divide(a: Number, b: Number) -> Number:
    if b == 0:
        raise DivisionByZeroError("ERROR: division by zero")
    return a/b

def power(a: Number, b: Number) -> Number:
    if a == 0 and b < 0:
        raise DivisionByZeroError("ERROR: division by zero")
    return a**b

def equal(a: Number, b: Number) -> bool:
    return a == b

def less_equal(a: Number, b: Number) -> bool:
    return a <= b

def greater_equal(a: Number, b: Number) -> bool:
    return a >= b

def greater(a: Number, b: Number) -> bool:
    return a > b

def less(a: Number, b: Number) -> bool:
    return a < b

def mod(a: Number, b: Number) -> Number:
    return a % b

def unary_minus(a: Number) -> Number:
    return -a

operations = {
    '+': plus,
    '-': minus,
    '*': multiply,
    '/': divide,
    '^': power,
    '%': mod
}

comparison = {
    '<=': less_equal,
    '>=': greater_equal,
    '==': equal,
    '>': greater,
    '<': less
}