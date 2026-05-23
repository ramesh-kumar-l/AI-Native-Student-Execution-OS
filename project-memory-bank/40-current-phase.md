# 40 · Current Phase

## Phase 1 · Daemon Walking Skeleton

**As of:** 2026-05-23
**Status:** In progress — scaffold committed; smoke test written; exit criteria partially met.

---

## Phase goal

Produce a runnable local daemon with:
- SQLite storage (L0 event log, L1 entities, L2 vector store)
- AI orchestrator routing to Ollama (local) and Claude (cloud) through a provider abstraction
- HTTP loopback + SSE streaming IPC
- Audit log
- Smoke test that proves the full mentor-turn path works end-to-end

---

## In scope this phase

- [x] ADR-0002 accepted — stack formalized (Tauri 2.x, SQLite, Ollama, Axum)
- [x] Workspace `Cargo.toml` + `src-tauri/Cargo.toml`
- [x] Database layer: `db/mod.rs` + `db/migrations.rs` (L0/L1/L2/audit tables)
- [x] Memory Layer L0: append-only event log (`memory/l0.rs`)
- [x] Memory Layer L1: normalized entities — Conversation + Message (`memory/l1.rs`)
- [x] Memory Layer L2: vector store with pure-Rust cosine similarity (`memory/l2.rs`)
- [x] Provider trait + ProviderRegistry (`ai/providers/mod.rs`)
- [x] OllamaProvider: streaming NDJSON (`ai/providers/ollama.rs`)
- [x] ClaudeProvider: streaming SSE (`ai/providers/claude.rs`)
- [x] MockProvider: deterministic canned response for CI (`ai/providers/mock.rs`)
- [x] AI Orchestrator: routes TaskSpec, emits audit events (`ai/orchestrator.rs`)
- [x] Mentor Engine: full turn pipeline with L0/L1 persistence (`engines/mentor.rs`)
- [x] HTTP IPC server: health, mentor/turn (SSE), audit/recent (`ipc/routes.rs`)
- [x] Audit log (`observability/audit.rs`)
- [x] AppState wiring + `start()` function for testability (`lib.rs`)
- [x] Tauri entry point (`main.rs`)
- [x] Minimal React frontend — daemon health display
- [x] Smoke test (`tests/smoke_test.rs`)
- [ ] `cargo test` passes (requires Cargo.lock + dependency resolution)
- [ ] Phase-gate summary delivered and approved

---

## Explicitly out of scope this phase

- Capability, Workflow, Artifact, Trust engines (stubs only)
- Multi-project support
- L3 temporal/relational graph
- L4 compressed summaries
- Sync / multi-device
- VSCode extension
- sqlite-vec native extension (deferred to ADR-0003)
- Auth, encryption, RBAC
- Telemetry / metrics pipeline

---

## Phase-gate exit criteria

1. `cargo test -p cognition-daemon` passes (all smoke test assertions green).
2. `GET /api/v1/health` returns `{"status":"ok","daemon_ready":true}`.
3. `POST /api/v1/mentor/turn` with a MockProvider streams back ≥1 chunk with `done=true`.
4. Audit log has entries for `mentor_turn_received`, `ai_call_started`, `ai_call_completed`, `mentor_turn_completed`.
5. Memory bank updated to reflect Phase 1 state.

---

## Risks active this phase

- **E3** sqlite-vec deferred — acceptable; L2 interface unchanged for future swap.
- **Dependency resolution** — `tokio-rusqlite`, `axum 0.7`, `tauri 2.x` may have version conflicts on first `cargo build`. Expect one iteration to resolve.
- **Tauri bundling** — we set `bundle.active: false` to avoid icon requirements; change before Phase 2 packaging.

---

## Related

- [30-roadmap.md](30-roadmap.md)
- [41-active-tasks.md](41-active-tasks.md)
- [42-implementation-status.md](42-implementation-status.md)
- [45-next-steps.md](45-next-steps.md)
- [50-adrs/0002-phase1-stack.md](50-adrs/0002-phase1-stack.md)
