"""Python wrappers for the Nextral Rust core."""

from ._version import __version__
from .core import lexical_score

__all__ = ["__version__", "lexical_score"]
