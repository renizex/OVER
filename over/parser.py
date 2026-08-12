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
        self.count_power = 0

    def current(self) -> tokens.Token | None:
        #print(self.current_index, len(self.tokens))
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
            pointer = ' ' * current.position + '^' if self.current_index else '^'
        elif previous is not None:
            pointer = ' ' * previous.position + '^' if self.current_index else '^'
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

    def consume(self, *values: str) -> str | None:
        current = self.current()
        if current is None:
            return None
        if current.value not in values:
            return None
        self.advance()
        return current.value

    def expect(self, *args) -> bool:
        current = self.current()
        if current is not None:
            if current.value in args:
                return True
            self.error(f"ERROR: expected {args}, got {current.value}.")
        return False

    def parse_program(self) -> nodes.Node:
        node_list: list[nodes.Node] = []
        while self.current_index < len(self.tokens_list):
            self.count_statements += 1
            if self.count_statements > 100:
                raise InvalidExpressionError("too many statements.")
            node_list.append(self.parse_statement())
        return nodes.BlockNode(node_list)

    def parse_statement(self) -> nodes.Node:
        current = self.current()
        match current:
            case tokens.IfToken():
                self.advance()
                return self.parse_if_statement()
            case tokens.WhileToken():
                self.advance()
                return self.parse_while_statement()
            case _:
                return self.parse_assignment()

    def parse_if_statement(self) -> nodes.Node:
        condition = self.parse_expression()
        body = self.parse_block()
        if self.consume('else') is None:
            return nodes.IfNode(condition, body, None)
        else_body = self.parse_block()
        return nodes.IfNode(condition, body, else_body)

    def parse_while_statement(self) -> nodes.Node:
        condition = self.parse_expression()
        body = self.parse_block()
        if self.consume('else') is None:
            return nodes.WhileNode(condition, body, None)
        else_body = self.parse_block()
        return nodes.WhileNode(condition, body, else_body)

    def parse_block(self) -> nodes.BlockNode:
        block: list[nodes.Node] = []
        if self.consume('{') is None:
            self.error(f"ERROR: expected '{'{'}', got 'None'.")
        while True:
            current = self.current()
            if current is None:
                self.error(f"ERROR: expected {'}'}, got 'None'")
            if current.value == '}':
                break
            block.append(self.parse_statement())
        self.consume('}')
        return nodes.BlockNode(block)

    def parse_assignment(self) -> nodes.Node:
        variable = self.parse_expression()
        while True:
            operator = self.consume('=')
            if operator is None:
                break
            if not isinstance(variable, nodes.VariableNode):
                self.error(f"ERROR: expected a variable, got '{variable}'.")
            right = self.parse_expression()
            variable = nodes.AssignNode(variable, operator, right)
        return variable

    def parse_expression(self) -> nodes.Node:
        left = self.parse_comparison()
        while True:
            operator = self.consume('+', '-')
            if operator is None:
                break
            right = self.parse_comparison()
            left = nodes.BinaryOperatorNode(left, operator, right)
        return left

    def parse_comparison(self) -> nodes.Node:
        left = self.parse_term()
        while True:
            operator = self.consume( '>', '<', '==')
            if operator is None:
                break
            right = self.parse_term()
            left = nodes.BinaryOperatorNode(left, operator, right)
        return left

    def parse_term(self) -> nodes.Node:
        left = self.parse_unary()
        while True:
            operator = self.consume('*', '/')
            if operator is None:
                break
            right = self.parse_unary()
            left = nodes.BinaryOperatorNode(left, operator, right)
        return left

    def parse_unary(self) -> nodes.Node:
        if self.match('-'):
            self.advance()
            expression = self.parse_unary()
            return nodes.UnaryMinusNode(expression)
        return self.parse_power()

    def parse_power(self) -> nodes.Node:
        max_power = 1000
        left = self.parse_factor()
        while True:
            operator = self.consume('^')
            if operator is None:
                break
            self.count_power += 1
            if self.count_power > max_power:
                raise InvalidExpressionError(f"ERROR: power expression is too deep.")
            right = self.parse_power()
            left = nodes.BinaryOperatorNode(left, operator, right)
        return left

    def parse_factor(self) -> nodes.Node:
        current = self.current()
        if current is None:
            self.error(f"ERROR: unexpected end of expression.")
        match current:
            case tokens.NumberToken():
                number = nodes.NumberNode(current.value)
                self.advance()
                return number
            case tokens.VariableToken():
                variable = nodes.VariableNode(current.value)
                self.advance()
                return variable
            case tokens.OpeningParenthesisToken():
                self.advance()
                node = self.parse_expression()
                if self.expect(')'):
                    self.advance()
                    return node
                self.error(f"ERROR: expected closing parenthesis at position {self.current_index + 1}.")
        self.error(f"ERROR: unexpected token '{current.value}' at position {self.current_index + 1}.")
