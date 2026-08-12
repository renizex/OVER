import over.nodes as nodes
from over.typing_utils import Number, Memory
from over.exceptions import InvalidExpressionError
import over.operators as operators
from dataclasses import dataclass

@dataclass
class Evaluator:
    count_cycles: int = 0

evaluator = Evaluator()

def evaluate(node: nodes.Node, memory: Memory) -> Number | None:
    max_cycles = 10000
    match node:
        case nodes.AssignNode():
            assign(node, memory)
            return None
        case nodes.NumberNode() | nodes.VariableNode():
            return resolve_operand(node, memory)
        case nodes.UnaryMinusNode():
            value = evaluate(node.operand, memory)
            if value is None:
                raise InvalidExpressionError(f"ERROR: expected number, got type None.")
            return operators.unary_minus(value)
        case nodes.BinaryOperatorNode():
            left = evaluate(node.left, memory)
            right = evaluate(node.right, memory)
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
            if not evaluate(node.condition, memory):
                if node.else_body is not None:
                    return evaluate(node.else_body, memory)
                return None
            return evaluate(node.body, memory)
        case nodes.WhileNode():
            evaluator.count_cycles = 0
            result = None
            while True:
                condition_result = evaluate(node.condition, memory)
                if not condition_result:
                    break
                result = evaluate(node.body, memory)
                evaluator.count_cycles += 1
                if evaluator.count_cycles > max_cycles:
                    raise InvalidExpressionError(f"ERROR: execution limit exceeded")
            if node.else_body is not None:
                return evaluate(node.else_body, memory)
            return result
        case nodes.BlockNode():
            result = None
            for block in node.block:
                result = evaluate(block, memory)
            return result
    raise InvalidExpressionError("ERROR: unsupported AST node.")

def assign(node: nodes.AssignNode, memory: Memory) -> None:
    value = evaluate(node.right, memory)
    if value is None:
        raise InvalidExpressionError(f"ERROR: unexpected None type.")
    memory[node.variable.value] = value
    return None

def resolve_operand(node: nodes.Node, memory: Memory) -> Number:
    match node:
        case nodes.NumberNode():
            return node.value
        case nodes.VariableNode(variable):
            if variable in memory:
                return memory[variable]
            raise InvalidExpressionError(f"ERROR: variable '{variable}' does not exist.")
        case _:
            raise InvalidExpressionError(f"ERROR: can't resolve operand for type '{type(node).__name__}'.")
