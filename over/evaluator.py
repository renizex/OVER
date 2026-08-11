import OVER.nodes as nodes
from OVER.typing_utils import Number, Memory
from OVER.exceptions import InvalidExpressionError
import OVER.operators as operators

def evaluate(node: nodes.Node, memory: Memory) -> Number | None:
    count_blocks = 0
    count_cycles = 0
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
            if node.operator in operators.operations:
                return operators.operations[node.operator](left, right)
            else:
                return operators.comparison[node.operator](left, right)
        case nodes.IfNode():
            if not evaluate(node.condition, memory):
                if node.else_body is not None:
                    return evaluate(node.else_body, memory)
                return None
            return evaluate(node.body, memory)
        case nodes.WhileNode():
            result = None
            while evaluate(node.condition, memory):
                count_cycles += 1
                if count_cycles > 10000:
                    raise InvalidExpressionError(f"ERROR: execution limit exceeded")
                result = evaluate(node.body, memory)
            if not evaluate(node.condition, memory):
                if node.else_body is not None:
                    return evaluate(node.else_body, memory)
                return None
            return result
        case nodes.BlockNode():
            result = None
            for block in node.block:
                result = evaluate(block, memory)
                count_blocks += 1
                if count_blocks > 1000:
                    raise InvalidExpressionError(f"ERROR: too big expression")
            return result
    raise InvalidExpressionError("ERROR: unsupported AST node.")

def assign(node: nodes.AssignNode, memory: dict[str, Number]) -> None:
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