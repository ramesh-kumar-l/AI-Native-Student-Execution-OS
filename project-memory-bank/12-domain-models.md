# 12 · Domain Models

> Core entities and their relationships, stack-agnostic. Field-level schemas are deferred until storage tech is chosen.

**Status:** scaffold — to be expanded into formal schemas in the same ADR that picks the storage stack.

---

## Top-level entities

```
User ─┬─ Project ─┬─ Roadmap ─── RoadmapItem
      │           ├─ Task
      │           ├─ Session ── WorkflowSignal*
      │           ├─ Conversation ── Message*
      │           ├─ Artifact*
      │           └─ Reflection*
      ├─ CapabilityScore* (cross-project)
      └─ Settings
```

`*` = many per parent.

---

## Entity sketches

### User
Single-user-per-device for v0. Identity is local; cloud account is an optional sync identity.

### Project
A bounded scope of work. Has a goal, an active roadmap, status, and a creation date. Container for tasks, sessions, conversations, artifacts.

### Roadmap / RoadmapItem
Plan structure. Items are ordered, can have dependencies, and reference tasks.

### Task
Execution unit. Status (`planned | active | done | dropped`), estimate, links to roadmap items and sessions.

### Session
A contiguous work session: start, end, project context, mentor conversation (if any), workflow signals collected.

### Conversation / Message
Mentor exchanges. Conversations have a topic and a closure summary. Messages have role, content, tool calls, and an audit reference.

### WorkflowSignal
Atomic signal from VSCode / desktop / OS. Type, payload, timestamp, source.

### CapabilityScore
A scored signal of capability over time. Skill, score, basis (which signals contributed), as-of date.

### Artifact
Generated output: portfolio entry, project page, narrative, exported bundle. Linked to project + source data.

### Reflection
User-authored note about themselves or their process. Carries higher trust weight than AI-generated summaries.

### Settings
User preferences: AI routing policy, privacy levels per project, sync configuration, observability opt-ins.

---

## Cross-cutting metadata

Every entity carries:

- `id` (ULID or UUIDv7 — decision pending)
- `created_at`, `updated_at`
- `version` (Lamport-style for sync)
- `tenant_id` (= user id for v0; reserved for future multi-tenant)
- `privacy_level` where applicable (`local-only | syncable | shareable`)

---

## Open items

- ID strategy (ULID vs UUIDv7) — ADR pending.
- Soft-delete vs hard-delete defaults.
- How conversations relate to sessions when a conversation spans multiple sessions.
- Capability skill taxonomy: free-form vs controlled vocabulary vs embeddings-only.

---

## Related

- [11-service-boundaries.md](11-service-boundaries.md)
- [13-event-flows.md](13-event-flows.md)
- [16-memory-architecture.md](16-memory-architecture.md)
