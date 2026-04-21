# Project structure (initialized for 0.0.1)

```text
neuros/
├── pyproject.toml
├── README.md
├── CHANGELOG.md
├── docs/
│   ├── README.md
│   ├── architecture/
│   │   └── project-structure.md
│   ├── cli/
│   │   ├── README.md
│   │   └── help.md
│   ├── getting-started/
│   │   ├── installation.md
│   │   └── quickstart.md
│   ├── releases/
│   │   └── 0.0.1.md
│   └── memory/
│       ├── README.md
│       ├── architecture.md
│       ├── types/
│       ├── pipeline/
│       ├── contracts/
│       ├── workflow/
│       └── operations/
└── src/
    └── neuros/
        ├── __init__.py
        ├── __main__.py
        ├── _version.py
        ├── cli.py
        ├── memory/
        │   ├── __init__.py
        │   └── types/
        │       └── __init__.py
        ├── tools/
        │   └── __init__.py
        ├── files/
        │   └── __init__.py
        ├── storage/
        │   └── __init__.py
        └── integrations/
            ├── __init__.py
            └── langchain/
                └── __init__.py
```

## Intent of this scaffold

- Memory-first architecture aligned with docs in `docs/memory/`
- Future tool runtime namespace in `src/neuros/tools`
- Future file-to-memory ingestion namespace in `src/neuros/files`
- Future LangChain adapters in `src/neuros/integrations/langchain`

No runtime logic is shipped in `0.0.1`.

