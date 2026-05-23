# 64 · Current Context

> **This file is loaded first in every new AI session.** It is a compressed orientation — read it, then load only the deeper files you need for your task.

**Last refreshed:** 2026-05-23
**Project owner:** lrameshkumar126
**Repo:** `E:\ClaudeProjects\AI-Native-Student-Execution-OS` (Apache 2.0)

---

## What this project is

**AI-Native Student Execution OS** — a trust-first, offline-first, local-first cognition layer for AI-native builders. Two product surfaces today: Desktop Execution OS (primary) and VSCode Capability Intelligence Layer (secondary). Mobile is a future surface. Long-horizon arc is "global cognition infrastructure for human capability building."

## What phase we're in

**Phase 0 · Foundation** — memory bank bootstrapped; stack ADR pending.

- Application code: **none yet** (no daemon, no UI, no extension).
- Memory bank: **bootstrapped** (this directory).
- Stack: **awaiting user decision** — see `45-next-steps.md`.

## Operating rules for AI sessions

1. **Memory-first.** Read this file → `40-current-phase.md` → `45-next-steps.md` → task-specific files. Don't scan the repo to "be thorough" — that wastes tokens and is explicitly an anti-pattern in `CLAUDE.md`.
2. **Phase-gated.** End each implementation phase with a summary + tradeoffs + risks; wait for approval. Don't drift forward.
3. **Offline-first.** Every feature must work without network or cloud LLM. Cloud is enhancement only.
4. **Provider-agnostic.** No vendor names in business logic. Use the AI orchestrator (`15-ai-system-design.md`).
5. **No silent failures.** Every degraded mode must be visible to the user.
6. **Surgical changes.** Don't refactor adjacent code, don't reformat, don't drive-by-improve.

## Top-of-mind risks

- **E1** Memory architecture wrong → multi-month rework. Mitigated by investing early in `16-memory-architecture.md` and ADR-0001.
- **E2** Provider abstraction leaks vendor specifics. Mitigated by strict TaskSpec contract.
- **P1** Mentor feels like a chatbot → no switching cost. Mitigated by memory + workflow embeddedness as headline differentiators.

(Full register: `43-risks.md`.)

## Open decisions blocking Phase 1

Seven stack-level decisions in `45-next-steps.md` (desktop framework, local DB, vector store, local LLM runtime, IPC mechanism, frontend framework, embedding model). Recommendations are documented but not committed; ADR-0002 will formalize once the user decides.

## What to read next

- **Planning a feature?** Start with `30-roadmap.md` + `31-feature-priorities.md`, then `32-user-workflows.md`.
- **Designing implementation?** `10-system-architecture.md` + the relevant subsystem file (memory, AI, offline, security, observability).
- **Touching memory or AI?** `15-ai-system-design.md` + `16-memory-architecture.md` are the load-bearing files.
- **Touching the user's data?** Read `17-security-architecture.md` and `24-security-guidelines.md` before any code.
- **About to make an architectural call?** Read existing ADRs in `50-adrs/` first.

## When this file is stale

This file should be refreshed at every phase-gate, and whenever stack decisions land. If you're an AI assistant and the contents here disagree with `40-current-phase.md` or `45-next-steps.md`, **update this file in your PR**.

---

## Cross-references

- `README.md` — project README
- `CLAUDE.md` (project root) — operating rules
- `project-memory-bank/README.md` — full memory-bank index
- `project-memory-bank/99-autoprompt-source.md` — verbatim canonical directive
