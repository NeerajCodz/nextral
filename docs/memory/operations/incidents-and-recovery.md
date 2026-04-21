# Incidents and Recovery

## 1. Incident categories

| Category | Example |
|---|---|
| Retrieval outage | both vector and graph retrieval unavailable |
| Write-path outage | extraction/embedding writes failing |
| Graph drift | duplicate/conflicting graph structures rising |
| Data consistency drift | index count diverges from vector/graph counts |
| Deletion failure | forget request partially completed |

## 2. Triage sequence

1. identify impacted pipeline stage(s)
2. assess user impact radius (`user_id` subset vs global)
3. decide degraded mode or partial shutdown policy
4. isolate failing dependency/store
5. initiate replay/recovery plan

## 3. Degraded modes

| Condition | Allowed degraded mode |
|---|---|
| graph path down | vector-only retrieval with marker |
| vector path down | graph-only retrieval with marker |
| consolidation down | continue session + queue backlog |
| graphify down | continue vector writes, replay graphify later |

## 4. Recovery runbooks

### Retrieval outage

1. verify Qdrant/Neo4j health and network paths
2. switch to single-path retrieval if one path healthy
3. drain backlog and restore dual-path operation
4. validate retrieval quality sample

### Write-path outage

1. pause non-critical extraction jobs
2. preserve raw transcripts and enqueue replay
3. restore embed/write dependencies
4. replay from durable queue with idempotent keys

### Deletion failure

1. mark request as incomplete with explicit store checklist
2. retry failed store operations
3. block completion until all store confirmations succeed
4. issue compliance audit record

## 5. Post-incident review checklist

- root cause and blast radius
- timeline of detection and mitigation
- missing alert or noisy alert analysis
- contract/runbook updates required
- replay integrity verification

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
