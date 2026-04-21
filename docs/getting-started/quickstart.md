# Quickstart

Version `0.1.0` introduces the Nextral monorepo structure and runtime-neutral scaffolding.

## Inspect available command surfaces

```bash
python -m nextral --help
python -m nextral memory --help
python -m nextral tools --help
python -m nextral files --help
```

## Check architecture docs

- `docs/architecture/project-structure.md`
- `docs/memory/README.md`

## Current status

- Canonical Rust core boundaries are scaffolded in `src/`
- Python and Node FFI adapters are scaffolded in `bindings/`
- Production runtime features are still planned and not fully implemented

