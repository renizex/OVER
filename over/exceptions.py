class InvalidExpressionError(Exception):
    pass

class InvalidLexemeError(InvalidExpressionError):
    pass

class DivisionByZeroError(InvalidExpressionError):
    pass