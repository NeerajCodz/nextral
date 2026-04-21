# Memory Pipelines

## Index

1. [Documents](#documents)
2. [Pipeline rules](#pipeline-rules)
3. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Documents

| File | Scope |
|---|---|
| `ingestion-pipeline.md` | request-time message and session write pipeline |
| `consolidation-pipeline.md` | fast/deep memory extraction and durable write pipeline |
| `graphify-pipeline.md` | memory-to-graph extraction and merge pipeline |
| `retrieval-pipeline.md` | dual-path retrieval, rerank, and context merge pipeline |

## Pipeline rules

1. idempotent writes and replay safety
2. explicit failure states and retry classes
3. trace correlation across all pipeline stages

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
