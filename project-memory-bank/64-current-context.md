# 64 · Current Context

> **This file is loaded first in every new AI session.** It is a compressed orientation — read it, then load only the deeper files you need for your task.

**Last refreshed:** 2026-05-23
**Project owner:** lrameshkumar126
**Repo:** `E:\ClaudeProjects\AI-Native-Student-Execution-OS` (Apache 2.0)

---

## What this project is

**AI-Native Student Execution OS** — a trust-first, offline-first, local-first cognition layer for AI-native builders. Two product surfaces today: Desktop Execution OS (primary) and VSCode Capability Intelligence Layer (secondary). Mobile is a future surface. Long-horizon arc is "global cognition infrastructure for human capability building."

## What phase we're in

**Phase 1 · Daemon Walking Skeleton** — in progress.

- Stack ADR: **ADR-0002 ACCEPTED** (Tauri 2.x + Rust + SQLite + Ollama + Axum HTTP/SSE).
- Application code: **daemon scaffold exists** — see `42-implementation-status.md` for what's built.
- Memory layers L0+L1+L2: **implemented** (in `src-tauri/src/memory/`).
- AI orchestrator: **implemented** with OllamaProvider + ClaudeProvider + MockProvider.
- HTTP IPC: **implemented** — `POST /api/v1/mentor/turn` (SSE streaming), `GET /api/v1/health`, `GET /api/v1/audit/recent`.
- Smoke test: **written** at `src-tauri/tests/smoke_test.rs` — run with `cargo test -p cognition-daemon`.
- Frontend: **minimal React placeholder** — shows daemon health status.
- Phase-gate: **pending exit** — smoke test must pass; engine stubs (Capability, Workflow, Artifact, Trust) pending.

## Operating rules for AI sessions

1. **Memory-first.** Read this file → `40-current-phase.md` → `45-next-steps.md` → task-specific files. Don't scan the repo to "be thorough" — that wastes tokens and is explicitly an anti-pattern in `CLAUDE.md`.
2. **Phase-gated.** End each implementation phase with a summary + tradeoffs + risks; wait for approval. Don't drift forward.
3. **Offline-first.** Every feature must work without network or cloud LLM. Cloud is enhancement only.
4. **Provider-agnostic.** No vendor names in business logic. Use the AI orchestrator (`15-ai-system-design.md`).
5. **No silent failures.** Every degraded mode must be visible to the user.
6. **Surgical changes.** Don't refactor adjacent code, don't reformat, don't drive-by-improve.

## Top-of-mind risks

- **E1** Memory architecture wrong → multi-month rework. Mitigated by investing early in `16-memory-architecture.md` and ADR-0001; Phase 1 implements L0+L1+L2.
- **E2** Provider abstraction leaks vendor specifics. Mitigated by strict `Provider` trait — engines never call providers directly.
- **P1** Mentor feels like a chatbot → no switching cost. Mitigated by memory + workflow embeddedness as headline differentiators.
- **E3** sqlite-vec deferred — L2 uses pure-Rust cosine similarity. Fine for Phase 1 volume; ADR-0003 needed before scale matters.

## Key source locations (Phase 1)

- `src-tauri/src/` — Rust daemon source
- `src-tauri/src/ai/` — provider abstraction + orchestrator
- `src-tauri/src/memory/` — L0/L1/L2 layers
- `src-tauri/src/engines/mentor.rs` — mentor engine
- `src-tauri/src/ipc/` — HTTP routes + types
- `src-tauri/src/observability/audit.rs` — audit log
- `src-tauri/tests/smoke_test.rs` — Phase 1 exit-criterion test

## What to read next

- **Planning a feature?** Start with `30-roadmap.md` + `31-feature-priorities.md`, then `32-user-workflows.md`.
- **Designing implementation?** `10-system-architecture.md` + the relevant subsystem file (memory, AI, offline, security, observability).
- **Touching memory or AI?** `15-ai-system-design.md` + `16-memory-architecture.md` are the load-bearing files.
- **Touching the user's data?** Read `17-security-architecture.md` and `24-security-guidelines.md` before any code.
- **About to make an architectural call?** Read existing ADRs in `50-adrs/` first.

## When this file is stale

This file should be refreshed at every phase-gate. If you're an AI assistant and the contents here disagree with `40-current-phase.md` or `45-next-steps.md`, **update this file in the same change**.

---

## Cross-references

- `README.md` — project README
- `CLAUDE.md` (project root) — operating rules
- `project-memory-bank/README.md` — full memory-bank index
- `project-memory-bank/50-adrs/0002-phase1-stack.md` — ADR-0002 (accepted stack)
- `project-memory-bank/99-autoprompt-source.md` — verbatim canonical directive
