# 16 · Memory Architecture

> Memory is the moat. Wrong here, everything compounds wrong.

---

## What "memory" means here

Not "chat history." Memory in this system is a multi-layered representation of a user's:

- **Projects** — what they're building, why, status, decisions
- **Execution** — what they did, when, on what
- **Mentorship** — conversations and resolutions
- **Capability** — skill signals over time
- **Artifacts** — outputs they produced
- **Reflections** — explicit user statements about themselves

The memory layer's job is to make all of this **retrievable, summarizable, and trustworthy** for the AI orchestrator.

---

## Layers

```
┌──────────────────────────────────────────────────────────────┐
│  L0 · Raw events (append-only log)                           │
│    every state change, message, signal, decision             │
├──────────────────────────────────────────────────────────────┤
│  L1 · Normalized entities (relational tables)                │
│    projects, tasks, conversations, messages, artifacts,      │
│    capability scores, sessions                               │
├──────────────────────────────────────────────────────────────┤
│  L2 · Semantic index (vector embeddings)                     │
│    chunks of messages, project descriptions, artifacts       │
├──────────────────────────────────────────────────────────────┤
│  L3 · Temporal / relational graph                            │
│    "project X led to artifact Y"; "user revisited concept Z" │
├──────────────────────────────────────────────────────────────┤
│  L4 · Compressed summaries                                   │
│    weekly digests, project briefs, capability narratives     │
└──────────────────────────────────────────────────────────────┘
```

L0 is the source of truth. L1–L4 are derived; they can be rebuilt by replay.

---

## Why these layers

- **L0** gives us **auditability and replay**. If memory gets corrupted, we re-derive from events.
- **L1** gives us **fast structured queries** (today's tasks, this project's history).
- **L2** gives us **semantic recall** ("what did I figure out about X last month?").
- **L3** gives us **causal narratives** ("how did I get from problem A to artifact B?").
- **L4** gives us **prompt-budget-friendly context** for AI calls. Naïve transcript dumping is a token disaster.

The retrieval order for AI context assembly is **L4 → L3 → L2 → L1**. We never paste L0 into a prompt.

---

## Write path

1. User action or workflow signal arrives.
2. Engine emits a domain event → appended to L0.
3. Event dispatcher updates L1 (writes/updates rows).
4. Background workers update L2 (re-embed affected chunks) and L3 (graph edges).
5. Periodic compactor refreshes L4 summaries on a cadence (e.g., end-of-day for daily summary, weekly for week summary).

All steps are crash-safe via L0; on restart, partially-derived layers are re-derived.

---

## Read path (for AI orchestrator)

Given a `context_policy` from a TaskSpec:

1. Pull current state from L1 (project, tasks, active conversation).
2. Pull recent L4 summaries that overlap with the task scope.
3. Run L2 vector search keyed by the query, filtered by L3 relationships if specified.
4. Concatenate within budget; drop lowest-score chunks if over.
5. Return assembled context + a manifest of what was included (for audit).

The orchestrator never reaches into L0 for AI context. L0 is for audit and replay only.

---

## Entity sketch (L1 — to be refined in `12-domain-models.md`)

- `User` (single-user system v0)
- `Project` — top-level container
- `Roadmap`, `RoadmapItem` — plan structure
- `Task` — execution unit
- `Session` — a contiguous work session
- `Conversation`, `Message` — mentor interactions
- `WorkflowSignal` — IDE/desktop sensing
- `CapabilityScore` — capability deltas over time
- `Artifact` — generated outputs
- `Reflection` — user-authored notes about self/process

Domain modeling will be expanded in `12-domain-models.md`.

---

## Retrieval semantics

- **Relevance scoring** uses vector similarity + lightweight reranker. Reranker is local; cloud rerank is opt-in.
- **Freshness boost** for state-bearing entities; less for reflective ones.
- **Project scope filter** is mandatory unless explicitly cross-project.
- **Trust scoring** — content sourced from user reflections is weighted higher than AI-generated summaries when both apply.

---

## Compression strategy

- **End-of-day summary** of session activity → L4.
- **Weekly project brief** rolling up daily summaries → L4.
- **Capability narrative** rolling up signals into "what you got better at" → L4.
- **Mentor session summary** at the close of each conversation → L4, linked to L1 conversation.

Summaries are themselves entities with provenance — we can always show "why does the AI think I'm good at X?" by tracing the L4 back to L0.

---

## Memory ownership and portability

- All memory lives in the local store. Export = SQLite dump + blob bundle + L0 event log + L4 summaries.
- Import on a new device = replay L0 → rebuild L1–L4.
- This is also our **disaster recovery story**.

---

## Privacy controls

- Per-project privacy levels: `local-only`, `syncable`, `shareable`. Affects sync behavior.
- Selective forget: a user can delete a project, a conversation, or an artifact. Cascade rules update L1–L4 and emit a *forget event* into L0 (audit trail of deletion).
- Hard delete: a "shred" command nukes the L0 events too, leaving an opaque tombstone.

---

## Open decisions

- Embedding chunking strategy (semantic vs fixed-token).
- Reranker model choice (or no reranker for v0).
- Graph DB: separate (e.g., embedded Neo4j-like) vs derive on the fly from L1+L2.
- Summarizer model: local-only or cloud-allowed for L4 generation.

Tracked in `45-next-steps.md`.

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [12-domain-models.md](12-domain-models.md)
- [13-event-flows.md](13-event-flows.md)
- [14-offline-first-architecture.md](14-offline-first-architecture.md)
- [15-ai-system-design.md](15-ai-system-design.md)
- [17-security-architecture.md](17-security-architecture.md)
