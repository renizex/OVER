import pytest
from over.exceptions import InvalidExpressionError
from over.interpreter import interpret
from over.nodes import *

@pytest.mark.parametrize(
    "source, node, expected", [
    ("2-2",
     BlockNode(block=[BinaryOperatorNode(left=NumberNode(value=2), operator='-', right=NumberNode(value=2))]),
     0),
    ("2 + 3 * 4 / 5 ^ 2",
     BlockNode(block=[BinaryOperatorNode(left=NumberNode(value=2), operator='+', right=BinaryOperatorNode(left=BinaryOperatorNode(left=NumberNode(value=3), operator='*', right=NumberNode(value=4)), operator='/', right=BinaryOperatorNode(left=NumberNode(value=5), operator='^', right=NumberNode(value=2))))]),
     2.48),
    ("x = 5 % 51",
     BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=BinaryOperatorNode(left=NumberNode(value=5), operator='%', right=NumberNode(value=51)))]),
     None),
    ("x = 5 x >= 10",
     BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=NumberNode(value=5)), BinaryOperatorNode(left=VariableNode(value='x'), operator='>=', right=NumberNode(value=10))]),
     False),
    ("5 >= 10 40 <= 50 100 == 100500 90 > 200 90 < 200",
     BlockNode(block=[BinaryOperatorNode(left=NumberNode(value=5), operator='>=', right=NumberNode(value=10)), BinaryOperatorNode(left=NumberNode(value=40), operator='<=', right=NumberNode(value=50)), BinaryOperatorNode(left=NumberNode(value=100), operator='==', right=NumberNode(value=100500)), BinaryOperatorNode(left=NumberNode(value=90), operator='>', right=NumberNode(value=200)), BinaryOperatorNode(left=NumberNode(value=90), operator='<', right=NumberNode(value=200))]),
     True),
    ("-123",
     BlockNode(block=[UnaryMinusNode(operand=NumberNode(value=123))]),
     -123),
    ("x = 5 y = 10 if x > y {x + y} else {x - y}",
     BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=NumberNode(value=5)), AssignNode(variable=VariableNode(value='y'), operator='=', right=NumberNode(value=10)), IfNode(condition=BinaryOperatorNode(left=VariableNode(value='x'), operator='>', right=VariableNode(value='y')), body=BlockNode(block=[BinaryOperatorNode(left=VariableNode(value='x'), operator='+', right=VariableNode(value='y'))]), else_body=BlockNode(block=[BinaryOperatorNode(left=VariableNode(value='x'), operator='-', right=VariableNode(value='y'))]))]),
     -5),
    ("function func(x) {while x < 100 {x = x + 1} return x} func(1)",
     BlockNode(block=[FunctionNode(name='func', args=[VariableNode(value='x')], body=BlockNode(block=[WhileNode(condition=BinaryOperatorNode(left=VariableNode(value='x'), operator='<', right=NumberNode(value=100)), body=BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=BinaryOperatorNode(left=VariableNode(value='x'), operator='+', right=NumberNode(value=1)))]), else_body=None), ReturnNode(expression=VariableNode(value='x'))])), CallNode(name=VariableNode(value='func'), args=[NumberNode(value=1)])]),
     100),
    ("function func(x) {while x < 100 {x = x + 1} else {x = x + 1} return x} func(1)",
     BlockNode(block=[FunctionNode(name='func', args=[VariableNode(value='x')], body=BlockNode(block=[WhileNode(condition=BinaryOperatorNode(left=VariableNode(value='x'), operator='<', right=NumberNode(value=100)), body=BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=BinaryOperatorNode(left=VariableNode(value='x'), operator='+', right=NumberNode(value=1)))]), else_body=BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=BinaryOperatorNode(left=VariableNode(value='x'), operator='+', right=NumberNode(value=1)))])), ReturnNode(expression=VariableNode(value='x'))])), CallNode(name=VariableNode(value='func'), args=[NumberNode(value=1)])]),
     101),
    ("функция функ() {я = 0 пока я < 10 {я = я + 1} вернуть я} функ()",
     BlockNode(block=[FunctionNode(name='функ', args=[], body=BlockNode(block=[AssignNode(variable=VariableNode(value='я'), operator='=', right=NumberNode(value=0)), WhileNode(condition=BinaryOperatorNode(left=VariableNode(value='я'), operator='<', right=NumberNode(value=10)), body=BlockNode(block=[AssignNode(variable=VariableNode(value='я'), operator='=', right=BinaryOperatorNode(left=VariableNode(value='я'), operator='+', right=NumberNode(value=1)))]), else_body=None), ReturnNode(expression=VariableNode(value='я'))])), CallNode(name=VariableNode(value='функ'), args=[])]),
     10),
    ("function factorial(n) {if n == 0 {return 1} return n * factorial(n - 1)} factorial(5)",
     BlockNode(block=[FunctionNode(name='factorial', args=[VariableNode(value='n')], body=BlockNode(block=[IfNode(condition=BinaryOperatorNode(left=VariableNode(value='n'), operator='==', right=NumberNode(value=0)), body=BlockNode(block=[ReturnNode(expression=NumberNode(value=1))]), else_body=None), ReturnNode(expression=BinaryOperatorNode(left=VariableNode(value='n'), operator='*', right=CallNode(name=VariableNode(value='factorial'), args=[BinaryOperatorNode(left=VariableNode(value='n'), operator='-', right=NumberNode(value=1))])))])), CallNode(name=VariableNode(value='factorial'), args=[NumberNode(value=5)])]),
     120),
    ("print(2 ^ 10)",
     BlockNode(block=[CallNode(name=VariableNode(value='print'), args=[BinaryOperatorNode(left=NumberNode(value=2), operator='^', right=NumberNode(value=10))])]),
     None),
    ("x = 500 function func(x) {x = 100500 return x} func(10) x",
     BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=NumberNode(value=500)), FunctionNode(name='func', args=[VariableNode(value='x')], body=BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=NumberNode(value=100500)), ReturnNode(expression=VariableNode(value='x'))])), CallNode(name=VariableNode(value='func'), args=[NumberNode(value=10)]), VariableNode(value='x')]),
     500),
    ("x = 500 function func() {return x} func()",
     BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=NumberNode(value=500)), FunctionNode(name='func', args=[], body=BlockNode(block=[ReturnNode(expression=VariableNode(value='x'))])), CallNode(name=VariableNode(value='func'), args=[])]),
     500),
    ("function func() {}",
     BlockNode(block=[FunctionNode(name='func', args=[], body=BlockNode(block=[]))]),
     None),
    ]
)
def test_interpreter(source, node, expected):
    assert interpret(node) == expected

@pytest.mark.parametrize(
    "node, expected",
    [
        (BlockNode(block=[VariableNode(value='x')]),
         "ERROR: variable 'x' does not exist."),
        (BlockNode(block=[UnaryMinusNode(operand=CallNode(name=VariableNode(value='print'), args=[NumberNode(value=5)]))]),
         "ERROR: expected number, got type 'None'."),
        (BlockNode(block=[BinaryOperatorNode(left=NumberNode(value=2), operator='+',right=CallNode(name=VariableNode(value='print'),args=[NumberNode(value=5)]))]),
         "ERROR: operator '+' requires two valid numbers, got int and NoneType."),
        (BlockNode(block=[FunctionNode(name='func', args=[VariableNode(value='x'), VariableNode(value='y')],body=BlockNode(block=[CallNode(name=VariableNode(value='print'), args=[BinaryOperatorNode(left=VariableNode(value='x'), operator='+',right=VariableNode(value='y'))])])),CallNode(name=VariableNode(value='func'),args=[NumberNode(value=100500), NumberNode(value=200), NumberNode(value=300)])]),
         "ERROR: expected 2 arguments, got 3."),
        (BlockNode(block=[CallNode(name=NumberNode(value=5), args=[VariableNode(value='x'), VariableNode(value='y')])]),
         "ERROR: invalid function call '5'."),
        ("ShitNode([])", "ERROR: unsupported AST node.\nnode: ShitNode([])"),
        (BlockNode(block=[AssignNode(variable=VariableNode(value='x'), operator='=', right=CallNode(name=VariableNode(value='print'), args=[NumberNode(value=5)]))]),
         "ERROR: unexpected None type."),
        (BlockNode(block=[CallNode(name=VariableNode(value='func'), args=[])]),
         "ERROR: function 'func' does not exist."),
    ]
)
def test_invalid(node, expected):
    with pytest.raises(InvalidExpressionError) as msg:
        interpret(node)
    assert str(msg.value) == expected