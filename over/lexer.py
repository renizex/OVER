import re
from over.exceptions import InvalidLexemeError
import over.tokens as tokens

def lex(expression: str) -> list[tokens.Token]:
    max_tokens = 10000
    tokens_list: list[tokens.Token] = []
    matches = re.finditer(r"(\d+\.\d+|\d+)|([A-Za-zА-Яа-я_]\w*)|(==|<=|>=|[,+\-*/=()^><{}%])|(\s+)|(.)", expression)
    for match in matches:
        if match.group(1):
            raw_number = match.group(1)
            number = float(raw_number) if '.' in raw_number else int(raw_number)
            tokens_list.append(tokens.NumberToken(number, match.start()))
        elif match.group(2):
            variable = match.group(2)
            if variable in keywords:
                tokens_list.append(keywords[variable](variable, match.start()))
            else:
                tokens_list.append(tokens.VariableToken(variable, match.start()))
        elif match.group(3):
            operator = match.group(3)
            if operator in special_operators:
                tokens_list.append(special_operators[operator](operator, match.start()))
            else:
                tokens_list.append(tokens.BinaryOperatorToken(operator, match.start()))
        elif match.group(4):
            pass
        elif match.group(5):
            raise InvalidLexemeError(f"ERROR: unknown lexeme '{match.group(5)}' at position {match.start()}.")
    if len(tokens_list) > max_tokens:
        raise InvalidLexemeError(f"ERROR: too many tokens in expression.")
    return tokens_list

keywords = {
    'if': tokens.IfToken,
    'else': tokens.ElseToken,
    'while': tokens.WhileToken,
    'function': tokens.FunctionToken,
    'return': tokens.ReturnToken,
    'если': tokens.IfToken,
    'иначе': tokens.ElseToken,
    'пока': tokens.WhileToken,
    'функция': tokens.FunctionToken,
    'вернуть': tokens.ReturnToken,
}

special_operators = {
    '=': tokens.AssignToken,
    '(': tokens.OpeningParenthesisToken,
    ')': tokens.ClosingParenthesisToken,
    '{': tokens.OpeningBraceToken,
    '}': tokens.ClosingBraceToken,
    ',': tokens.ContinueArgsToken,
}