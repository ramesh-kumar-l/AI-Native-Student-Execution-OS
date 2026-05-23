# 11 · Service Boundaries

> Stack-agnostic logical boundaries between the seven engines plus the AI, memory, and storage layers. Implementation details (process boundaries, IPC mechanism) are in `10-system-architecture.md`.

**Status:** scaffold — to be expanded as engine interfaces firm up.

---

## Engine boundary table

| Engine | Owns | Reads | Writes |
| --- | --- | --- | --- |
| Cognition | Memory layers L0–L4 | L0 (own) | L0–L4 (own) |
| Execution | Projects, roadmaps, tasks, sessions | L1, L4 (memory) | Domain events to L0 |
| Capability | Capability scores & narratives | Workflow signals (Workflow), Execution data, Mentor outcomes | Capability events to L0; L4 narratives |
| Mentor | Conversation orchestration | L1–L4 via Cognition; AI via Orchestrator | Mentor events to L0 |
| Workflow | Workflow signal capture (from VSCode + desktop) | OS/IDE events | Signal events to L0 |
| Artifact | Artifact generation & storage | L1, L4; AI via Orchestrator | Artifact entities + blobs; events to L0 |
| Trust | Audit log, reliability gates, explainability surfaces | All engines' events | Audit summaries; degradation flags |

---

## Boundary rules

1. **No engine reaches into another engine's storage.** All cross-engine reads go through documented interfaces.
2. **Engines never call AI providers directly.** They go through the Orchestrator (`15-ai-system-design.md`).
3. **All writes are events first, state second.** State is derived from event log.
4. **Trust Engine is read-mostly.** It observes everything but only writes audit/governance state.
5. **Workflow Engine never blocks user actions.** Signal capture is best-effort and queued.

---

## Interfaces (sketch — to be formalized per engine)

Each engine exposes:

- A typed command API (request/response).
- A typed event stream (publish only — others subscribe).
- A health endpoint (for Trust + observability).

Engines do **not** expose their underlying tables.

---

## Open items

- Formal interface definitions per engine (one ADR per engine when interface stabilizes).
- Process model: do all engines run in one daemon process, or do some run as sidecars?
- VSCode extension's relationship to Workflow Engine (it's a *signal source*, not the engine itself).

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [12-domain-models.md](12-domain-models.md)
- [13-event-flows.md](13-event-flows.md)
