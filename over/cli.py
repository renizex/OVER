from over.typing_utils import Number
from over.exceptions import InvalidExpressionError, ReturnStatement
from over.lexer import lex
from over.parser import parse
from over.interpreter import interpret

def memory_show(memory: dict[str, Number]) -> str:
    if not memory:
        return "memory is empty"
    return '\n'.join(f"{variable} = {number}" for variable, number in memory.items())

def memory_clear(memory: dict[str, Number]) -> str:
    memory.clear()
    return "memory cleared"

def help_show() -> str:
    return """
commands:
    help: show this

operators:
    basic:
        '+', '-', '*', '/' including unary minus
    '^': power
    '=': assign
    '(', ')': parentheses
    'if', 'else', '<', '>', '=='
    example:
        > x = 5
        > y = 6
        > if x > y {x+y} else {x-y}
        output: 
            -1

planned:
    functions.
    """.strip()

help_commands = {
    "help": help_show
}

def main() -> None:
    print("OVER")
    print("enter 'help' for commands and operators")
    while True:
        try:
            expression = input("> ")
            if check_expression(expression):
                continue
            tokens = lex(expression)
            node = parse(tokens, expression)
            result = interpret(node, expression)
            if result is not None:
                raise InvalidExpressionError(f"ERROR: this expression is invalid.\nuse 'print({expression})'.")
        except InvalidExpressionError as msg:
            print(msg)
        except ReturnStatement as result:
            print(f"ERROR: this expression is invalid.\nuse 'print({result.expression})'.")

def check_expression(expression: str) -> bool:
    if is_command(expression):
        return True
    if expression.strip() == '':
        raise InvalidExpressionError(f"ERROR: empty input.")
    return False

def is_command(expression: str) -> bool:
    if expression in help_commands:
        print(help_commands[expression]())
        return True
    return False