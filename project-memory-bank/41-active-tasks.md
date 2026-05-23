# 41 · Active Tasks

> The actively-being-worked items. When a task closes, it moves to `42-implementation-status.md` (or to git history once well-documented elsewhere).

**As of:** 2026-05-23

---

## In flight

- (none — Phase 0 bootstrap completed in this PR; awaiting stack ADR decision)

## Awaiting user decision (blocks Phase 1)

- Stack ADR: desktop framework (Tauri recommended), local DB (SQLite/libsql), vector (sqlite-vec), local LLM runtime (Ollama), IPC mechanism (HTTP loopback + WebSocket).
  - See proposals in `10-system-architecture.md`.
  - See open decision tracker in `45-next-steps.md`.

## Next-up (post-decision)

- Author the stack ADR(s) in `50-adrs/`.
- Update memory-bank files referring to "proposed" / "pending" to "decided" + ADR link.
- Define Phase 1 "Hello, daemon" milestone DoD.

---

## Conventions

- Tasks here should be small enough to close within a week. Anything bigger belongs in `30-roadmap.md` as a phase or in `31-feature-priorities.md` as a feature.
- Each task lists: what, why, owner (if multi-contributor), DoD.

---

## Related

- [40-current-phase.md](40-current-phase.md)
- [42-implementation-status.md](42-implementation-status.md)
- [45-next-steps.md](45-next-steps.md)
