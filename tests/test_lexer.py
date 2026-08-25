import pytest
from over.lexer import lex
from over.tokens import *
from over.exceptions import InvalidLexemeError

@pytest.mark.parametrize(
    "source, expected",
    [
        ("2 + 2", [NumberToken(value=2, position=0), BinaryOperatorToken(value='+', position=2), NumberToken(value=2, position=4)]),
        ("2 + 3 * 4 / 5", [NumberToken(value=2, position=0), BinaryOperatorToken(value='+', position=2), NumberToken(value=3, position=4), BinaryOperatorToken(value='*', position=6), NumberToken(value=4, position=8), BinaryOperatorToken(value='/', position=10), NumberToken(value=5, position=12)]),
        ("2 ^ 5", [NumberToken(value=2, position=0), BinaryOperatorToken(value='^', position=2), NumberToken(value=5, position=4)]),
        ("x = 5 % 51", [VariableToken(value='x', position=0), AssignToken(value='=', position=2), NumberToken(value=5, position=4), BinaryOperatorToken(value='%', position=6), NumberToken(value=51, position=8)]),
        ("x >= 10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='>=', position=2), NumberToken(value=10, position=5)]),
        ("x <= 10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='<=', position=2), NumberToken(value=10, position=5)]),
        ("x > 10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='>', position=2), NumberToken(value=10, position=4)]),
        ("x < 10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='<', position=2), NumberToken(value=10, position=4)]),
        ("x == 10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='==', position=2), NumberToken(value=10, position=5)]),
        ("2+3", [NumberToken(value=2, position=0), BinaryOperatorToken(value='+', position=1), NumberToken(value=3, position=2)]),
        ("x>=10",[VariableToken(value='x', position=0), BinaryOperatorToken(value='>=', position=1), NumberToken(value=10, position=3)] ),
        ("x<=10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='<=', position=1), NumberToken(value=10, position=3)]),
        ("x==10", [VariableToken(value='x', position=0), BinaryOperatorToken(value='==', position=1), NumberToken(value=10, position=3)]),
        ("x=5", [VariableToken(value='x', position=0), AssignToken(value='=', position=1), NumberToken(value=5, position=2)]),
        ("-123", [BinaryOperatorToken(value='-', position=0), NumberToken(value=123, position=1)]),
        ("2 + -5", [NumberToken(value=2, position=0), BinaryOperatorToken(value='+', position=2), BinaryOperatorToken(value='-', position=4), NumberToken(value=5, position=5)]),
        ("x", [VariableToken(value='x', position=0)]),
        ("hello", [VariableToken(value='hello', position=0)]),
        ("привет", [VariableToken(value='привет', position=0)]),
        ("_value", [VariableToken(value='_value', position=0)]),
        ("_всемприветребята", [VariableToken(value='_всемприветребята', position=0)]),
        ("var123", [VariableToken(value='var123', position=0)]),
        ("_123", [VariableToken(value='_123', position=0)]),
        ("if x > y {return x + y} else {return x - y}", [IfToken(value='if', position=0), VariableToken(value='x', position=3), BinaryOperatorToken(value='>', position=5), VariableToken(value='y', position=7), OpeningBraceToken(value='{', position=9), ReturnToken(value='return', position=10), VariableToken(value='x', position=17), BinaryOperatorToken(value='+', position=19), VariableToken(value='y', position=21), ClosingBraceToken(value='}', position=22), ElseToken(value='else', position=24), OpeningBraceToken(value='{', position=29), ReturnToken(value='return', position=30), VariableToken(value='x', position=37), BinaryOperatorToken(value='-', position=39), VariableToken(value='y', position=41), ClosingBraceToken(value='}', position=42)]),
        ("function func(x) {while x > 100 {x = x + 1} return x}", [FunctionToken(value='function', position=0), VariableToken(value='func', position=9), OpeningParenthesisToken(value='(', position=13), VariableToken(value='x', position=14), ClosingParenthesisToken(value=')', position=15), OpeningBraceToken(value='{', position=17), WhileToken(value='while', position=18), VariableToken(value='x', position=24), BinaryOperatorToken(value='>', position=26), NumberToken(value=100, position=28), OpeningBraceToken(value='{', position=32), VariableToken(value='x', position=33), AssignToken(value='=', position=35), VariableToken(value='x', position=37), BinaryOperatorToken(value='+', position=39), NumberToken(value=1, position=41), ClosingBraceToken(value='}', position=42), ReturnToken(value='return', position=44), VariableToken(value='x', position=51), ClosingBraceToken(value='}', position=52)]),
        ("функция функ(б) {пока б > 100 {б = б + 1} вернуть б}", [FunctionToken(value='функция', position=0), VariableToken(value='функ', position=8), OpeningParenthesisToken(value='(', position=12), VariableToken(value='б', position=13), ClosingParenthesisToken(value=')', position=14), OpeningBraceToken(value='{', position=16), WhileToken(value='пока', position=17), VariableToken(value='б', position=22), BinaryOperatorToken(value='>', position=24), NumberToken(value=100, position=26), OpeningBraceToken(value='{', position=30), VariableToken(value='б', position=31), AssignToken(value='=', position=33), VariableToken(value='б', position=35), BinaryOperatorToken(value='+', position=37), NumberToken(value=1, position=39), ClosingBraceToken(value='}', position=40), ReturnToken(value='вернуть', position=42), VariableToken(value='б', position=50), ClosingBraceToken(value='}', position=51)]),
        ("if", [IfToken(value='if', position=0)]),
        ("else", [ElseToken(value='else', position=0)]),
        ("while", [WhileToken(value='while', position=0)]),
        ("function", [FunctionToken(value='function', position=0)]),
        ("return", [ReturnToken(value='return', position=0)]),
        ("если", [IfToken(value='если', position=0)]),
        ("иначе", [ElseToken(value='иначе', position=0)]),
        ("пока", [WhileToken(value='пока', position=0)]),
        ("функция", [FunctionToken(value='функция', position=0)]),
        ("вернуть", [ReturnToken(value='вернуть', position=0)]),
        ("(2 + 3)", [OpeningParenthesisToken(value='(', position=0), NumberToken(value=2, position=1), BinaryOperatorToken(value='+', position=3), NumberToken(value=3, position=5), ClosingParenthesisToken(value=')', position=6)]),
        ("{ x = 5 }", [OpeningBraceToken(value='{', position=0), VariableToken(value='x', position=2), AssignToken(value='=', position=4), NumberToken(value=5, position=6), ClosingBraceToken(value='}', position=8)]),
        ("if x {y}", [IfToken(value='if', position=0), VariableToken(value='x', position=3), OpeningBraceToken(value='{', position=5), VariableToken(value='y', position=6), ClosingBraceToken(value='}', position=7)]),
        ("func(x)", [VariableToken(value='func', position=0), OpeningParenthesisToken(value='(', position=4), VariableToken(value='x', position=5), ClosingParenthesisToken(value=')', position=6)]),
        ("если x > 0 {if y > 0 {вернуть x}}", [IfToken(value='если', position=0), VariableToken(value='x', position=5), BinaryOperatorToken(value='>', position=7), NumberToken(value=0, position=9), OpeningBraceToken(value='{', position=11), IfToken(value='if', position=12), VariableToken(value='y', position=15), BinaryOperatorToken(value='>', position=17), NumberToken(value=0, position=19), OpeningBraceToken(value='{', position=21), ReturnToken(value='вернуть', position=22), VariableToken(value='x', position=30), ClosingBraceToken(value='}', position=31), ClosingBraceToken(value='}', position=32)]),
        ("2    +    3", [NumberToken(value=2, position=0), BinaryOperatorToken(value='+', position=5), NumberToken(value=3, position=10)]),
        ("  2 + 3  ", [NumberToken(value=2, position=2), BinaryOperatorToken(value='+', position=4), NumberToken(value=3, position=6)]),
        ("0", [NumberToken(value=0, position=0)]),
        ("123", [NumberToken(value=123, position=0)]),
        ("999999", [NumberToken(value=999999, position=0)]),
        ("3.14", [NumberToken(value=3.14, position=0)]),
        ("0.5", [NumberToken(value=0.5, position=0)]),
        ("123.456", [NumberToken(value=123.456, position=0)]),
        ("01", [NumberToken(value=1, position=0)]),
        ("00", [NumberToken(value=0, position=0)]),
        ("1.0", [NumberToken(value=1.0, position=0)]),
        ("0.0", [NumberToken(value=0.0, position=0)]),
        ("10.01", [NumberToken(value=10.01, position=0)]),
        ("+ - * / % ^ = == > < >= <=", [BinaryOperatorToken(value='+', position=0), BinaryOperatorToken(value='-', position=2), BinaryOperatorToken(value='*', position=4), BinaryOperatorToken(value='/', position=6), BinaryOperatorToken(value='%', position=8), BinaryOperatorToken(value='^', position=10), AssignToken(value='=', position=12), BinaryOperatorToken(value='==', position=14), BinaryOperatorToken(value='>', position=17), BinaryOperatorToken(value='<', position=19), BinaryOperatorToken(value='>=', position=21), BinaryOperatorToken(value='<=', position=24)]),
        ("""
        if x > 0 {
            return x
            }
        """, [IfToken(value='if', position=9), VariableToken(value='x', position=12), BinaryOperatorToken(value='>', position=14), NumberToken(value=0, position=16), OpeningBraceToken(value='{', position=18), ReturnToken(value='return', position=32), VariableToken(value='x', position=39), ClosingBraceToken(value='}', position=53)])
    ]
)
def test_lexer(source, expected):
    assert lex(source) == expected

@pytest.mark.parametrize(
    "source",
    [
        "2 @ 3",
        "x # 5",
        "a & b",
        "x : 10",
    ]
)
def test_invalid(source):
    with pytest.raises(InvalidLexemeError):
        lex(source)