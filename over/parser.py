from typing import NoReturn
import over.tokens as tokens
import over.nodes as nodes
from over.exceptions import InvalidExpressionError

def parse(tokens_list: list[tokens.Token], expression: str) -> nodes.Node:
    parser = Parser(tokens_list, expression)
    node = parser.parse_program()
    return node

class Parser:
    def __init__(self, tokens_list: list[tokens.Token], expression: str) -> None:
        self.tokens_list = tokens_list
        self.expression = expression
        self.current_index = 0
        self.count_statements = 0

    def current(self) -> tokens.Token | None:
        if self.current_index >= len(self.tokens_list):
            return None
        return self.tokens_list[self.current_index]

    def previous(self) -> tokens.Token | None:
        if self.current_index > 0:
            return self.tokens_list[self.current_index - 1]
        return None

    def advance(self) -> None:
        self.current_index += 1

    def error(self, message: str) -> NoReturn:
        current = self.current()
        previous = self.previous()
        if current is not None:
            pointer = ' ' * current.position + '^'
        elif previous is not None:
            pointer = ' ' * previous.position + '^'
        else:
            pointer = '^'
        raise InvalidExpressionError(
            f"      {message}\n"
            f"      {self.expression}\n"
            f"      {pointer}"
        )

    def match(self, *values: str) -> bool:
        current = self.current()
        return current is not None and current.value in values

    def expect(self, *values: str) -> str:
        current = self.current()
        if "IDENTIFIER" in values:
            if not isinstance(current, tokens.VariableToken):
                self.error(f"ERROR: expected identifier, got '{current}'.")
            return current.value
        if current is not None:
            if current.value in values:
                return current.value
            self.error(f"ERROR: expected {values}, got '{current.value}'.")
        self.error(f"ERROR: expected {values}, got 'None'.")

    def consume(self, *values: str) -> str:
        current = self.expect(*values)
        self.advance()
        return current

    def parse_program(self) -> nodes.Node:
        node_list: list[nodes.Node] = []
        while self.current_index < len(self.tokens_list):
            self.count_statements += 1
            if self.count_statements > 100:
                raise InvalidExpressionError("too many statements.")
            node_list.append(self.parse_statement())
        node = node_list[-1]
        return nodes.BlockNode(node_list, position=node.position, end=node.end)

    def parse_statement(self) -> nodes.Node:
        current = self.current()
        match current:
            case tokens.IfToken(position=position, end=end):
                self.advance()
                return self.parse_if_statement(position, end)
            case tokens.WhileToken(position=position, end=end):
                self.advance()
                return self.parse_while_statement(position, end=end)
            case tokens.FunctionToken(position=position, end=end):
                self.advance()
                return self.parse_function(position, end=end)
            case tokens.ReturnToken(position=position, end=end):
                self.advance()
                return self.parse_return(position, end=end)
            case _:
                return self.parse_assignment()

    def parse_if_statement(self, position, end) -> nodes.Node:
        condition = self.parse_expression()
        body = self.parse_block()
        if not self.match('else', 'иначе'):
            return nodes.IfNode(condition, body, None, position=position, end=end)
        self.advance()
        else_body = self.parse_block()
        return nodes.IfNode(condition, body, else_body, position=position, end=end)

    def parse_while_statement(self, position, end) -> nodes.Node:
        condition = self.parse_expression()
        body = self.parse_block()
        if not self.match('else', 'иначе'):
            return nodes.WhileNode(condition, body, None, position=position, end=end)
        self.advance()
        else_body = self.parse_block()
        return nodes.WhileNode(condition, body, else_body, position=position, end=end)

    def parse_function(self, position, end) -> nodes.Node:
        args: list[nodes.Node] = []
        name = self.consume('IDENTIFIER')
        self.consume('(')
        if not self.match(')'):
            args.append(self.parse_expression())
            while self.match(','):
                self.advance()
                args.append(self.parse_expression())
        self.consume(')')
        body = self.parse_block()
        return nodes.FunctionNode(name, args, body, position=position, end=end)

    def parse_return(self, position, end) -> nodes.Node:
        expression = self.parse_expression()
        return nodes.ReturnNode(expression, position=position, end=end)

    def parse_block(self) -> nodes.BlockNode:
        block: list[nodes.Node] = []
        self.consume('{')
        while True:
            current = self.current()
            if current is None:
                self.error(f"ERROR: expected {'}'}, got 'None'")
            if current.value == '}':
                break
            block.append(self.parse_statement())
        self.consume('}')
        node = block[-1]
        return nodes.BlockNode(block, position=node.position, end=node.end)

    def parse_assignment(self) -> nodes.Node:
        variable = self.parse_expression()
        if self.match('=') and not isinstance(variable, nodes.VariableNode):
            self.error(f"ERROR: expected a variable, got '{variable}'.")
        if self.match('='):
            operator = self.consume('=')
            right = self.parse_expression()
            variable = nodes.AssignNode(variable, operator, right, position=variable.position, end=right.end)
        return variable

    def parse_expression(self) -> nodes.Node:
        left = self.parse_comparison()
        while self.match('+', '-'):
            operator = self.consume('+', '-')
            right = self.parse_comparison()
            left = nodes.BinaryOperatorNode(left, operator, right, position=left.position, end=right.end)
        return left

    def parse_comparison(self) -> nodes.Node:
        left = self.parse_term()
        if self.match('>', '<', '==', '<=', '>='):
            operator = self.consume( '>', '<', '==', '<=', '>=')
            right = self.parse_term()
            left = nodes.BinaryOperatorNode(left, operator, right, position=left.position, end=right.end)
        return left

    def parse_term(self) -> nodes.Node:
        left = self.parse_unary()
        while self.match('*', '/', '%'):
            operator = self.consume('*', '/', '%')
            right = self.parse_unary()
            left = nodes.BinaryOperatorNode(left, operator, right, position=left.position, end=right.end)
        return left

    def parse_unary(self) -> nodes.Node:
        if self.match('-'):
            start_token = self.current()
            self.advance()
            expression = self.parse_unary()
            return nodes.UnaryMinusNode(expression, position=start_token.position, end=expression.end)
        return self.parse_power()

    def parse_power(self) -> nodes.Node:
        left = self.parse_call()
        if self.match('^'):
            operator = self.consume('^')
            right = self.parse_power()
            left = nodes.BinaryOperatorNode(left, operator, right, position=left.position, end=right.end)
        return left

    def parse_call(self) -> nodes.Node:
        func = self.parse_factor()
        while self.match('('):
            self.consume('(')
            args: list[nodes.Node] = []
            if not self.match(')'):
                args.append(self.parse_expression())
                while self.match(','):
                    self.consume(',')
                    args.append(self.parse_expression())
            position = func.position
            if self.expect(')'):
                end = self.current().end
                self.advance()
            func = nodes.CallNode(func, args, position=position, end=end)
        return func

    def parse_factor(self) -> nodes.Node:
        current = self.current()
        if current is None:
            self.error(f"ERROR: unexpected end of expression.")
        match current:
            case tokens.NumberToken(position=position, end=end):
                number = nodes.NumberNode(current.value, position=position, end=end)
                self.advance()
                return number
            case tokens.VariableToken(position=position, end=end):
                variable = nodes.VariableNode(current.value, position=position, end=end)
                self.advance()
                return variable
            case tokens.OpeningParenthesisToken():
                self.consume('(')
                node = self.parse_expression()
                self.consume(')')
                return node
        self.error(f"ERROR: unexpected token '{current.value}' at position {current.position}.")