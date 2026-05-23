# 14 · Offline-First Architecture

## Contract

> Every core workflow — planning, tracking, mentor conversations (text), workflow sensing, artifact viewing, memory recall — **must complete with no network connection**. Cloud is an enhancement layer, never a dependency for core.

Failure to meet this contract for any "core" feature is a design defect, not a known limitation.

---

## What "core" means (offline-mandatory)

| Workflow | Offline must work |
| --- | --- |
| Open the app, view today's plan | Yes |
| Edit / create projects, tasks, plans | Yes |
| Conversational mentor turn (text) | Yes — local model |
| VSCode workflow sensing → memory write | Yes |
| Capability dashboard | Yes |
| Search past mentor conversations / projects | Yes |
| View existing artifacts | Yes |
| Export user data | Yes |

## What is "enhanced" (cloud-optional)

| Workflow | Cloud enhances |
| --- | --- |
| Higher-quality mentor turns | Cloud LLM routing |
| Image/voice features | Cloud multimodal |
| Cross-device cognition view | Sync service |
| Encrypted off-site backup | Sync service |
| Sharing artifacts publicly | Cloud-hosted artifact pages |

If a cloud-enhanced feature degrades offline, the UI must show that degradation **explicitly** — never silently fail or hang.

---

## Sync model

We're not building real-time collaborative editing. We're building **single-user, multi-device eventual consistency**.

### Data classes

| Class | Examples | Conflict policy |
| --- | --- | --- |
| **Append-only events** | mentor messages, workflow signals, audit log entries | Merge by timestamp; no conflicts |
| **Last-write-wins documents** | task status, plan items, settings | LWW by Lamport-style version; client-side resolution UI for high-stakes overwrites |
| **CRDT documents** *(later)* | project notes, roadmap (free-form structured) | CRDT (Automerge/Yjs) for conflict-free merge |
| **Immutable blobs** | artifact files, exports | Content-addressed; no conflict possible |

Phase 0/1 lives with classes 1, 2, and 4. CRDTs are added when free-form editing surfaces ship.

### Sync engine sketch (post-Phase-0)

```
device A ──┐
           ├─► sync coordinator ──► encrypted blob store
device B ──┘                  └──► event log (per user)
```

- All payloads encrypted with a **user-held key** before leaving the device.
- The sync coordinator never sees plaintext memory.
- Conflict resolution happens **on-device**, never in the cloud.

Detailed sync design lives in a future ADR.

---

## Local-first guarantees the user is promised

These are user-facing promises; they constrain engineering:

1. Your data lives on your machine. You can find it, copy it, back it up.
2. Disabling cloud sync never breaks the product.
3. Exports are first-class: full JSON / SQLite dump + artifact bundle.
4. Deleting your account locally deletes everything locally; deleting in cloud removes cloud copy.
5. There is no telemetry the user cannot inspect and turn off.

---

## Failure modes & UX surfacing

| Failure | UX behavior |
| --- | --- |
| No network | Banner: "Offline — cloud LLM unavailable, using local model." No blocking modal. |
| Local model not running | Banner: "Local model not running — `ollama serve` to start, or enable cloud routing." |
| Sync conflict (high-stakes) | Inline diff UI on next open, never silent overwrite. |
| Cloud LLM rate-limited | Auto-fallback to local model with notice. |
| Disk full | Surface in settings; block new writes with clear message, never corrupt existing data. |

The pattern: **degraded mode is always visible, never silent**.

---

## Engineering constraints derived from this contract

These appear elsewhere in the memory bank; they originate here.

- The **AI provider abstraction** must support a working local default. (`15-ai-system-design.md`)
- The **memory layer** must function with no network. (`16-memory-architecture.md`)
- The **VSCode extension** must speak to the local daemon directly, not to a cloud endpoint. (`10-system-architecture.md`)
- **Observability** must work locally — telemetry batching can wait for network but observation must not. (`18-observability-architecture.md`)
- **First-run UX** must work without sign-in. Sign-in is for sync; never for unlocking features.

---

## Open questions (track in `45-next-steps.md`)

- Exact sync protocol (CRDT vs operational; Automerge vs Yjs vs custom).
- Encryption key management: passphrase-derived vs OS-keychain vs hybrid.
- Sync coordinator self-host story (does an OSS sync server exist users can run?).

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [15-ai-system-design.md](15-ai-system-design.md)
- [16-memory-architecture.md](16-memory-architecture.md)
- [17-security-architecture.md](17-security-architecture.md)
