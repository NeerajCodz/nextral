# Changelog

## 0.1.0

- Rebranded project namespace from `neuros` to `nextral`.
- Reorganized repository into a monorepo with a canonical Rust core in `src/`.
- Added runtime-neutral FFI scaffolding:
  - `bindings/python` (PyO3 + maturin layout)
  - `bindings/node` (napi-rs layout)
- Added native app surfaces under `apps/cli` and `apps/mcp`.
- Updated top-level and architecture documentation for the new structure.

## 0.0.1

- Initialized `neuros` Python package metadata and build configuration.
- Added CLI entry point with `--help`, `--version`, and placeholder command groups.
- Added architecture scaffolding for memory, tools, file ingestion, storage, and LangChain integration namespaces.
- Added docs-first project documentation and release notes.
- No runtime features implemented in this release.

