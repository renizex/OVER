import over.nodes as nodes
from over.typing_utils import Number, Memory
from over.exceptions import InvalidExpressionError, ReturnStatement
import over.operators as operators
from over.builtins import builtins

def interpret(node):
    evaluator = Evaluator()
    evaluator.evaluate(node)

class Evaluator:
    def __init__(self):
        self.builtins = builtins
        self.functions: dict[str, nodes.FunctionNode] = {}
        self.memory: Memory | Scope = {}
        self.scope = self.memory

    def block_node(self, node) -> Number | None:
        result = None
        for block in node.block:
            result = self.evaluate(block)
        return result

    def return_node(self, node) -> Number | None:
        result = None
        if node.expression is not None:
            result = self.evaluate(node.expression)
        raise ReturnStatement(result)

    def unary_minus_node(self, node) -> Number | None:
        value = self.evaluate(node.operand)
        if value is None:
            raise InvalidExpressionError(f"ERROR: expected number, got type None.")
        return operators.unary_minus(value)

    def binary_operator_node(self, node) -> Number | None:
        left = self.evaluate(node.left)
        right = self.evaluate(node.right)
        if left is None or right is None:
            raise InvalidExpressionError(f"ERROR: operator '{node.operator}' requires two valid numbers, got {type(left).__name__} and {type(right).__name__}.")
        if node.operator in operators.operations or node.operator in operators.comparison:
            if node.operator in operators.operations:
                return operators.operations[node.operator](left, right)
            else:
                return operators.comparison[node.operator](left, right)
        else:
            raise InvalidExpressionError(f"ERROR unexpected operator '{node.operator}'.")

    def if_node(self, node) -> Number | None:
        if not self.evaluate(node.condition):
            if node.else_body is not None:
                return self.evaluate(node.else_body)
            return None
        return self.evaluate(node.body)

    def while_node(self, node) -> Number | None:
        result = None
        while True:
            condition_result = self.evaluate(node.condition)
            if not condition_result:
                break
            result = self.evaluate(node.body)
        if node.else_body is not None:
            return self.evaluate(node.else_body)
        return result

    def create_scope(self, variables, arguments):
        try:
            local_memory = dict(zip(variables, arguments, strict=True))
        except ValueError:
            raise InvalidExpressionError(f"ERROR: expected {len(variables)} arguments, got {len(arguments)}.")
        previous_scope = self.scope
        self.scope = Scope(local_memory, previous_scope)
        return previous_scope

    def call_node(self, node) -> Number | None:
        arguments = [self.evaluate(arg) for arg in node.args]
        function = self.resolve_function(node)
        if isinstance(function, nodes.BuiltinCallNode):
            builtins[function.name](*arguments)
            return None
        variables = [var.value for var in self.functions[function.name].args]
        if not isinstance(node.name, nodes.VariableNode):
            raise InvalidExpressionError(f"ERROR: invalid function call '{node.name.value}'.")
        body = self.functions[function.name].body
        previous_scope = self.create_scope(variables, arguments)
        try:
            return self.evaluate(body)
        except ReturnStatement as result:
            return result.expression
        finally:
            self.scope = previous_scope

    def evaluate(self, node: nodes.Node) -> Number | None:
        match node:
            case nodes.AssignNode():
                self.assign(node)
                return None
            case nodes.NumberNode() | nodes.VariableNode():
                return self.resolve_operand(node)
            case nodes.UnaryMinusNode():
                return self.unary_minus_node(node)
            case nodes.BinaryOperatorNode():
                return self.binary_operator_node(node)
            case nodes.IfNode():
                return self.if_node(node)
            case nodes.WhileNode():
                return self.while_node(node)
            case nodes.FunctionNode():
                self.functions[node.name] = node
                return None
            case nodes.CallNode():
                return self.call_node(node)
            case nodes.ReturnNode():
                return self.return_node(node)
            case nodes.BlockNode():
                return self.block_node(node)
        raise InvalidExpressionError(f"ERROR: unsupported AST node.\nnode: {node}")

    def assign(self, node: nodes.AssignNode) -> None:
        value = self.evaluate(node.right)
        if value is None:
            raise InvalidExpressionError(f"ERROR: unexpected None type.")
        self.scope[node.variable.value] = value
        return None

    def resolve_operand(self, node: nodes.Node) -> Number:
        match node:
            case nodes.NumberNode():
                return node.value
            case nodes.VariableNode(variable):
                if variable in self.scope:
                    return self.scope[variable]
                raise InvalidExpressionError(f"ERROR: variable '{variable}' does not exist.")
            case _:
                raise InvalidExpressionError(f"ERROR: can't resolve operand for type '{type(node).__name__}'.")

    def resolve_function(self, node: nodes.CallNode) -> nodes.BuiltinCallNode | nodes.UserCallNode:
        if node.name.value in builtins:
            return nodes.BuiltinCallNode(node.name.value, node.args)
        elif node.name.value not in self.functions:
            raise InvalidExpressionError(f"ERROR: function '{node.name.value}' does not exist.")
        return nodes.UserCallNode(node.name.value, node.args)

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