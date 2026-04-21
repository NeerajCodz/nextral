# Semantic Memory

## Index

1. [Purpose](#purpose)
2. [Primary Stores](#primary-stores)
3. [Write-Time Signals](#write-time-signals)
4. [Retrieval Scoring](#retrieval-scoring)
5. [What It Enables](#what-it-enables)
6. [Failure Mode It Prevents](#failure-mode-it-prevents)
7. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Purpose

Semantic memory stores stable knowledge: facts, preferences, decisions, notes, and durable user truths.

## Primary Stores

| Store | Role |
|---|---|
| Qdrant (`memories`, `knowledge`) | Long-term semantic vectors with payload metadata |
| PostgreSQL | Canonical IDs, audit fields, and secondary indexes |

## Write-Time Signals

Each memory is assigned an importance score and metadata such as content type, entities, and privacy level.

## Retrieval Scoring

```text
retrieval_score =
  (0.5 * semantic_similarity) +
  (0.2 * exp(-0.05 * days_since_creation)) +
  (0.2 * importance_score) +
  (0.1 * normalized_access_count)
```

This keeps semantic relevance primary while still factoring recency, durable importance, and repeat access.

## What It Enables

- Consistent recall of user preferences and decisions
- Reuse of knowledge from notes, docs, and journal corpora
- Better continuity across separate sessions and projects

## Failure Mode It Prevents

Without semantic memory, Tony repeatedly relearns stable facts and contradicts previous preferences.

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
