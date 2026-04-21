"""High-level Python wrappers over the native extension."""

from ._nextral import lexical_score as _lexical_score


def lexical_score(text: str, query: str) -> float:
    return float(_lexical_score(text, query))
