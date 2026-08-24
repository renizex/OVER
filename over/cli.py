from over.typing_utils import Number, Memory
from over.exceptions import InvalidExpressionError
from over.lexer import lex
from over.parser import parse
from over.evaluator import evaluate
import over.nodes as nodes

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
    memory: show memory
    clear: clear memory
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

memory_commands = {
    "memory": memory_show,
    "clear": memory_clear
}

help_commands = {
    "help": help_show
}

def main() -> None:
    memory: dict[str, Number] = {}
    functions: dict[str, nodes.FunctionNode] = {}
    print("AST evaluator")
    print("enter 'help' for commands and operators")
    while True:
        try:
            expression = input("> ")
            if check_expression(expression, memory):
                continue
            tokens = lex(expression)
            node = parse(tokens, expression)
            result = evaluate(node, memory, functions)
            if result is not None:
                raise InvalidExpressionError(f"ERROR: this expression is invalid.\nuse 'print({expression})'.")
        except InvalidExpressionError as msg:
            print(msg)

def check_expression(expression: str, memory: Memory) -> bool:
    if is_command(expression, memory):
        return True
    if expression.strip() == '':
        raise InvalidExpressionError(f"ERROR: empty input.")
    return False

def is_command(expression: str, memory: Memory) -> bool:
    if expression in memory_commands:
        print(memory_commands[expression](memory))
        return True
    elif expression in help_commands:
        print(help_commands[expression]())
        return True
    return False