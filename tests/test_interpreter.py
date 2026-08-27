import pytest
from over.exceptions import InvalidExpressionError
from over.interpreter import interpret
from over.nodes import *

@pytest.mark.parametrize(
    "source, node, expected", [
    ("2-2",
     BlockNode(position=0, end=3, block=[BinaryOperatorNode(position=0, end=3, left=NumberNode(position=0, end=1, value=2), operator='-', right=NumberNode(position=2, end=3, value=2))]),
     0),
    ("2 + 3 * 4 / 5 ^ 2",
     BlockNode(position=0, end=17, block=[BinaryOperatorNode(position=0, end=17, left=NumberNode(position=0, end=1, value=2), operator='+', right=BinaryOperatorNode(position=4, end=17, left=BinaryOperatorNode(position=4, end=9, left=NumberNode(position=4, end=5, value=3), operator='*', right=NumberNode(position=8, end=9, value=4)), operator='/', right=BinaryOperatorNode(position=12, end=17, left=NumberNode(position=12, end=13, value=5), operator='^', right=NumberNode(position=16, end=17, value=2))))]),
     2.48),
    ("x = 5 % 51",
     BlockNode(position=0, end=10, block=[AssignNode(position=0, end=10, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=BinaryOperatorNode(position=4, end=10, left=NumberNode(position=4, end=5, value=5), operator='%', right=NumberNode(position=8, end=10, value=51)))]),
     None),
    ("x = 5 x >= 10",
     BlockNode(position=6, end=13, block=[AssignNode(position=0, end=5, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=NumberNode(position=4, end=5, value=5)), BinaryOperatorNode(position=6, end=13, left=VariableNode(position=6, end=7, value='x'), operator='>=', right=NumberNode(position=11, end=13, value=10))]),
     False),
    ("5 >= 10 40 <= 50 100 == 100500 90 > 200 90 < 200",
     BlockNode(position=40, end=48, block=[BinaryOperatorNode(position=0, end=7, left=NumberNode(position=0, end=1, value=5), operator='>=', right=NumberNode(position=5, end=7, value=10)), BinaryOperatorNode(position=8, end=16, left=NumberNode(position=8, end=10, value=40), operator='<=', right=NumberNode(position=14, end=16, value=50)), BinaryOperatorNode(position=17, end=30, left=NumberNode(position=17, end=20, value=100), operator='==', right=NumberNode(position=24, end=30, value=100500)), BinaryOperatorNode(position=31, end=39, left=NumberNode(position=31, end=33, value=90), operator='>', right=NumberNode(position=36, end=39, value=200)), BinaryOperatorNode(position=40, end=48, left=NumberNode(position=40, end=42, value=90), operator='<', right=NumberNode(position=45, end=48, value=200))]),
     True),
    ("-123",
     BlockNode(position=0, end=4, block=[UnaryMinusNode(position=0, end=4, operand=NumberNode(position=1, end=4, value=123))]),
     -123),
    ("x = 5 y = 10 if x > y {x + y} else {x - y}",
     BlockNode(position=13, end=15, block=[AssignNode(position=0, end=5, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=NumberNode(position=4, end=5, value=5)), AssignNode(position=6, end=12, variable=VariableNode(position=6, end=7, value='y'), operator='=', right=NumberNode(position=10, end=12, value=10)), IfNode(position=13, end=15, condition=BinaryOperatorNode(position=16, end=21, left=VariableNode(position=16, end=17, value='x'), operator='>', right=VariableNode(position=20, end=21, value='y')), body=BlockNode(position=23, end=28, block=[BinaryOperatorNode(position=23, end=28, left=VariableNode(position=23, end=24, value='x'), operator='+', right=VariableNode(position=27, end=28, value='y'))]), else_body=BlockNode(position=36, end=41, block=[BinaryOperatorNode(position=36, end=41, left=VariableNode(position=36, end=37, value='x'), operator='-', right=VariableNode(position=40, end=41, value='y'))]))]),
     -5),
    ("function func(x) {while x < 100 {x = x + 1} return x} func(1)",
     BlockNode(position=54, end=61, block=[FunctionNode(position=0, end=8, name='func', args=[VariableNode(position=14, end=15, value='x')], body=BlockNode(position=44, end=50, block=[WhileNode(position=18, end=23, condition=BinaryOperatorNode(position=24, end=31, left=VariableNode(position=24, end=25, value='x'), operator='<', right=NumberNode(position=28, end=31, value=100)), body=BlockNode(position=33, end=42, block=[AssignNode(position=33, end=42, variable=VariableNode(position=33, end=34, value='x'), operator='=', right=BinaryOperatorNode(position=37, end=42, left=VariableNode(position=37, end=38, value='x'), operator='+', right=NumberNode(position=41, end=42, value=1)))]), else_body=None), ReturnNode(position=44, end=50, expression=VariableNode(position=51, end=52, value='x'))])), CallNode(position=54, end=61, name=VariableNode(position=54, end=58, value='func'), args=[NumberNode(position=59, end=60, value=1)])]),
     100),
    ("function func(x) {while x < 100 {x = x + 1} else {x = x + 1} return x} func(1)",
     BlockNode(position=71, end=78, block=[FunctionNode(position=0, end=8, name='func', args=[VariableNode(position=14, end=15, value='x')], body=BlockNode(position=61, end=67, block=[WhileNode(position=18, end=23, condition=BinaryOperatorNode(position=24, end=31, left=VariableNode(position=24, end=25, value='x'), operator='<', right=NumberNode(position=28, end=31, value=100)), body=BlockNode(position=33, end=42, block=[AssignNode(position=33, end=42, variable=VariableNode(position=33, end=34, value='x'), operator='=', right=BinaryOperatorNode(position=37, end=42, left=VariableNode(position=37, end=38, value='x'), operator='+', right=NumberNode(position=41, end=42, value=1)))]), else_body=BlockNode(position=50, end=59, block=[AssignNode(position=50, end=59, variable=VariableNode(position=50, end=51, value='x'), operator='=', right=BinaryOperatorNode(position=54, end=59, left=VariableNode(position=54, end=55, value='x'), operator='+', right=NumberNode(position=58, end=59, value=1)))])), ReturnNode(position=61, end=67, expression=VariableNode(position=68, end=69, value='x'))])), CallNode(position=71, end=78, name=VariableNode(position=71, end=75, value='func'), args=[NumberNode(position=76, end=77, value=1)])]),
     101),
    ("функция функ() {я = 0 пока я < 10 {я = я + 1} вернуть я} функ()",
     BlockNode(position=57, end=63, block=[FunctionNode(position=0, end=7, name='функ', args=[], body=BlockNode(position=46, end=53, block=[AssignNode(position=16, end=21, variable=VariableNode(position=16, end=17, value='я'), operator='=', right=NumberNode(position=20, end=21, value=0)), WhileNode(position=22, end=26, condition=BinaryOperatorNode(position=27, end=33, left=VariableNode(position=27, end=28, value='я'), operator='<', right=NumberNode(position=31, end=33, value=10)), body=BlockNode(position=35, end=44, block=[AssignNode(position=35, end=44, variable=VariableNode(position=35, end=36, value='я'), operator='=', right=BinaryOperatorNode(position=39, end=44, left=VariableNode(position=39, end=40, value='я'), operator='+', right=NumberNode(position=43, end=44, value=1)))]), else_body=None), ReturnNode(position=46, end=53, expression=VariableNode(position=54, end=55, value='я'))])), CallNode(position=57, end=63, name=VariableNode(position=57, end=61, value='функ'), args=[])]),
     10),
    ("function factorial(n) {if n == 0 {return 1} return n * factorial(n - 1)} factorial(5)",
     BlockNode(position=73, end=85, block=[FunctionNode(position=0, end=8, name='factorial', args=[VariableNode(position=19, end=20, value='n')], body=BlockNode(position=44, end=50, block=[IfNode(position=23, end=25, condition=BinaryOperatorNode(position=26, end=32, left=VariableNode(position=26, end=27, value='n'), operator='==', right=NumberNode(position=31, end=32, value=0)), body=BlockNode(position=34, end=40, block=[ReturnNode(position=34, end=40, expression=NumberNode(position=41, end=42, value=1))]), else_body=None), ReturnNode(position=44, end=50, expression=BinaryOperatorNode(position=51, end=71, left=VariableNode(position=51, end=52, value='n'), operator='*', right=CallNode(position=55, end=71, name=VariableNode(position=55, end=64, value='factorial'), args=[BinaryOperatorNode(position=65, end=70, left=VariableNode(position=65, end=66, value='n'), operator='-', right=NumberNode(position=69, end=70, value=1))])))])), CallNode(position=73, end=85, name=VariableNode(position=73, end=82, value='factorial'), args=[NumberNode(position=83, end=84, value=5)])]),
     120),
    ("print(2 ^ 10)",
     BlockNode(position=0, end=13, block=[CallNode(position=0, end=13, name=VariableNode(position=0, end=5, value='print'), args=[BinaryOperatorNode(position=6, end=12, left=NumberNode(position=6, end=7, value=2), operator='^', right=NumberNode(position=10, end=12, value=10))])]),
     None),
    ("x = 500 function func(x) {x = 100500 return x} func(10) x",
     BlockNode(position=56, end=57, block=[AssignNode(position=0, end=7, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=NumberNode(position=4, end=7, value=500)), FunctionNode(position=8, end=16, name='func', args=[VariableNode(position=22, end=23, value='x')], body=BlockNode(position=37, end=43, block=[AssignNode(position=26, end=36, variable=VariableNode(position=26, end=27, value='x'), operator='=', right=NumberNode(position=30, end=36, value=100500)), ReturnNode(position=37, end=43, expression=VariableNode(position=44, end=45, value='x'))])), CallNode(position=47, end=55, name=VariableNode(position=47, end=51, value='func'), args=[NumberNode(position=52, end=54, value=10)]), VariableNode(position=56, end=57, value='x')]),
     500),
    ("x = 500 function func() {return x} func()",
     BlockNode(position=35, end=41, block=[AssignNode(position=0, end=7, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=NumberNode(position=4, end=7, value=500)), FunctionNode(position=8, end=16, name='func', args=[], body=BlockNode(position=25, end=31, block=[ReturnNode(position=25, end=31, expression=VariableNode(position=32, end=33, value='x'))])), CallNode(position=35, end=41, name=VariableNode(position=35, end=39, value='func'), args=[])]),
     500),
    ("function func() {}",
     BlockNode(position=0, end=8, block=[FunctionNode(position=0, end=8, name='func', args=[], body=BlockNode(position=16, end=18, block=[]))]),
     None),
    ]
)
def test_interpreter(source: str, node: BlockNode, expected: int | float | bool | None):
    assert interpret(node, source) == expected

@pytest.mark.parametrize(
    "source, node, expected",
    [
        ("x",
         BlockNode(position=0, end=1, block=[VariableNode(position=0, end=1, value='x')]),
         "      ERROR: variable 'x' does not exist.\n      x\n      ^"),
        ("-print(5)",
         BlockNode(position=0, end=9, block=[UnaryMinusNode(position=0, end=9, operand=CallNode(position=1, end=9, name=VariableNode(position=1, end=6, value='print'), args=[NumberNode(position=7, end=8, value=5)]))]),
         "      ERROR: expected number, got type 'None'.\n      -print(5)\n      ^^^^^^^^^"),
        ("2 + print(5)",
         BlockNode(position=0, end=12, block=[BinaryOperatorNode(position=0, end=12, left=NumberNode(position=0, end=1, value=2), operator='+', right=CallNode(position=4, end=12, name=VariableNode(position=4, end=9, value='print'), args=[NumberNode(position=10, end=11, value=5)]))]),
         "      ERROR: operator '+' requires two valid numbers, got int and NoneType.\n      2 + print(5)\n      ^^^^^^^^^^^^"),
        ("function func(x, y) {print(x + y)} func(10000, 100500, 414132442)",
         BlockNode(position=35, end=65, block=[FunctionNode(position=0, end=8, name='func', args=[VariableNode(position=14, end=15, value='x'), VariableNode(position=17, end=18, value='y')], body=BlockNode(position=20, end=34, block=[CallNode(position=21, end=33, name=VariableNode(position=21, end=26, value='print'), args=[BinaryOperatorNode(position=27, end=32, left=VariableNode(position=27, end=28, value='x'), operator='+', right=VariableNode(position=31, end=32, value='y'))])])), CallNode(position=35, end=65, name=VariableNode(position=35, end=39, value='func'), args=[NumberNode(position=40, end=45, value=10000), NumberNode(position=47, end=53, value=100500), NumberNode(position=55, end=64, value=414132442)])]),
         ""
         "      ERROR: expected 2 arguments, got 3.\n"
         "      function func(x, y) {print(x + y)} func(10000, 100500, 414132442)\n"
         "                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"),
        ("5(x, y)",
         BlockNode(position=0, end=7, block=[CallNode(position=0, end=7, name=NumberNode(position=0, end=1, value=5), args=[VariableNode(position=2, end=3, value='x'), VariableNode(position=5, end=6, value='y')])]),
         ""
         "      ERROR: 'int' is not callable.\n"
         "      5(x, y)\n"
         "      ^^^^^^^"),
        ("x = print(5)",
         BlockNode(position=0, end=12, block=[AssignNode(position=0, end=12, variable=VariableNode(position=0, end=1, value='x'), operator='=', right=CallNode(position=4, end=12, name=VariableNode(position=4, end=9, value='print'), args=[NumberNode(position=10, end=11, value=5)]))]),
         "      ERROR: variable 'x' can't be assigned to a None value.\n      x = print(5)\n          ^^^^^^^^"),
        ("func()",
         BlockNode(position=0, end=6, block=[CallNode(position=0, end=6, name=VariableNode(position=0, end=4, value='func'), args=[])]),
         ""
         "      ERROR: function 'func' does not exist.\n"
         "      func()\n"
         "      ^^^^^^"),
    ]
)
def test_invalid(source: str, node: BlockNode | str, expected: str):
    with pytest.raises(InvalidExpressionError) as msg:
        interpret(node, source)
    print(str(msg.value))
    assert str(msg.value) == expected