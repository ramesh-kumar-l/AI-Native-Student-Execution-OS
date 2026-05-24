# 40 · Current Phase

## Phase 6 Foundation · TD-01 Architecture + Mobile

**As of:** 2026-05-24
**Status:** COMPLETE — all exit criteria met (cargo build ✅, cargo test ✅, npm run build ✅).

### TD-01 (Vector search architecture) — done
- Dual-path L2Store: sqlite-vec KNN fast-path (DLL-gated) + cosine-scan fallback
- `vec_embedding_map` table added to regular migrations (no extension required)
- `run_vec()` creates `vec_embeddings` virtual table when extension is loaded at runtime
- Windows MSVC linker blocks static sqlite-vec compilation (documented ADR-0004)
- Fast-path slot ready for DLL sidecar OR pure-Rust HNSW (`instant-distance`)

### Phase 6 Mobile CSS + scaffold — done
- `BottomNav.tsx`: 5-item primary nav (Projects, Mentor, Skills, Artifacts, Sync)
- `useIsMobile()` hook in App.tsx: sidebar at >768 px, bottom nav at ≤768 px
- `index.css`: mobile media query, safe-area padding, single-column grid, touch targets
- `capacitor.config.ts`: Capacitor 6.x config scaffold for future native API bridge
- ADR-0005: Mobile build pipeline documented (Tauri Mobile + Capacitor dual-layer)

---

## Phase 5 · Sync + Trust Polish + Tier 1 Hardening

**As of:** 2026-05-24
**Status:** COMPLETE — all exit criteria met (cargo build ✅, cargo test ✅, npm run build ✅).

### Tier 1 hardening (TD-02, TD-04, TD-05) — completed same session

- **TD-02 (Real Ollama):** `OllamaProvider` was already wired for non-test mode. Fixed: `Orchestrator` now reads `config.ai.default_local_model` (env `DEFAULT_LOCAL_MODEL`, default `llama3.2`) instead of hardcoding the model name.
- **TD-04 (Conversation titles):** `create_conversation` now derives a title from the first 60 chars of the user message (truncated at word boundary). `L1Store::create_conversation` accepts `Option<String>` title. New `update_conversation_title` method added to L1Store for future AI-title generation.
- **TD-05 (Input validation):** All mutation IPC handlers validate at system boundary: project name (1–200 chars), task title (1–200 chars), task priority (1–5), mentor message (1–32 000 chars). Returns `400 BAD_REQUEST` with an `ApiError` body on violation.

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
