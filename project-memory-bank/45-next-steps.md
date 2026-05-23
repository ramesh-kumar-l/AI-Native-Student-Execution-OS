# 45 · Next Steps

> Decisions pending and work queued. The user's reference for "what unblocks Phase 1?"

**As of:** 2026-05-23

---

## Pending user decisions (blocks Phase 1)

The stack-options matrix in `10-system-architecture.md` lays out the choices. **Recommendation per row is given; user decides.**

| # | Decision | Recommendation | Alternatives | Stakes |
| --- | --- | --- | --- | --- |
| D-1 | Desktop framework | **Tauri 2.x (Rust core + webview frontend)** | Electron; native | Hardest to change later; affects daemon language |
| D-2 | Local DB | **SQLite via libsql** | DuckDB; pglite | Storage migration is painful but doable |
| D-3 | Vector store | **sqlite-vec** | LanceDB; Qdrant | Swappable through a memory-layer interface |
| D-4 | Local LLM runtime | **Ollama** | LM Studio; llama.cpp | Swappable through provider abstraction |
| D-5 | IPC mechanism | **HTTP loopback + WebSocket** | gRPC/UDS; Tauri-only commands | Affects VSCode extension architecture |
| D-6 | Frontend framework | **React** (with TypeScript) | Svelte; SolidJS | UI-team-velocity tradeoff |
| D-7 | Embedding model (local) | **nomic-embed-text** | bge-m3; all-MiniLM | Affects re-embed migration if changed |

Once you decide, we author **one or more ADRs** in `project-memory-bank/50-adrs/` formalizing each.

---

## Other open architectural questions (can defer to Phase 1)

- **ID strategy** — ULID vs UUIDv7.
- **Sync protocol** — CRDT vs operational; library choice.
- **Key management** — passphrase-derived vs OS-keychain vs hybrid.
- **Reranker** — local model vs none for v0.
- **Capability skill taxonomy** — free-form vs controlled vocabulary.

These do not block Phase 1; we'll surface ADRs for them as we approach the relevant work.

---

## Recommended sequence after Phase-0 phase-gate approval

1. **Author ADRs** for D-1 … D-5 (D-6, D-7 can follow once the daemon scaffold exists).
2. **Update memory-bank** references from "proposed" / "pending" to ADR-linked.
3. **Define Phase 1 "Hello, daemon" DoD** in `30-roadmap.md` and `40-current-phase.md`.
4. **Scaffold the daemon** (per ADRs): repo layout, CI, smoke test that starts the daemon and exits 0.
5. **Implement minimal L0 + L1** with one entity (e.g., Project CRUD).
6. **Wire the AI orchestrator** with the Ollama provider; one mentor turn end-to-end.

Each step ends in a smaller phase-gate (we don't drift forward).

---

## Open product / strategy items (lower priority but tracked)

- Wedge messaging draft (`03-market-positioning.md` has the positioning statement to refine).
- Telemetry consent UX wireframe (Phase 2).
- Self-host story for sync coordinator (Phase 5).

---

## Related

- [10-system-architecture.md](10-system-architecture.md)
- [40-current-phase.md](40-current-phase.md)
- [41-active-tasks.md](41-active-tasks.md)
- [50-adrs/](50-adrs/)
