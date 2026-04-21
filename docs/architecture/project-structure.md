# Project structure (Nextral runtime-neutral layout)

```text
nextral/
├── src/
│   ├── memory/
│   ├── retrieval/
│   ├── graph/
│   ├── scoring/
│   ├── runtime/
│   ├── contracts/
│   └── lib.rs
├── bindings/
│   ├── python/
│   │   ├── nextral/
│   │   ├── src/
│   │   ├── pyproject.toml
│   │   └── Cargo.toml
│   └── node/
│       ├── src/
│       ├── package.json
│       ├── Cargo.toml
│       └── index.ts
├── apps/
│   ├── cli/
│   ├── mcp/
│   ├── web/
│   └── examples/
├── tests/
├── docs/
├── scripts/
├── Cargo.toml
├── package.json
├── pyproject.toml
├── README.md
└── CHANGELOG.md
```

## Runtime-neutral boundary

- The Rust core in `src/` is canonical and language-agnostic.
- Core APIs use Rust-native types (`Vec<T>`, structs, enums) and `thiserror` for domain errors.
- FFI crates in `bindings/python` and `bindings/node` map those errors into runtime-native exceptions.

## Async strategy

- Internal concurrency and orchestration are handled in the Rust runtime module (Tokio-based).
- Python bindings bridge async work into `asyncio` with `pyo3-async-runtimes`.
- Node bindings expose async Rust work as Promise-based APIs through napi-rs.

## Serialization strategy

- Shared graph/memory payloads are represented as Serde-compatible Rust types in the core.
- Bindings convert those payloads into runtime-native objects without duplicating business logic.

