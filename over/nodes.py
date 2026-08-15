from dataclasses import dataclass
from over.typing_utils import Number

@dataclass
class Node:
    pass

@dataclass
class BinaryOperatorNode(Node):
    left: Node
    operator: str
    right: Node

@dataclass
class NumberNode(Node):
    value: Number

@dataclass
class VariableNode(Node):
    value: str

@dataclass
class UnaryMinusNode(Node):
    operand: Node

@dataclass
class AssignNode(Node):
    variable: VariableNode
    operator: str
    right: Node

@dataclass
class BlockNode(Node):
    block: list[Node]

@dataclass
class IfNode(Node):
    condition: Node
    body: BlockNode
    else_body: BlockNode | None

@dataclass
class WhileNode(Node):
    condition: Node
    body: BlockNode
    else_body: BlockNode | None

@dataclass
class FunctionNode(Node):
    name: str
    args: list[VariableNode]
    body: BlockNode

@dataclass
class CallNode(Node):
    name: str
    args: list[Node]

@dataclass
class ReturnNode(Node):
    expression: Node | None