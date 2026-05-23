# 13 · Event Flows

> Key end-to-end flows. Describes what happens between user action and persisted state.

**Status:** scaffold — three illustrative flows below; more added as engines come online.

---

## Flow 1 · Mentor turn

```
User types message in Desktop UI
  → Desktop UI POSTs to daemon (HTTP loopback)
    → Mentor Engine receives command
      → Builds TaskSpec (kind = mentor_turn)
        → Orchestrator (15-ai-system-design.md)
          → Memory Layer assembles context (16-memory-architecture.md)
          → Provider abstraction calls chosen model
          → Stream chunks back through orchestrator → Mentor → UI
        → On stream close:
          → L0 append: MessageSent, MessageReceived, AICallCompleted
          → L1 update: Conversation, Message rows
          → L2 enqueue: re-embed new content
          → Trust Engine audit log entry
```

Failures: degraded model availability → orchestrator visible-fallback; partial stream → keep partial in L0 with `incomplete=true`.

---

## Flow 2 · Workflow signal from VSCode

```
VSCode extension observes file edit / file open / language activity
  → Batches signals locally (extension memory)
  → Pushes to daemon over WebSocket
    → Workflow Engine validates + enriches
      → L0 append: WorkflowSignal events
      → Async: Capability Engine consumes signals → updates scores
      → Async: Memory Layer L2 re-embed if signal references project content
```

Failures: daemon unavailable → extension queues locally, retries with backoff; never blocks the user's editing.

---

## Flow 3 · Daily summary (background)

```
Scheduled (end-of-day per user TZ)
  → Cognition Engine assembles day's L1 + L0 slice
    → TaskSpec (kind = summarize, context_policy = today only)
      → Orchestrator → local model preferred
        → L4 write: DailySummary entity
        → L0 append: SummaryGenerated
```

Failures: model unavailable → retry next interval; never silently skip.

---

## Cross-cutting

- **Correlation IDs** flow from UI action through every event. Audit can reconstruct full causality.
- **Idempotency keys** on user-initiated commands (UI generates a UUID per submission) so retries don't double-write.
- **Backpressure**: L2 re-embed is a queue; bursts are smoothed, never block user actions.

---

## To add

- Project creation flow
- Artifact generation flow
- Sync flow (when sync ships)
- Cross-device conflict resolution flow
- Export flow

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [11-service-boundaries.md](11-service-boundaries.md)
- [12-domain-models.md](12-domain-models.md)
- [15-ai-system-design.md](15-ai-system-design.md)
- [16-memory-architecture.md](16-memory-architecture.md)
- [18-observability-architecture.md](18-observability-architecture.md)
