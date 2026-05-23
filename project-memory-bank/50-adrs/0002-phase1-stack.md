# ADR-0002 · Phase 1 Stack Selection

**Date:** 2026-05-23
**Status:** ACCEPTED
**Supersedes:** Pending decisions D-1…D-7 in `45-next-steps.md`

---

## Context

Phase 0 produced a stack-options matrix in `10-system-architecture.md` with seven pending decisions
(D-1…D-7). Phase 1 "Daemon Walking Skeleton" cannot begin until these decisions are formalized.
The user approved proceeding with Phase 1, implicitly accepting all recommended options.

---

## Decisions

| # | Area | Decision | Rationale |
|---|------|----------|-----------|
| D-1 | Desktop framework | **Tauri 2.x** (Rust core + webview) | Smallest binary; Rust daemon; offline-first |
| D-2 | Local DB | **rusqlite 0.31 (bundled SQLite)** | Battle-tested; single-file; FTS5; easy backup |
| D-3 | Vector store | **Pure-Rust cosine similarity on f32 BLOBs in SQLite** | See note below |
| D-4 | Local LLM runtime | **Ollama** (HTTP API at localhost:11434) | Easiest install; largest model coverage |
| D-5 | IPC mechanism | **HTTP loopback + SSE** (Axum 0.7) | Universal — VSCode extension and desktop share one interface |
| D-6 | Frontend framework | **React + TypeScript** (Vite) | Largest ecosystem; good Tauri integration |
| D-7 | Embedding model | **nomic-embed-text** (via Ollama) | Quality/size tradeoff; revisit with data |

**D-3 clarification (sqlite-vec deferral):** The `sqlite-vec` SQLite extension is the target store.
However, its Rust FFI integration requires loading a platform native library (`.so`/`.dll`/`.dylib`)
at runtime, which creates bundling and cross-compilation complexity for Phase 1. The interface
(`VectorStore` trait with `insert_embedding` / `search_similar`) is designed for a drop-in swap.
A follow-on ADR-0003 will govern the sqlite-vec integration once the daemon is running.

**Async DB access pattern:** `tokio-rusqlite` wraps rusqlite on a dedicated thread and exposes
an async `call()` interface. This keeps all request handlers non-blocking without `spawn_blocking`
scatter throughout business logic.

**Cloud provider (Phase 1):** Anthropic Claude (SSE streaming) as the single cloud provider
in Phase 1. Abstraction supports adding OpenAI, Gemini, DeepSeek, Groq in Phase 2.

---

## Consequences

**Positive:**
- Single storage file → trivial backup / export / disaster recovery.
- Tauri Rust core IS the daemon — no separate process to manage.
- HTTP+SSE works identically from the desktop app, VSCode extension, and smoke tests.
- Mock provider in `#[cfg(test)]` paths means CI never needs a running Ollama.

**Negative / mitigations:**
- sqlite-vec deferred → no ANN search in Phase 1; linear scan acceptable at Phase 1 data volume
  (<<10k embeddings). ADR-0003 will close this before scale matters.
- `tokio-rusqlite` adds one abstraction layer over raw rusqlite. If we need fine-grained
  rusqlite connection control later, we can switch to `spawn_blocking` without changing callers.

---

## Alternatives considered

See full tradeoff matrix in `10-system-architecture.md`. Key rejected options:
- Electron — rejected for binary size and "trust-first" feel misalignment.
- LanceDB — rejected for two-store complexity; revisit if vector query quality degrades.
- gRPC/UDS — rejected because VSCode extension has no native gRPC client.

---

## Related

- [10-system-architecture.md](../10-system-architecture.md)
- [45-next-steps.md](../45-next-steps.md)
- [0001-memory-bank-as-canonical-cognition.md](0001-memory-bank-as-canonical-cognition.md)
