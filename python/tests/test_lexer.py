import pytest
from over.lexer import lex
from over.tokens import *
from over.exceptions import InvalidLexemeError

@pytest.mark.parametrize(
    "source, expected",
    [
        ("2 + 2", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='+', position=2, end=3), NumberToken(value=2, position=4, end=5)]),
        ("2 + 3 * 4 / 5", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='+', position=2, end=3), NumberToken(value=3, position=4, end=5), BinaryOperatorToken(value='*', position=6, end=7), NumberToken(value=4, position=8, end=9), BinaryOperatorToken(value='/', position=10, end=11), NumberToken(value=5, position=12, end=13)]),
        ("2 ^ 5", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='^', position=2, end=3), NumberToken(value=5, position=4, end=5)]),
        ("x = 5 % 51", [VariableToken(value='x', position=0, end=1), AssignToken(value='=', position=2, end=3), NumberToken(value=5, position=4, end=5), BinaryOperatorToken(value='%', position=6, end=7), NumberToken(value=51, position=8, end=10)]),
        ("x >= 10", [VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='>=', position=2, end=4), NumberToken(value=10, position=5, end=7)]),
        ("x <= 10", [VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='<=', position=2, end=4), NumberToken(value=10, position=5, end=7)]),
        ("x > 10", [VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='>', position=2, end=3), NumberToken(value=10, position=4, end=6)]),
        ("x < 10", [VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='<', position=2, end=3), NumberToken(value=10, position=4, end=6)]),
        ("x == 10", [VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='==', position=2, end=4), NumberToken(value=10, position=5, end=7)]),
        ("2+3", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='+', position=1, end=2), NumberToken(value=3, position=2, end=3)]),
        ("x>=10",[VariableToken(value='x', position=0, end=1), BinaryOperatorToken(value='>=', position=1, end=3), NumberToken(value=10, position=3, end=5)]),
        ("-123", [BinaryOperatorToken(value='-', position=0, end=1), NumberToken(value=123, position=1, end=4)]),
        ("2 + -5", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='+', position=2, end=3), BinaryOperatorToken(value='-', position=4, end=5), NumberToken(value=5, position=5, end=6)]),
        ("x", [VariableToken(value='x', position=0, end=1)]),
        ("hello", [VariableToken(value='hello', position=0, end=5)]),
        ("привет", [VariableToken(value='привет', position=0, end=6)]),
        ("_value", [VariableToken(value='_value', position=0, end=6)]),
        ("var123", [VariableToken(value='var123', position=0, end=6)]),
        ("_123", [VariableToken(value='_123', position=0, end=4)]),
        ("if x > y {return x + y} else {return x - y}", [IfToken(value='if', position=0, end=2), VariableToken(value='x', position=3, end=4), BinaryOperatorToken(value='>', position=5, end=6), VariableToken(value='y', position=7, end=8), OpeningBraceToken(value='{', position=9, end=10), ReturnToken(value='return', position=10, end=16), VariableToken(value='x', position=17, end=18), BinaryOperatorToken(value='+', position=19, end=20), VariableToken(value='y', position=21, end=22), ClosingBraceToken(value='}', position=22, end=23), ElseToken(value='else', position=24, end=28), OpeningBraceToken(value='{', position=29, end=30), ReturnToken(value='return', position=30, end=36), VariableToken(value='x', position=37, end=38), BinaryOperatorToken(value='-', position=39, end=40), VariableToken(value='y', position=41, end=42), ClosingBraceToken(value='}', position=42, end=43)]),
        ("function func(x) {while x > 100 {x = x + 1} return x}", [FunctionToken(value='function', position=0, end=8), VariableToken(value='func', position=9, end=13), OpeningParenthesisToken(value='(', position=13, end=14), VariableToken(value='x', position=14, end=15), ClosingParenthesisToken(value=')', position=15, end=16), OpeningBraceToken(value='{', position=17, end=18), WhileToken(value='while', position=18, end=23), VariableToken(value='x', position=24, end=25), BinaryOperatorToken(value='>', position=26, end=27), NumberToken(value=100, position=28, end=31), OpeningBraceToken(value='{', position=32, end=33), VariableToken(value='x', position=33, end=34), AssignToken(value='=', position=35, end=36), VariableToken(value='x', position=37, end=38), BinaryOperatorToken(value='+', position=39, end=40), NumberToken(value=1, position=41, end=42), ClosingBraceToken(value='}', position=42, end=43), ReturnToken(value='return', position=44, end=50), VariableToken(value='x', position=51, end=52), ClosingBraceToken(value='}', position=52, end=53)]),
        ("функция функ(б) {пока б > 100 {б = б + 1} вернуть б}", [FunctionToken(value='функция', position=0, end=7), VariableToken(value='функ', position=8, end=12), OpeningParenthesisToken(value='(', position=12, end=13), VariableToken(value='б', position=13, end=14), ClosingParenthesisToken(value=')', position=14, end=15), OpeningBraceToken(value='{', position=16, end=17), WhileToken(value='пока', position=17, end=21), VariableToken(value='б', position=22, end=23), BinaryOperatorToken(value='>', position=24, end=25), NumberToken(value=100, position=26, end=29), OpeningBraceToken(value='{', position=30, end=31), VariableToken(value='б', position=31, end=32), AssignToken(value='=', position=33, end=34), VariableToken(value='б', position=35, end=36), BinaryOperatorToken(value='+', position=37, end=38), NumberToken(value=1, position=39, end=40), ClosingBraceToken(value='}', position=40, end=41), ReturnToken(value='вернуть', position=42, end=49), VariableToken(value='б', position=50, end=51), ClosingBraceToken(value='}', position=51, end=52)]),
        ("if", [IfToken(value='if', position=0, end=2)]),
        ("else", [ElseToken(value='else', position=0, end=4)]),
        ("while", [WhileToken(value='while', position=0, end=5)]),
        ("function", [FunctionToken(value='function', position=0, end=8)]),
        ("return", [ReturnToken(value='return', position=0, end=6)]),
        ("если", [IfToken(value='если', position=0, end=4)]),
        ("иначе", [ElseToken(value='иначе', position=0, end=5)]),
        ("пока", [WhileToken(value='пока', position=0, end=4)]),
        ("функция", [FunctionToken(value='функция', position=0, end=7)]),
        ("вернуть", [ReturnToken(value='вернуть', position=0, end=7)]),
        ("(2 + 3)", [OpeningParenthesisToken(value='(', position=0, end=1), NumberToken(value=2, position=1, end=2), BinaryOperatorToken(value='+', position=3, end=4), NumberToken(value=3, position=5, end=6), ClosingParenthesisToken(value=')', position=6, end=7)]),
        ("{ x = 5 }", [OpeningBraceToken(value='{', position=0, end=1), VariableToken(value='x', position=2, end=3), AssignToken(value='=', position=4, end=5), NumberToken(value=5, position=6, end=7), ClosingBraceToken(value='}', position=8, end=9)]),
        ("if x {y}", [IfToken(value='if', position=0, end=2), VariableToken(value='x', position=3, end=4), OpeningBraceToken(value='{', position=5, end=6), VariableToken(value='y', position=6, end=7), ClosingBraceToken(value='}', position=7, end=8)]),
        ("func(x)", [VariableToken(value='func', position=0, end=4), OpeningParenthesisToken(value='(', position=4, end=5), VariableToken(value='x', position=5, end=6), ClosingParenthesisToken(value=')', position=6, end=7)]),
        ("если x > 0 {if y > 0 {вернуть x}}",  [IfToken(value='если', position=0, end=4), VariableToken(value='x', position=5, end=6), BinaryOperatorToken(value='>', position=7, end=8), NumberToken(value=0, position=9, end=10), OpeningBraceToken(value='{', position=11, end=12), IfToken(value='if', position=12, end=14), VariableToken(value='y', position=15, end=16), BinaryOperatorToken(value='>', position=17, end=18), NumberToken(value=0, position=19, end=20), OpeningBraceToken(value='{', position=21, end=22), ReturnToken(value='вернуть', position=22, end=29), VariableToken(value='x', position=30, end=31), ClosingBraceToken(value='}', position=31, end=32), ClosingBraceToken(value='}', position=32, end=33)]),
        ("2    +    3", [NumberToken(value=2, position=0, end=1), BinaryOperatorToken(value='+', position=5, end=6), NumberToken(value=3, position=10, end=11)]),
        ("  2 + 3  ", [NumberToken(value=2, position=2, end=3), BinaryOperatorToken(value='+', position=4, end=5), NumberToken(value=3, position=6, end=7)]),
        ("0", [NumberToken(value=0, position=0, end=1)]),
        ("123", [NumberToken(value=123, position=0, end=3)]),
        ("999999", [NumberToken(value=999999, position=0, end=6)]),
        ("3.14", [NumberToken(value=3.14, position=0, end=4)]),
        ("0.5", [NumberToken(value=0.5, position=0, end=3)]),
        ("123.456", [NumberToken(value=123.456, position=0, end=7)]),
        ("01", [NumberToken(value=1, position=0, end=2)]),
        ("00", [NumberToken(value=0, position=0, end=2)]),
        ("1.0", [NumberToken(value=1.0, position=0, end=3)]),
        ("0.0", [NumberToken(value=0.0, position=0, end=3)]),
        ("10.01", [NumberToken(value=10.01, position=0, end=5)]),
        ("+ - * / % ^ = == > < >= <=", [BinaryOperatorToken(value='+', position=0, end=1), BinaryOperatorToken(value='-', position=2, end=3), BinaryOperatorToken(value='*', position=4, end=5), BinaryOperatorToken(value='/', position=6, end=7), BinaryOperatorToken(value='%', position=8, end=9), BinaryOperatorToken(value='^', position=10, end=11), AssignToken(value='=', position=12, end=13), BinaryOperatorToken(value='==', position=14, end=16), BinaryOperatorToken(value='>', position=17, end=18), BinaryOperatorToken(value='<', position=19, end=20), BinaryOperatorToken(value='>=', position=21, end=23), BinaryOperatorToken(value='<=', position=24, end=26)]),
        ("""
        if x > 0 {
            return x
            }
        """, [IfToken(value='if', position=9, end=11), VariableToken(value='x', position=12, end=13), BinaryOperatorToken(value='>', position=14, end=15), NumberToken(value=0, position=16, end=17), OpeningBraceToken(value='{', position=18, end=19), ReturnToken(value='return', position=32, end=38), VariableToken(value='x', position=39, end=40), ClosingBraceToken(value='}', position=53, end=54)])
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