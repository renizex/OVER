from dataclasses import dataclass
from over.typing_utils import Number

@dataclass
class Token:
    value: str
    position: int
    end: int

@dataclass
class BinaryOperatorToken(Token):
    pass

@dataclass
class AssignToken(Token):
    pass

@dataclass
class OpeningParenthesisToken(Token):
    pass

@dataclass
class ClosingParenthesisToken(Token):
    pass

@dataclass
class OpeningBraceToken(Token):
    pass

@dataclass
class ClosingBraceToken(Token):
    pass

@dataclass
class NumberToken(Token):
    value: Number

@dataclass
class VariableToken(Token):
    pass

@dataclass
class IfToken(Token):
    pass

@dataclass
class ElseToken(Token):
    pass

@dataclass
class WhileToken(Token):
    pass

@dataclass
class FunctionToken(Token):
    pass

@dataclass
class ContinueArgsToken(Token):
    pass

@dataclass
class ReturnToken(Token):
    pass