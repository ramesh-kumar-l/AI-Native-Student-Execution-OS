# 30 · Roadmap

> Phased build. Each phase ends in a phase-gate (summary + tradeoffs + decision request). No phase is auto-advanced.

---

## Phase 0 · Foundation *(current)*

**Goal:** Establish canonical context, engineering standards, and stack-decision readiness so subsequent phases execute fast.

**In scope**
- Memory bank bootstrap *(done as of this PR)*
- Project CLAUDE.md, README, .gitignore *(done)*
- Stack-options matrix in `10-system-architecture.md` *(done)*
- First ADR scaffolding in `50-adrs/` *(done)*
- Risk register, technical-debt log, next-steps log *(done)*

**Phase-gate exit criteria**
- User has decided on the stack ADR (desktop framework, DB, vector, local LLM, IPC).
- All architecture files reference real, decided tech.
- A "Hello, daemon" milestone is defined for Phase 1.

---

## Phase 1 · Daemon walking skeleton

**Goal:** A runnable local daemon that exposes IPC, persists to the local DB, embeds + retrieves text, and routes a single AI call through the provider abstraction.

**In scope**
- Local daemon process (per stack ADR).
- SQLite + sqlite-vec storage.
- Memory layer L0 + L1 + L2 (basic).
- AI orchestrator with Ollama provider (local) + one cloud provider through abstraction.
- HTTP loopback + WebSocket IPC.
- Audit log entity + minimal viewer (CLI is fine).
- Smoke test: a script that sends a "mentor turn" command and gets back a streamed response from a local model.

**Out of scope** for Phase 1: UI, VSCode extension, sync, capability engine, artifact engine.

**Phase-gate exit criteria**
- Daemon starts, accepts IPC, persists data, talks to local + one cloud provider.
- Audit log records every AI call.
- Integration test covers offline mentor turn end-to-end.

---

## Phase 2 · Desktop UI MVP

**Goal:** A user can plan a project, hold a mentor conversation, and see today's plan — entirely from the desktop app.

**In scope**
- Desktop UI shell (per stack ADR).
- Project / roadmap / task CRUD.
- Mentor chat surface with streaming.
- Memory recall surfaces (mentor "see context").
- Degraded-mode banners (offline, local-model-down, etc.).
- Audit log viewer in UI.

**Phase-gate exit criteria**
- One user (you) uses the app daily for a week without leaving it for a competitor tool to do the same job.
- All Phase-2 flows work offline.

---

## Phase 3 · VSCode Capability Intelligence Layer

**Goal:** Workflow signal capture from VSCode flows into the memory layer; mentor turns benefit from this context.

**In scope**
- VSCode extension scaffold.
- Workflow signal types + capture.
- WebSocket connection to daemon.
- Inline architecture hints (lightweight).
- Capability signal extraction (basic).

**Phase-gate exit criteria**
- Daemon receives signals at expected rates from VSCode usage.
- A mentor turn demonstrably uses workflow context.
- Extension has no perceivable activation overhead.

---

## Phase 4 · Capability + Artifact engines

**Goal:** Capability scores over time become user-visible; artifact generation produces shareable outputs.

**In scope**
- Capability Engine: signal aggregation → scores + narratives.
- Artifact Engine: project page, portfolio entry, narrative export.
- L3 graph + L4 summaries (richer compression).
- "What I'm getting better at" surface in UI.

**Phase-gate exit criteria**
- Capability narratives feel accurate (qualitative test).
- Generated artifacts are share-quality.

---

## Phase 5 · Sync + Trust polish

**Goal:** Cross-device sync ships; Trust Engine surfaces full audit + reliability story.

**In scope**
- Encrypted sync via cloud coordinator.
- Cross-device pairing.
- Trust Engine UI: health, audit, what-was-sent-where.
- Export / import flows.

---

## Phase 6+ · Mobile, voice, institutional

Deferred — see `02-long-term-strategy.md`.

---

## Phase health rules

- Each phase has an explicit exit criterion. We do not "drift forward."
- Tech debt opened during a phase is logged in `44-technical-debt.md`, not silently carried.
- A phase that overruns by >50% triggers a re-scope conversation, not a "push harder."

---

## Related

- [02-long-term-strategy.md](02-long-term-strategy.md)
- [31-feature-priorities.md](31-feature-priorities.md)
- [40-current-phase.md](40-current-phase.md)
- [45-next-steps.md](45-next-steps.md)
