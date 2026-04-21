# Session Memory

## Index

1. [Purpose](#purpose)
2. [Storage Tiers](#storage-tiers)
3. [Lifecycle Behavior](#lifecycle-behavior)
4. [What It Enables](#what-it-enables)
5. [Failure Mode It Prevents](#failure-mode-it-prevents)
6. [Builder Addendum: Expanded Control Surface](#builder-addendum-expanded-control-surface)


## Purpose

Session memory preserves continuity inside the active conversation while keeping retrieval low-latency.

## Storage Tiers

| Tier | Backend | Data | Latency target | Retention |
|---|---|---|---|---|
| Hot | Redis | Last 20 messages (`session:{id}:messages`) | Sub-millisecond | TTL around 2h |
| Warm | PostgreSQL | Full transcript + session summaries | ~5ms | Configured window (commonly 90 days) |

## Lifecycle Behavior

1. Every new message is appended to Redis.
2. `ContextAssembler` reads the most recent tail for immediate context.
3. If message count exceeds 20, older turns are persisted to PostgreSQL and trimmed from Redis.
4. Idle/close events trigger summary generation into `session_summaries`.

## What It Enables

- Fast local conversational continuity
- Date/topic search across older turns
- A bounded hot window that avoids stale-context overhead

## Failure Mode It Prevents

Without session memory tiering, the agent either loses the thread or pays high latency reloading full transcripts every turn.

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
