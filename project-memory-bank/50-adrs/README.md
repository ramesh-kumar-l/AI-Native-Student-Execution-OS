# 50 · Architecture Decision Records

> ADRs document **why** we chose what we chose. Future engineers (human and AI) should be able to read an ADR and either understand the reasoning or know why to revisit it.

---

## Format

Use Michael Nygard's ADR template (slightly extended):

```markdown
# ADR-NNNN · <Decision title>

- **Status:** Proposed | Accepted | Superseded by ADR-MMMM | Deprecated
- **Date:** YYYY-MM-DD
- **Deciders:** <names>
- **Context tags:** <stack|architecture|product|process>

## Context
What forces are at play? What is the problem we're solving?

## Decision
What we are going to do.

## Consequences
Positive, negative, neutral. What becomes easier? Harder? What new risks?

## Alternatives considered
What else we looked at and why we rejected it.

## References
Memory-bank files, prior ADRs, external sources.
```

---

## Numbering

- Four-digit numbers, zero-padded (`0001`).
- Never reuse a number; superseded ADRs stay in place with a status update.

## Lifecycle

1. **Proposed** — opened as a PR; status = Proposed; under review.
2. **Accepted** — merged with status = Accepted.
3. **Superseded** — when a new ADR replaces it: old ADR status = "Superseded by ADR-NNNN"; new ADR links back.
4. **Deprecated** — decision no longer relevant (technology removed); kept for history.

## Where ADRs live

- File path: `project-memory-bank/50-adrs/NNNN-kebab-title.md`
- Linked from the relevant architecture / engineering memory-bank file.
- Major ADRs also referenced in `60-architecture-summary.md`.

---

## Open ADRs (proposed but not yet merged)

| # | Title | Status |
| --- | --- | --- |
| 0001 | Memory bank as canonical cognition source | Accepted |
| 0002 | Stack baseline: desktop framework / DB / vector / LLM / IPC | **Pending user decision** — see `45-next-steps.md` |

---

## Index

- [0001-memory-bank-as-canonical-cognition.md](0001-memory-bank-as-canonical-cognition.md)
- *(0002 will be authored after user decision)*
