# 64 · Current Context

> **This file is loaded first in every new AI session.** It is a compressed orientation — read it, then load only the deeper files you need for your task.

**Last refreshed:** 2026-05-24
**Project owner:** lrameshkumar126
**Repo:** `E:\ClaudeProjects\AI-Native-Student-Execution-OS` (Apache 2.0)

---

## What this project is

**AI-Native Student Execution OS** — a trust-first, offline-first, local-first cognition layer for AI-native builders. Two product surfaces today: Desktop Execution OS (primary) and VSCode Capability Intelligence Layer (secondary). Mobile is a future surface. Long-horizon arc is "global cognition infrastructure for human capability building."

## What phase we're in

**Phase 4 · Capability + Artifact Engines** — build gate PASSED; pending manual E2E smoke.

- Stack ADR: **ADR-0002 ACCEPTED** (Tauri 2.x + Rust + SQLite + Ollama + Axum HTTP/SSE).
- Phases 1–4 fully compiled: `cargo build` ✅ `cargo test` ✅ `npm run build` ✅
- DB: **Phase 4 tables** — `capability_scores` + `artifacts` on top of P1–P3 schema.
- API: **22 endpoints** — added 3 capability + 4 artifact endpoints (Phase 4).
- Engines: **MentorEngine + CapabilityEngine + ArtifactEngine** all wired into AppState.
- Frontend: **8 pages** — Projects, ProjectDetail, MentorChat, AuditLog, Signals (via Sidebar), Capability, Artifacts.
- Phase-gate: **build criteria met**; manual E2E smoke (Recompute Scores, Generate Project Page) still needed.

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
- **E3** sqlite-vec deferred — L2 uses pure-Rust cosine similarity. Fine for Phase 2 volume; ADR-0003 needed before scale matters.

## Key source locations (Phase 2)

**Daemon (Rust):**
- `src-tauri/src/` — Rust daemon source
- `src-tauri/src/ai/` — provider abstraction + orchestrator
- `src-tauri/src/memory/` — L0/L1/L2 layers (L1 includes Project + Task CRUD)
- `src-tauri/src/engines/mentor.rs` — mentor engine
- `src-tauri/src/ipc/` — HTTP routes + types (15 endpoints)
- `src-tauri/src/observability/audit.rs` — audit log
- `src-tauri/tests/smoke_test.rs` — Phase 1 exit-criterion test

**Frontend (React/TypeScript):**
- `src/api.ts` — typed API client
- `src/App.tsx` — app shell + `NavState` type
- `src/components/StatusBanner.tsx` — daemon health polling
- `src/components/Sidebar.tsx` — navigation + project list
- `src/pages/ProjectsPage.tsx` — project grid + create modal
- `src/pages/ProjectDetailPage.tsx` — task CRUD + status cycling
- `src/pages/MentorChatPage.tsx` — SSE streaming chat
- `src/pages/AuditLogPage.tsx` — audit log table
- `src/index.css` — dark-theme CSS design system

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
