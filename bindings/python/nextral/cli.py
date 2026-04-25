"""CLI for nextral."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Sequence

from ._version import __version__
from .core import validate_config


def _load_json(path: str) -> dict:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def _handle_about(_: argparse.Namespace) -> int:
    print(f"nextral {__version__}")
    print("Status: package-first production runtime with configured storage and model providers.")
    print("Docs:")
    print("  - docs\\README.md")
    print("  - docs\\architecture\\project-structure.md")
    print("  - docs\\memory\\README.md")
    return 0


def _handle_config_validate(args: argparse.Namespace) -> int:
    result = validate_config(_load_json(args.config))
    print(json.dumps(result, indent=2))
    return 0


def _handle_not_implemented(args: argparse.Namespace) -> int:
    raise SystemExit(
        f"`nextral {args.command}` is reserved for the production runtime surface. "
        "Implement the corresponding Rust service/client path before enabling it."
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="nextral",
        description="Nextral CLI - package-first memory runtime backed by configured production stores.",
    )
    parser.add_argument("--version", action="version", version=f"%(prog)s {__version__}")

    subparsers = parser.add_subparsers(dest="command", metavar="COMMAND")
    about_parser = subparsers.add_parser("about", help="Show package status and documentation entry points.")
    about_parser.set_defaults(handler=_handle_about)

    config_parser = subparsers.add_parser("config", help="Configuration commands.")
    config_subparsers = config_parser.add_subparsers(dest="config_command", metavar="SUBCOMMAND")
    validate_parser = config_subparsers.add_parser("validate", help="Validate a Nextral runtime config file.")
    validate_parser.add_argument("config", help="Path to JSON config.")
    validate_parser.set_defaults(handler=_handle_config_validate)

    for command, help_text in [
        ("db", "Database migration and provisioning commands."),
        ("memory", "Memory ingestion, search, and governance commands."),
        ("session", "Session append and lifecycle commands."),
        ("graph", "Graph query commands."),
        ("reminders", "Prospective memory commands."),
        ("serve", "Run optional HTTP, gRPC, or GraphQL service modes."),
    ]:
        reserved = subparsers.add_parser(command, help=help_text)
        reserved.set_defaults(handler=_handle_not_implemented)

    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    handler = getattr(args, "handler", None)
    if handler is None:
        parser.print_help()
        return 0
    return handler(args)
