# 15 · AI System Design

## Goals

1. **Provider-agnostic.** No business logic ever depends on which model serves a request.
2. **Local-first.** Default routing favors local; cloud is opt-in enhancement.
3. **Deterministic where possible.** Same input → same output in tests; controlled non-determinism elsewhere.
4. **Auditable.** Every AI call is logged with model, prompt, context, output, and a correlation ID.
5. **Resilient.** Provider failures degrade visibly; never silently produce worse output.

---

## Layered AI architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Engine code (Mentor, Capability, Artifact, …)               │
│     calls   ai.run(task: TaskSpec) → TaskResult              │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│  Orchestrator                                                │
│   - selects route (local | cloud | cached | fallback)        │
│   - assembles context (RAG via memory layer)                 │
│   - assembles prompt (template + injected context + tools)   │
│   - enforces safety + auditing                               │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│  Provider abstraction                                        │
│   Provider interface:                                        │
│     chat(messages, opts) → stream<chunk>                     │
│     embed(texts) → vec[]                                     │
│     models() → ModelInfo[]                                   │
│                                                              │
│   Implementations:                                           │
│     OllamaProvider, LMStudioProvider, LlamaCppProvider,      │
│     ClaudeProvider, OpenAIProvider, GeminiProvider,          │
│     DeepSeekProvider, GroqProvider                           │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
                    network / IPC
```

---

## TaskSpec contract (proposed)

```
TaskSpec {
  task_kind: enum (mentor_turn, capability_score, artifact_draft,
                   summarize, classify, embed, ...)
  inputs: { prompt, attachments, conversation_id, project_id, ... }
  context_policy: {
    retrieval: { sources: [...], top_k, recency_bias }
    redaction: [PII, secrets]
  }
  routing_policy: {
    preferred: local | cloud | auto
    quality_floor: low | medium | high
    latency_budget_ms?
    cost_budget_cents?
  }
  safety_policy: {
    refuse_categories: [...]
    require_explanations: bool
  }
  determinism: { temperature, seed?, cache_key? }
  audit: { user_id, correlation_id }
}
```

Engines never specify a *model name* — they specify what the task needs. The orchestrator picks the model.

---

## Routing logic (orchestrator)

1. Resolve `routing_policy.preferred` against availability:
   - `local` → local provider; if unavailable → fallback per policy
   - `cloud` → cloud; if user has no cloud key configured → fall back to local with visible warning
   - `auto` → choose by `quality_floor` × current latency profile × cost budget
2. Apply `quality_floor` to filter candidate models.
3. Apply `latency_budget_ms` to prefer fast providers when slack is tight.
4. Apply `cost_budget_cents` per turn; refuse to route if no cheap path satisfies.
5. Emit a *routing decision event* with the chosen provider, model, and reasoning — this is logged and visible in audit UI.

---

## Context assembly (RAG)

Context is built by the orchestrator, not the engine. The engine asks for context via `context_policy`; the memory layer returns sources; the orchestrator concatenates within prompt budget.

Rules:

- **Relevance over recency, except for state.** Mentor turns prefer relevant memories; current project state is always included.
- **Compressed summaries first.** We retrieve compressed memory before raw transcripts.
- **Hard prompt budget.** If retrieval overflows, drop lowest-score items, log what was dropped.
- **Never inject untrusted user input as instructions.** Treat user data as data, not as system prompts. (See `17-security-architecture.md` re: prompt injection.)

See `16-memory-architecture.md` for retrieval mechanics.

---

## Determinism strategy

- **Embeddings:** deterministic per (model, input). Stored with a content hash so cache hits are exact.
- **Generations:** when `determinism.seed` is set and provider supports it, pin seed; else mark turn as non-deterministic in audit log.
- **Tests:** in test environments, the orchestrator routes to a **recorded-response provider** (golden cassettes) by default; only opt-in tests hit live models.

---

## Tool use

When an engine task requires tools (e.g. mentor pulling from project files), tool calls go through:

1. A typed tool registry inside the daemon (no remote tools yet).
2. Each tool call is logged with arguments and result.
3. Tool definitions are version-pinned; changing a tool's signature requires bumping its version and updating dependents.

---

## Failure model

| Failure | Behavior |
| --- | --- |
| Local provider crashed | Surface banner; route to cloud if allowed; else explicit error |
| Cloud provider 5xx | One retry with jitter; then fallback to local; log degradation |
| Cloud auth expired | Visible auth prompt; queue work, don't lose it |
| Context overflow | Compress harder; if still over budget, return a typed error not a silent truncation |
| Tool errored | Surface to engine; mentor turn explains the tool issue rather than hallucinating success |

**Never:** silently swap a worse model. **Never:** silently drop context. **Never:** return cached output beyond its declared TTL without indicating it's cached.

---

## Caching

Two layers:

1. **Exact-match prompt cache** — keyed by (model, prompt-hash, temperature). Useful for re-asks during dev and for cheap mentor "what did I say last time" reflections.
2. **Semantic cache** *(later)* — embeds queries; returns near-hits with similarity score visible to the user. Off by default; on means user explicitly traded freshness for cost/latency.

---

## Cost & latency observability

Every AI call records:

- model + provider
- prompt token count, response token count
- wall-clock latency
- estimated cost (provider-specific table)
- routing reason

These power both the user's "AI usage" panel and the engineering dashboards. See `18-observability-architecture.md`.

---

## Safety

- **Refusal taxonomy** stays consistent across providers (we own the taxonomy, providers don't dictate).
- **Prompt injection defense** lives at context-assembly time: user data is wrapped in delimiters, never concatenated raw into system prompts. See `17-security-architecture.md`.
- **PII redaction** is policy-driven per `context_policy.redaction`. Defaults are conservative.

---

## Open decisions

Tracked in `45-next-steps.md`:

- Which cloud providers to support at GA (vs Phase 1 subset).
- Whether the orchestrator runs inside the Rust daemon or in a thin sidecar process (depends on async ecosystem comfort).
- Embedding model upgrade path (when do we re-embed everything?).
- Long-context handling strategy (chunked summarization vs hierarchical retrieval).

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [14-offline-first-architecture.md](14-offline-first-architecture.md)
- [16-memory-architecture.md](16-memory-architecture.md)
- [17-security-architecture.md](17-security-architecture.md)
- [18-observability-architecture.md](18-observability-architecture.md)
- [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md)
