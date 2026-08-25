from collections.abc import Callable

builtins: dict[str, Callable] = {
    "print": print,
}