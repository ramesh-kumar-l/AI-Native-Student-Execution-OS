# 40 · Current Phase

## Phase 0 · Foundation

**As of:** 2026-05-23
**Status:** in progress — memory-bank bootstrapped; stack ADR pending user decision.

---

## Phase goal

Establish canonical context, engineering standards, and stack-decision readiness so subsequent phases execute fast and consistently.

---

## In scope this phase

- [x] Project root scaffolding: `CLAUDE.md`, `README.md`, `.gitignore`
- [x] `project-memory-bank/` bootstrap with index
- [x] Strategic files (00–04)
- [x] Architecture files (10–18): load-bearing files real; scaffolds for the rest
- [x] Engineering files (20–25)
- [x] Product files (30–34)
- [x] Execution files (40–45)
- [x] Decisions scaffolding (50–53) with first ADR placeholder
- [x] Compressed-context files (60–64) including `64-current-context.md`
- [x] AUTOPROMPT source preserved as `99-autoprompt-source.md`
- [ ] **Stack ADR merged** — user decision pending
- [ ] Phase-gate summary delivered and approved

---

## Explicitly out of scope (do NOT do this phase)

- Any application code (daemon, desktop UI, VSCode extension)
- Any sync work
- Any AI orchestrator implementation
- Database schema implementation
- CI pipelines beyond the minimum

These all wait for Phase 1, which requires the stack ADR.

---

## Phase-gate exit criteria

This phase exits when:

1. The user has reviewed and approved the bootstrapped memory bank.
2. A stack ADR (or short series of ADRs) has been merged covering: desktop framework, local DB, vector store, local LLM runtime, IPC mechanism.
3. `45-next-steps.md` is updated with a clear Phase-1 plan.
4. A "Hello, daemon" definition-of-done is documented for Phase 1.

---

## Risks active this phase

See `43-risks.md`. Key Phase-0 risks:

- Over-spec'ing the architecture before any code reveals what's wrong with it.
- Stack analysis paralysis — multiple equally-defensible options can stall the decision.
- Memory-bank content drifting from real implementation in later phases.

---

## Related

- [30-roadmap.md](30-roadmap.md)
- [41-active-tasks.md](41-active-tasks.md)
- [45-next-steps.md](45-next-steps.md)
- [43-risks.md](43-risks.md)
