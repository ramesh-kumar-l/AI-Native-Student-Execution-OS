# 10 · System Architecture

> **Status: stack decisions PENDING.** This file describes the logical architecture (stack-agnostic) plus a stack-options matrix awaiting user decision. References to specific technologies elsewhere in the memory bank are *proposals*, not commitments, until an ADR lands in `50-adrs/`.

---

## High-level shape

```
┌──────────────────────────────────────────────────────────────────────┐
│                        User-facing surfaces                          │
│  ┌────────────────────┐    ┌────────────────────┐    ┌─────────────┐ │
│  │  Desktop OS app    │    │  VSCode extension  │    │ Mobile (L8r)│ │
│  └─────────┬──────────┘    └─────────┬──────────┘    └──────┬──────┘ │
│            │                         │                       │       │
└────────────┼─────────────────────────┼───────────────────────┼───────┘
             │                         │                       │
             ▼                         ▼                       ▼
┌──────────────────────────────────────────────────────────────────────┐
│                       Local Cognition Daemon                         │
│  (single source of truth for one user on one device; runs locally)   │
│                                                                      │
│   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│   │Cognition │ │Execution │ │Capability│ │ Mentor   │ │ Workflow │  │
│   │  Engine  │ │  Engine  │ │  Engine  │ │  Engine  │ │  Engine  │  │
│   └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
│   ┌──────────┐ ┌──────────┐                                          │
│   │ Artifact │ │  Trust   │   (seven engines total)                 │
│   │  Engine  │ │  Engine  │                                          │
│   └──────────┘ └──────────┘                                          │
│                                                                      │
│   ┌──────────────────────────────────────────────────────────────┐  │
│   │  AI Orchestration Layer (provider-agnostic; routes local↔cloud)│ │
│   └──────────────────────────────────────────────────────────────┘  │
│   ┌──────────────────────────────────────────────────────────────┐  │
│   │  Memory Layer (semantic + temporal + relational + compressed) │ │
│   └──────────────────────────────────────────────────────────────┘  │
│   ┌──────────────────────────────────────────────────────────────┐  │
│   │  Storage (local DB + vector + blob)                            │ │
│   └──────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────┘
             │ optional, eventual-consistent
             ▼
┌──────────────────────────────────────────────────────────────────────┐
│                       Cloud Sync Service                             │
│  - device-to-device sync           - encrypted backup                │
│  - cross-device cognition view     - cloud LLM proxy (optional)      │
└──────────────────────────────────────────────────────────────────────┘
```

The cloud is **optional**. The local daemon is the source of truth.

---

## Architectural principles

These constrain every implementation decision:

1. **Local daemon as source of truth.** Surfaces (desktop UI, VSCode extension) talk to the daemon over local IPC. The daemon owns state.
2. **Provider-agnostic AI.** All model calls go through one abstraction. No vendor names in business logic.
3. **Offline-first.** Core flows function fully offline. Sync resumes when network returns.
4. **Event-driven internally.** Engines communicate via an internal event bus, not direct calls — enables observability, replay, and testability.
5. **Memory is its own layer.** Not embedded in any engine. Engines read/write through memory interfaces.
6. **Auditability everywhere.** Every AI call, every memory write, every sync action is logged with correlation IDs.
7. **No silent failures.** Every degraded mode is visible to the user.

---

## Component responsibilities (logical, stack-agnostic)

### Surfaces
- **Desktop OS app** — UI for roadmap, projects, mentor chat, artifact review, settings.
- **VSCode extension** — workflow sensing (active file, language, project context), inline architecture hints, capability signal extraction.
- **Mobile (future)** — ambient reflection, voice, reminders.

### Local Cognition Daemon
Single long-running process per user/device. Hosts the seven engines, AI orchestration, memory, and storage. Exposes a local IPC interface (TBD: gRPC over UDS / WebSocket / HTTP-loopback — see Options below).

### Seven engines
| Engine | One-line | Detail |
| --- | --- | --- |
| Cognition | Owns persistent memory and retrieval | `16-memory-architecture.md` |
| Execution | Owns plans, projects, tasks, tracking | `12-domain-models.md` |
| Capability | Computes capability signal from workflow + execution data | `12-domain-models.md` |
| Mentor | Orchestrates AI mentorship turns (RAG, context assembly) | `15-ai-system-design.md` |
| Workflow | Senses workflow signals (IDE, desktop activity, time) | (TBD doc) |
| Artifact | Generates portfolio entries, narratives, project pages | (TBD doc) |
| Trust | Governs reliability gates, audit, explainability surfaces | `17-security-architecture.md`, `18-observability-architecture.md` |

### AI Orchestration Layer
Provider-agnostic router. See `15-ai-system-design.md`.

### Memory Layer
Multi-mode store: relational + vector + temporal + compressed-summary. See `16-memory-architecture.md`.

### Cloud Sync Service (optional)
Eventual-consistent device-to-device sync with end-to-end encryption. See `14-offline-first-architecture.md`.

---

## Stack-options matrix (AWAITING DECISION)

Each row below has a recommendation and tradeoffs. **No stack decision is final until an ADR is merged.** When you (the user) decide, we'll write the ADR and update this section to point at it.

### Desktop framework

| Option | Pros | Cons |
| --- | --- | --- |
| **Tauri 2.x** *(recommended)* | Tiny binary (~10MB), Rust core fits cognition daemon, system webview = native feel, modern, strong roadmap | Rust learning curve; smaller ecosystem than Electron; webview parity quirks |
| Electron | Largest ecosystem, easiest hiring, predictable | Heavy (~100MB+), worse perf, less "trust-first" feel, Chromium baggage |
| Native (Swift/WinUI + GTK or Flutter desktop) | Best perf and platform polish | 3× the engineering for 3 OSs; not justified pre-PMF |

**Recommendation:** Tauri 2.x with React (or Svelte) frontend, Rust backend as the cognition daemon host. Matches local-first, offline-first, and trust-first design.

### Local DB (relational)

| Option | Pros | Cons |
| --- | --- | --- |
| **SQLite via `rusqlite` / `libsql`** *(recommended)* | Battle-tested, single-file, easy backup, FTS5 for keyword search, ubiquitous | No first-class vector support (handled separately) |
| DuckDB | Excellent analytics, columnar | Overkill for OLTP-style writes; not ideal as primary store |
| Postgres-embedded (e.g. pglite) | SQL fidelity to cloud | Heavier; not as mature for embedded use |

**Recommendation:** SQLite (libsql variant for sync potential).

### Vector store

| Option | Pros | Cons |
| --- | --- | --- |
| **`sqlite-vss` / `sqlite-vec`** *(recommended for v0)* | Co-located with relational data; one store to back up | Less mature; smaller scale ceiling |
| LanceDB | Embedded, Rust-native, great Arrow integration | Two stores to manage |
| Qdrant local | Mature server, gRPC | Heavier; runs as separate process |

**Recommendation:** `sqlite-vec` for v0 to keep one storage surface; revisit if scale or quality demands change.

### Local LLM runtime

| Option | Pros | Cons |
| --- | --- | --- |
| **Ollama** *(primary)* | Easiest install on all 3 OSs; HTTP API; widest model coverage | Wraps llama.cpp — not the fastest |
| LM Studio | Good UX | GUI-first; less scriptable |
| llama.cpp direct | Max control | More integration work |

**Recommendation:** Ollama as primary; abstraction allows LM Studio / llama.cpp swap.

### Cloud LLM providers (through abstraction)

Support: Claude, Gemini, OpenAI, DeepSeek, Groq via one provider interface. No business logic depends on which vendor is in use.

### IPC between surfaces and daemon

| Option | Pros | Cons |
| --- | --- | --- |
| **HTTP loopback + WebSocket** *(recommended)* | Universal — VSCode extension can speak it natively; easy to debug | Slightly more boilerplate than IPC |
| gRPC over Unix Domain Sockets / Named Pipes | Fast, typed | Harder for VSCode extension to integrate |
| Tauri commands only | Trivial inside the desktop app | VSCode extension has no path |

**Recommendation:** HTTP loopback + WebSocket — same interface for desktop and VSCode.

### Sync backend (optional cloud component)

Deferred until Phase 1+. Likely candidates: a thin service over Postgres + S3-compatible blob, or CRDT-based (Automerge / Yjs over a sync server). ADR needed before any code.

### Embeddings model (local)

| Option | Pros | Cons |
| --- | --- | --- |
| **`nomic-embed-text` (Ollama)** | Strong quality/size, open | English-leaning |
| `bge-m3` | Multilingual, strong | Larger |
| `all-MiniLM-L6-v2` | Tiny, fast | Quality ceiling |

**Recommendation:** `nomic-embed-text` for v0.

---

## Build & language summary (if all recommendations accepted)

- **Daemon:** Rust (Tauri 2.x backend)
- **Surfaces:** TypeScript + React (desktop renderer) + TypeScript (VSCode extension)
- **Storage:** SQLite (libsql) + sqlite-vec + local blob dir
- **LLM:** Ollama (local) + provider abstraction for cloud
- **IPC:** HTTP loopback + WebSocket
- **CI:** GitHub Actions; cross-platform Tauri builds for macOS / Windows / Linux

If the user picks different options, all subsequent memory-bank files referring to "Rust daemon" or "Tauri" should be revised in the same change.

---

## What this architecture explicitly enables

- Run the entire core product with **no network and no cloud account**.
- Swap any AI vendor in 1 file change.
- Add a mobile surface later without re-architecting (daemon protocol is the contract).
- Audit every AI decision and memory write.

## What this architecture explicitly does NOT include (yet)

- Multi-user / multi-tenant inside one daemon (post-PMF).
- Federated cognition across users (long-horizon).
- Third-party engine plugins (long-horizon).

---

## Related

- [11-service-boundaries.md](11-service-boundaries.md)
- [12-domain-models.md](12-domain-models.md)
- [13-event-flows.md](13-event-flows.md)
- [14-offline-first-architecture.md](14-offline-first-architecture.md)
- [15-ai-system-design.md](15-ai-system-design.md)
- [16-memory-architecture.md](16-memory-architecture.md)
- [17-security-architecture.md](17-security-architecture.md)
- [18-observability-architecture.md](18-observability-architecture.md)
- [45-next-steps.md](45-next-steps.md) — pending stack decisions tracked here
