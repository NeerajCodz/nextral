# Prospective Memory Contract

## 1. Purpose

Define how reminders, follow-ups, and commitments are represented, scheduled, and completed.

## 2. Canonical reminder record

```json
{
  "id": "rem_uuid",
  "user_id": "usr_uuid",
  "source_memory_id": "mem_uuid",
  "kind": "follow_up",
  "title": "Check Atlas database migration status",
  "details": "Follow up after schema migration window",
  "due_at": "2026-04-12T09:00:00Z",
  "timezone": "Asia/Kolkata",
  "priority": "high",
  "status": "scheduled",
  "attempt_count": 0,
  "last_attempt_at": null,
  "next_attempt_at": "2026-04-12T09:00:00Z",
  "created_at": "2026-04-10T19:30:00Z",
  "updated_at": "2026-04-10T19:30:00Z"
}
```

## 3. State machine

```text
draft -> scheduled -> due -> dispatched -> completed
                         -> failed -> retry_scheduled -> dispatched
scheduled -> cancelled
due -> expired
```

Transitions require explicit reason and actor/system metadata.

## 4. Scheduler contract

Input to scheduler:

- due window
- user policy constraints (quiet hours, channel policy)
- dedupe keys

Output from scheduler:

- dispatched task payload
- dispatch attempt event
- updated reminder state

## 5. Dedupe contract

Reminder dispatch dedupe key:

```text
dedupe_key = hash(user_id + source_memory_id + kind + due_at)
```

Duplicate dispatch for same dedupe key is rejected.

## 6. Reliability contract

- at-least-once scheduling with idempotent dispatch handlers
- retries with bounded backoff policy
- terminal failure marks explicit reason

## 7. Privacy contract

- reminder details inherit source memory privacy defaults
- sensitive reminders require policy check before proactive send
- all reminder state changes are audit-logged

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
