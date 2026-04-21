# Memory Contracts

## Index

1. [Documents](#documents)
2. [Contract principles](#contract-principles)
3. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Documents

| File | Scope |
|---|---|
| `memory-record-schema.md` | canonical memory payloads and storage-mapping schema |
| `retrieval-contract.md` | retrieval API input/output, ranking, and token-budget contract |
| `graphify-contract.md` | graph extraction payload, merge behavior, and confidence semantics |
| `prospective-contract.md` | reminders/follow-ups state model and scheduler contract |

## Contract principles

1. every field has ownership and validation rules
2. ids are stable across vector/index/graph boundaries
3. lifecycle transitions are explicit and auditable
4. deletion and redaction are first-class operations

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
