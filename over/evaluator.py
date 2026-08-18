import over.nodes as nodes
from over.typing_utils import Number, Memory
from over.exceptions import InvalidExpressionError, ReturnStatement
import over.operators as operators
from dataclasses import dataclass

@dataclass
class Evaluator:
    count_cycles: int = 0

class Scope:
    def __init__(self, local, parent):
        self.local = local
        self.parent = parent

    def __getitem__(self, key: str) -> Number:
        if key in self.local:
            return self.local[key]
        return self.parent[key]

    def __setitem__(self, key: str, value: Number) -> None:
        self.local[key] = value

    def __contains__(self, key: str) -> bool:
        return key in self.local or key in self.parent

evaluator = Evaluator()

def evaluate(node: nodes.Node, memory: Memory | Scope, functions: dict[str, nodes.FunctionNode]) -> Number | None:
    call_stack = []
    max_cycles = 10000
    match node:
        case nodes.AssignNode():
            assign(node, memory, functions)
            return None
        case nodes.NumberNode() | nodes.VariableNode():
            return resolve_operand(node, memory)
        case nodes.UnaryMinusNode():
            value = evaluate(node.operand, memory, functions)
            if value is None:
                raise InvalidExpressionError(f"ERROR: expected number, got type None.")
            return operators.unary_minus(value)
        case nodes.BinaryOperatorNode():
            left = evaluate(node.left, memory, functions)
            right = evaluate(node.right, memory, functions)
            if left is None or right is None:
                raise InvalidExpressionError(f"ERROR: operator '{node.operator}' requires two valid numbers, got {type(left).__name__} and {type(right).__name__}.")
            if node.operator in operators.operations or node.operator in operators.comparison:
                if node.operator in operators.operations:
                    return operators.operations[node.operator](left, right)
                else:
                    return operators.comparison[node.operator](left, right)
            else:
                raise InvalidExpressionError(f"ERROR unexpected operator '{node.operator}'.")
        case nodes.IfNode():
            if not evaluate(node.condition, memory, functions):
                if node.else_body is not None:
                    return evaluate(node.else_body, memory, functions)
                return None
            return evaluate(node.body, memory, functions)
        case nodes.WhileNode():
            evaluator.count_cycles = 0
            result = None
            while True:
                condition_result = evaluate(node.condition, memory, functions)
                if not condition_result:
                    break
                result = evaluate(node.body, memory, functions)
                evaluator.count_cycles += 1
                if evaluator.count_cycles > max_cycles:
                    raise InvalidExpressionError(f"ERROR: execution limit exceeded")
            if node.else_body is not None:
                return evaluate(node.else_body, memory, functions)
            return result
        case nodes.FunctionNode():
            functions[node.name] = node
            return None
        case nodes.CallNode():
            try:
                call_stack.append(node.name)
                arguments = [evaluate(arg, memory, functions) for arg in node.args]
                if node.name.value not in functions:
                    raise InvalidExpressionError(f"ERROR: function '{node.name.value}' does not exist.")
                variables = [var.value for var in functions[node.name.value].args]
                body = functions[node.name].body
                try:
                    local_memory = dict(zip(variables, arguments, strict=True))
                except ValueError:
                    raise InvalidExpressionError(f"ERROR: expected {len(variables)} arguments, got {len(arguments)}.")
                scope = Scope(local_memory, memory)
                try:
                    return evaluate(body, scope, functions)
                except ReturnStatement as result:
                    return result.expression
            finally:
                call_stack.pop()
        case nodes.ReturnNode():
            result = None
            if node.expression is not None:
                result = evaluate(node.expression, memory, functions)
            raise ReturnStatement(result)
        case nodes.BlockNode():
            result = None
            for block in node.block:
                result = evaluate(block, memory, functions)
            return result
    raise InvalidExpressionError(f"ERROR: unsupported AST node.\nnode: {node}")

def assign(node: nodes.AssignNode, memory: Memory | Scope, functions: dict[str, nodes.FunctionNode]) -> None:
    value = evaluate(node.right, memory, functions)
    if value is None:
        raise InvalidExpressionError(f"ERROR: unexpected None type.")
    memory[node.variable.value] = value
    return None

def resolve_operand(node: nodes.Node, memory: Memory | Scope) -> Number:
    match node:
        case nodes.NumberNode():
            return node.value
        case nodes.VariableNode(variable):
            if variable in memory:
                return memory[variable]
            raise InvalidExpressionError(f"ERROR: variable '{variable}' does not exist.")
        case _:
            raise InvalidExpressionError(f"ERROR: can't resolve operand for type '{type(node).__name__}'.")