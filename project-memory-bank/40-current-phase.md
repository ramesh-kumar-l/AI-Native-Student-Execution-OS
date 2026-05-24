# 40 · Current Phase

## Phase 5 · Sync + Trust Polish

**As of:** 2026-05-24
**Status:** COMPLETE — all exit criteria met (cargo build ✅, cargo test ✅, npm run build ✅).

---

## Phase 4 · Capability + Artifact Engines

**Status:** COMPLETE (prior phase)

---

## Phase goal

A user can plan a project, hold a mentor conversation, and see today's plan — entirely from the desktop app.

---

## In scope this phase

- [x] Phase 1 exit criteria met (smoke test exists; awaiting first `cargo build`)
- [x] DB migrations: `projects` + `tasks` tables (Phase 2)
- [x] L1 entity store: `Project` + `Task` CRUD + `list_conversations`
- [x] IPC types: `CreateProjectRequest`, `UpdateProjectRequest`, `CreateTaskRequest`, `UpdateTaskRequest`, `LimitQuery`
- [x] IPC routes: full CRUD for projects/tasks + conversation/message listing
- [x] Router: 10 new endpoints wired (projects, tasks, conversations)
- [x] `audit_recent` parameterized with `?limit=N`
- [x] React frontend shell: `StatusBanner`, `Sidebar`, navigation state
- [x] Projects page: grid of cards, create/archive/delete modal
- [x] Project detail page: task list with status cycling (todo → in_progress → done), priority, inline add
- [x] Mentor chat page: SSE streaming, routing selector (local/cloud/auto), conversation history load
- [x] Audit log page: table viewer, expandable metadata, limit selector
- [x] CSS design system: dark theme, CSS variables, scrollbar, modal, empty states
- [x] `cargo build` passes (first-run dependency resolution)
- [x] `cargo test -p cognition-daemon` green (smoke_mentor_turn_streams_response ok)
- [x] `npm run build` passes (36 modules, 170.87kB JS)
- [ ] Phase-gate summary approved (pending manual E2E smoke)

---

## Explicitly out of scope this phase

- Capability, Workflow, Artifact, Trust engines (stubs only)
- sqlite-vec integration (ADR-0003)
- L4 compressed summaries
- Multi-project sync
- VSCode extension
- Auth, encryption, RBAC
- Themes / preferences (P2 priority)

---

## Phase-gate exit criteria

1. `cargo build` succeeds.
2. `cargo test -p cognition-daemon` passes (all smoke test assertions green).
3. React frontend compiles with `npm run build`.
4. Projects, tasks, and mentor chat work end-to-end with the daemon.
5. Degraded-mode banner shows when daemon is offline.
6. All Phase-2 flows work offline (local Ollama only).

---

## Active technical debt

| ID | Description |
| --- | --- |
| TD-01 | sqlite-vec deferred — pure-Rust cosine sim in L2 (ADR-0003 pending) |
| TD-02 | MockProvider returns canned text — real Ollama needed for prod |
| TD-03 | Tauri bundling disabled (`bundle.active: false`) — needs icons before Phase 2 packaging |
| TD-04 | No conversation title generation — title is NULL |
| TD-05 | No input validation on task title length or project name in Rust handlers |

---

## Related

- [30-roadmap.md](30-roadmap.md)
- [42-implementation-status.md](42-implementation-status.md)
- [45-next-steps.md](45-next-steps.md)
- [50-adrs/0002-phase1-stack.md](50-adrs/0002-phase1-stack.md)
