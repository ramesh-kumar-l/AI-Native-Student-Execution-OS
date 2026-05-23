# 44 · Technical Debt

> Debt log. Each entry: what, why we took it, cost-if-unpaid, remediation.

**As of:** 2026-05-23 — none yet (no code).

---

## Open debt

(none — Phase 0)

## Closed debt

(none yet)

---

## Conventions

- Open debt when a shortcut is taken intentionally. Document immediately, not "later."
- Each entry has an owner (the person/PR that opened it) and a recommended remediation phase.
- Debt that crosses a phase-gate triggers a review: still needed? raise priority? accept permanently?

## Template

```markdown
### TD-NNN · <short title>

- **Opened:** YYYY-MM-DD (PR #NN)
- **What:** brief description
- **Why we took it:** time pressure / scope / dependency unavailable / …
- **Cost-if-unpaid:** what worsens over time if we don't fix this
- **Remediation:** how we'd fix it (and which phase)
- **Owner:** person responsible for tracking
- **Status:** open | scheduled | closed
```

---

## Related

- [20-engineering-standards.md](20-engineering-standards.md)
- [43-risks.md](43-risks.md)
