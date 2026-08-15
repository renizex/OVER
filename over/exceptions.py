class InvalidExpressionError(Exception):
    pass

class InvalidLexemeError(InvalidExpressionError):
    pass

class DivisionByZeroError(InvalidExpressionError):
    pass

class ReturnStatement(Exception):
    def __init__(self, expression):
        self.expression = expression