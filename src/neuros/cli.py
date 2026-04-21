"""CLI for neuros.

This release is docs-first. Commands expose architecture and planned surfaces only.
"""

from __future__ import annotations

import argparse
from typing import Sequence

from ._version import __version__


def _handle_about(_: argparse.Namespace) -> int:
    print(f"neuros {__version__}")
    print("Status: docs-first scaffold (no runtime features in 0.0.1).")
    print("Docs:")
    print("  - docs\\README.md")
    print("  - docs\\memory\\README.md")
    print("  - docs\\architecture\\project-structure.md")
    return 0


def _handle_group_help(args: argparse.Namespace) -> int:
    parser: argparse.ArgumentParser = args._parser
    parser.print_help()
    return 0


def _handle_placeholder(args: argparse.Namespace) -> int:
    if getattr(args, "path", None):
        print(f"`{args.path}` accepted as input.")
    print("This command surface is initialized, but functionality is not implemented in 0.0.1.")
    print("Refer to docs\\memory for the architecture and planned behavior.")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="neuros",
        description=(
            "Neuros CLI - docs-first scaffold for a memory-enabled agent runtime. "
            "Use subcommand help to inspect planned command surfaces."
        ),
    )
    parser.add_argument("--version", action="version", version=f"%(prog)s {__version__}")

    subparsers = parser.add_subparsers(dest="command", metavar="COMMAND")

    about_parser = subparsers.add_parser("about", help="Show package status and documentation entry points.")
    about_parser.set_defaults(handler=_handle_about)

    memory_parser = subparsers.add_parser("memory", help="Memory command group (placeholder surfaces).")
    memory_parser.set_defaults(handler=_handle_group_help, _parser=memory_parser)
    memory_subparsers = memory_parser.add_subparsers(dest="memory_command", metavar="SUBCOMMAND")

    memory_status = memory_subparsers.add_parser(
        "status",
        help="Show memory subsystem status (placeholder).",
    )
    memory_status.set_defaults(handler=_handle_placeholder)

    memory_add_file = memory_subparsers.add_parser(
        "add-file",
        help="Plan to add a file into memory ingestion flow (placeholder).",
    )
    memory_add_file.add_argument("path", nargs="?", help="File path to register for future memory ingestion.")
    memory_add_file.set_defaults(handler=_handle_placeholder)

    tools_parser = subparsers.add_parser("tools", help="Tool registry command group (placeholder surfaces).")
    tools_parser.set_defaults(handler=_handle_group_help, _parser=tools_parser)
    tools_subparsers = tools_parser.add_subparsers(dest="tools_command", metavar="SUBCOMMAND")

    tools_list = tools_subparsers.add_parser("list", help="List configured tools (placeholder).")
    tools_list.set_defaults(handler=_handle_placeholder)

    files_parser = subparsers.add_parser("files", help="File memory command group (placeholder surfaces).")
    files_parser.set_defaults(handler=_handle_group_help, _parser=files_parser)
    files_subparsers = files_parser.add_subparsers(dest="files_command", metavar="SUBCOMMAND")

    files_index = files_subparsers.add_parser(
        "index",
        help="Prepare a file for memory indexing (placeholder).",
    )
    files_index.add_argument("path", nargs="?", help="File path to index later.")
    files_index.set_defaults(handler=_handle_placeholder)

    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    handler = getattr(args, "handler", None)
    if handler is None:
        parser.print_help()
        return 0
    return handler(args)

