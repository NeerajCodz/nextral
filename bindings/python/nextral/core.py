"""High-level Python wrappers over the native extension."""

from __future__ import annotations

import json
from typing import Any

from ._nextral import lexical_score as _lexical_score
from ._nextral import validate_config as _validate_config


def lexical_score(text: str, query: str) -> float:
    return float(_lexical_score(text, query))


def validate_config(config: dict[str, Any]) -> dict[str, Any]:
    return json.loads(_validate_config(json.dumps(config)))
