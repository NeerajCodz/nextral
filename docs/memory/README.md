# Memory — Builder Docs

## Index

1. [Purpose](#purpose)
2. [Canonical Diagrams](#canonical-diagrams)
3. [Subfolders](#subfolders)
4. [Recommended Reading Order](#recommended-reading-order)
5. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Purpose

This folder contains implementation-facing documentation for Tony's full memory system:
seven memory types, hot-to-cold lifecycle tiers, and hybrid vector + graph retrieval.

## Canonical Diagrams

| Diagram | Scope |
|---|---|
| `memory_taxonomy.svg` | Full seven-type taxonomy and storage backends |
| `memory_lifecycle.svg` | End-to-end flow from live message to cold storage and retrieval |
| `graphify_dual_retrieval.svg` | Graphify extraction and dual-path (vector + graph) retrieval |

## Subfolders

| Path | Purpose |
|---|---|
| `types/` | Seven memory type docs (`working`, `session`, `episodic`, `semantic`, `relational`, `procedural`, `prospective`) |
| `architecture/` | Runtime, storage lifecycle, retrieval, and graph internals |
| `contracts/` | Data/interface/state contracts for memory pipelines |
| `operations/` | Runbooks, SLOs, incidents, and migration operations |
| `workflow/` | End-to-end behavioral flows (write, forget, proactive follow-up) |
| `pipeline/` | Stage-by-stage pipeline specs (ingestion, consolidation, graphify, retrieval) |

## Recommended Reading Order

1. `architecture.md`
2. `types/README.md`
3. `architecture/README.md`
4. `contracts/README.md`
5. `operations/README.md`
6. `workflow/README.md`
7. `pipeline/README.md`


<!-- memory-expansion-2026-04-10 -->

## Builder Addendum: Expanded Control Surface

This addendum extends the document with practical implementation controls for the Tony memory runtime.

| Control surface | Default posture | Why it matters |
|---|---|---|
| Candidate precision | threshold-gated writes | reduces low-signal memory pollution |
| Recall diversity | vector + graph blending | improves answer richness and grounding |
| Durability | multi-store receipts + audit trail | prevents silent memory loss |
| Cost efficiency | token-budget fitting and pruning | preserves quality under context limits |

```mermaid
flowchart LR
    A[Input signal] --> B[Memory classification]
    B --> C[Store writes and indexing]
    C --> D[Hybrid retrieval]
    D --> E[Context budget fit]
    E --> F[Response + post-hooks]
```
