# ADR-0001 · Memory bank as canonical cognition source

- **Status:** Accepted
- **Date:** 2026-05-23
- **Deciders:** Project owner (lrameshkumar126)
- **Context tags:** process, architecture

---

## Context

This project will be developed in close collaboration with AI assistants over a long horizon (5–10 years per `02-long-term-strategy.md`). Without a deliberate cognition architecture for the *project itself*, sessions will:

- waste tokens re-deriving context from raw code/git on each task,
- drift in assumptions across sessions,
- couple decisions to whichever files happened to be open,
- lose architectural intent that isn't expressed in code.

The AUTOPROMPT (`99-autoprompt-source.md`) explicitly mandates a `/project-memory-bank` as the **primary context source**, and prescribes a memory-first protocol.

We need to commit to that approach as a versioned, authoritative structure in this repo — not as a convention people might follow.

---

## Decision

The directory `project-memory-bank/` is the **canonical context source** for all engineering and product decisions in this repository.

Specifically:

1. **Memory-bank files are authoritative.** When they conflict with code, the bank states intent; the code may be wrong.
2. **AI assistants and humans read the bank before non-trivial work.** Read order is defined in `project-memory-bank/README.md` (current context → current phase → next steps → task-specific files).
3. **`64-current-context.md` is loaded first** in any new AI session as a compressed orientation.
4. **Memory-bank updates ship in the same PR as the change that invalidates them.** Drift is treated as a defect.
5. **The bank is structured numerically** (00–04, 10–18, 20–25, 30–34, 40–45, 50–53, 60–64) so the index is stable and links don't rot.
6. **Project root `CLAUDE.md`** distills the operating rules so AI assistants pick them up automatically.

---

## Consequences

**Positive**
- Sessions cold-start with full architectural context in minutes, not hours.
- Token efficiency: targeted file loads instead of repo scans.
- Decisions outlive whichever individual made them.
- New contributors (human or AI) have an obvious entry point.
- Phase-gated execution becomes feasible because "what phase we're in" is a file, not tribal knowledge.

**Negative**
- Real maintenance cost. Drift between bank and code is a real risk; PRs must update the bank.
- Some redundancy with code-level documentation.
- Risk of over-investing in docs that nobody reads (mitigated by `64-current-context.md` being the always-loaded entry).

**Neutral**
- Increases scrutiny of architecture decisions (they have to be written down). For some teams that's a positive; for this project it's the intent.

---

## Alternatives considered

### A. Code-only — let the code be the source of truth
**Rejected because:** code does not express intent, business goals, hypothesis status, or rejected approaches. AI assistants in particular struggle to infer "why" from "what."

### B. README-only — keep all docs in one big file
**Rejected because:** doesn't scale; loading order can't be controlled; hard to maintain semantically.

### C. External docs site (Notion / GitBook)
**Rejected because:** breaks local-first ethos for the project's own documentation; can't be versioned with code; can't be loaded by AI assistants alongside the repo.

### D. ADRs without a broader memory bank
**Rejected because:** ADRs capture decisions but not status, roadmap, current-phase, hypotheses, etc. ADRs are *part* of the bank, not a replacement.

---

## References

- `99-autoprompt-source.md` — the canonical directive mandating this structure
- `project-memory-bank/README.md` — the index and read order
- `project-memory-bank/CLAUDE.md` (project root) — operating rules that derive from this ADR
- `project-memory-bank/64-current-context.md` — the always-loaded entry point
