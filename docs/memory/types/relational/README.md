# Relational Memory

## Index

1. [Purpose](#purpose)
2. [Graphify Pipeline](#graphify-pipeline)
3. [Common Node Labels](#common-node-labels)
4. [Common Relationship Types](#common-relationship-types)
5. [Retrieval Behavior](#retrieval-behavior)
6. [Failure Mode It Prevents](#failure-mode-it-prevents)
7. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Purpose

Relational memory captures how entities connect, not just what text looks similar. It is implemented as a graph in Neo4j through the Graphify pipeline.

## Graphify Pipeline

1. A memory chunk is written to long-term memory.
2. Graphify runs a structured Claude extraction for nodes, edges, direction, and confidence.
3. Neo4j writes use `MERGE` for nodes and relationships (idempotent updates).
4. Edge metadata is maintained (`source_memory_id`, `confidence`, `created_at`, `last_confirmed_at`).

## Common Node Labels

`Person`, `Project`, `Concept`, `Decision`, `Task`, `Note`, `Event`, `Goal`, `Tool`, `Organisation`, `Place`, `Technology`

## Common Relationship Types

`WORKS_ON`, `DEPENDS_ON`, `BLOCKS`, `ABOUT`, `HAS_ROLE`, `RELATED_TO`, `PART_OF`, `SUPPORTS`, `CONTRADICTS`, `PRECEDES`, plus collaboration and ownership edges.

## Retrieval Behavior

1. Match entities extracted from the query.
2. Traverse 1-2 hops in Neo4j.
3. Return structurally related nodes and edge metadata.

This output is merged with vector candidates by `HybridMemoryRetriever`.

## Failure Mode It Prevents

Vector search can miss dependency chains and participant links. Relational memory restores explicit "who/what/depends-on-what" context.

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
