# nextral

`nextral` is a runtime-neutral monorepo with a canonical Rust core and FFI bindings for Python and Node.js.

Current public package bootstrap is **info-only** as `nextral` at version `0.0.1` for both PyPI and npm.

## Architecture at a glance

```text
nextral/
├── src/                 # canonical Rust core
├── bindings/
│   ├── python/          # PyO3 bridge + Python wrappers
│   └── node/            # napi-rs bridge + TS wrappers
├── apps/                # native consumers (CLI, MCP, web, examples)
├── tests/
├── docs/
├── scripts/
├── Cargo.toml
├── package.json
└── pyproject.toml
```

## Build surfaces

```bash
cargo build --workspace
pip install -e bindings/python
npm install nextral
```

## Documentation

- Main index: `docs/README.md`
- Project structure: `docs/architecture/project-structure.md`
- Memory system docs: `docs/memory/README.md`
- Package production runtime: `docs/package-production.md`

