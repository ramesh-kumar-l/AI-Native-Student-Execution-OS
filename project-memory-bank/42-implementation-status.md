# 42 · Implementation Status

> What's actually built vs planned. Single source of truth for "does this exist yet?"

**As of:** 2026-05-23 (Phase 2 scaffold)

---

## Built — Phase 0

| Component | Status | Notes |
| --- | --- | --- |
| Project README | ✅ | `README.md` |
| Project CLAUDE.md | ✅ | Operating rules for AI assistants in this repo |
| .gitignore | ✅ | Multi-stack baseline |
| Memory bank | ✅ | Bootstrapped; 44+ files |
| ADR-0001 + ADR-0002 | ✅ | Memory-bank as cognition; Phase 1 stack decisions |

## Built — Phase 1 Daemon Scaffold

| Component | File(s) | Status |
| --- | --- | --- |
| Workspace + Cargo config | `Cargo.toml`, `src-tauri/Cargo.toml` | ✅ |
| Tauri config | `src-tauri/tauri.conf.json`, `capabilities/` | ✅ |
| Error types | `src-tauri/src/error.rs` | ✅ |
| Config | `src-tauri/src/config.rs` | ✅ |
| DB migrations (L0/L1/L2/audit) | `src-tauri/src/db/migrations.rs` | ✅ |
| Memory L0 (event log) | `src-tauri/src/memory/l0.rs` | ✅ |
| Memory L1 (conversations + messages) | `src-tauri/src/memory/l1.rs` | ✅ |
| Memory L2 (vector store) | `src-tauri/src/memory/l2.rs` | ✅ |
| Provider trait + registry | `src-tauri/src/ai/providers/mod.rs` | ✅ |
| OllamaProvider | `src-tauri/src/ai/providers/ollama.rs` | ✅ |
| ClaudeProvider | `src-tauri/src/ai/providers/claude.rs` | ✅ |
| MockProvider (tests) | `src-tauri/src/ai/providers/mock.rs` | ✅ |
| AI Orchestrator | `src-tauri/src/ai/orchestrator.rs` | ✅ |
| TaskSpec contract | `src-tauri/src/ai/task.rs` | ✅ |
| Mentor Engine | `src-tauri/src/engines/mentor.rs` | ✅ |
| HTTP IPC (health + turn + audit) | `src-tauri/src/ipc/` | ✅ |
| Audit log | `src-tauri/src/observability/audit.rs` | ✅ |
| AppState + `start()` | `src-tauri/src/lib.rs` | ✅ |
| Tauri entry point | `src-tauri/src/main.rs` | ✅ |
| Smoke test | `src-tauri/tests/smoke_test.rs` | ✅ |

## Built — Phase 2 Desktop UI MVP

| Component | File(s) | Status |
| --- | --- | --- |
| DB: projects table | `src-tauri/src/db/migrations.rs` | ✅ |
| DB: tasks table | `src-tauri/src/db/migrations.rs` | ✅ |
| L1: Project + Task entity types | `src-tauri/src/memory/l1.rs` | ✅ |
| L1: Project CRUD (create/list/get/update/delete) | `src-tauri/src/memory/l1.rs` | ✅ |
| L1: Task CRUD (create/list/get/update/delete) | `src-tauri/src/memory/l1.rs` | ✅ |
| L1: `list_conversations()` | `src-tauri/src/memory/l1.rs` | ✅ |
| IPC types: project/task request types | `src-tauri/src/ipc/types.rs` | ✅ |
| IPC routes: project CRUD (5 endpoints) | `src-tauri/src/ipc/routes.rs` | ✅ |
| IPC routes: task CRUD (4 endpoints) | `src-tauri/src/ipc/routes.rs` | ✅ |
| IPC routes: conversation + message listing | `src-tauri/src/ipc/routes.rs` | ✅ |
| IPC router: all 10 new routes wired | `src-tauri/src/ipc/mod.rs` | ✅ |
| API client (typed fetch) | `src/api.ts` | ✅ |
| CSS design system | `src/index.css` | ✅ |
| App shell + navigation state | `src/App.tsx` | ✅ |
| StatusBanner (daemon health polling) | `src/components/StatusBanner.tsx` | ✅ |
| Sidebar (project list + nav) | `src/components/Sidebar.tsx` | ✅ |
| Projects page (grid + create/archive/delete) | `src/pages/ProjectsPage.tsx` | ✅ |
| Project detail page (task CRUD + status cycling) | `src/pages/ProjectDetailPage.tsx` | ✅ |
| Mentor chat page (SSE streaming + history) | `src/pages/MentorChatPage.tsx` | ✅ |
| Audit log page (table + expandable metadata) | `src/pages/AuditLogPage.tsx` | ✅ |

## Built — Phase 3 VSCode Capability Intelligence Layer

| Component | File(s) | Status |
| --- | --- | --- |
| DB: workflow_signals table | `src-tauri/src/db/migrations.rs` | ✅ |
| L0: WorkflowSignalReceived event kind | `src-tauri/src/memory/l0.rs` | ✅ |
| L1: WorkflowSignal type + record_signal + recent_signals | `src-tauri/src/memory/l1.rs` | ✅ |
| IPC types: IncomingSignal + SignalQuery | `src-tauri/src/ipc/types.rs` | ✅ |
| IPC route: WebSocket /api/v1/ws | `src-tauri/src/ipc/routes.rs` | ✅ |
| IPC route: GET /api/v1/signals | `src-tauri/src/ipc/routes.rs` | ✅ |
| Mentor engine: workflow context injection | `src-tauri/src/engines/mentor.rs` | ✅ |
| API client: WorkflowSignal + listSignals | `src/api.ts` | ✅ |
| VSCode extension: package.json + tsconfig | `vscode-extension/` | ✅ |
| VSCode extension: types.ts | `vscode-extension/src/types.ts` | ✅ |
| VSCode extension: daemon-client.ts (WS + reconnect) | `vscode-extension/src/daemon-client.ts` | ✅ |
| VSCode extension: signal-capture.ts | `vscode-extension/src/signal-capture.ts` | ✅ |
| VSCode extension: extension.ts (entry point) | `vscode-extension/src/extension.ts` | ✅ |

## Built — Phase 4 Capability + Artifact Engines

| Component | File(s) | Status |
| --- | --- | --- |
| DB: capability_scores table | `src-tauri/src/db/migrations.rs` | ✅ |
| DB: artifacts table | `src-tauri/src/db/migrations.rs` | ✅ |
| L0: CapabilityComputed + ArtifactGenerated events | `src-tauri/src/memory/l0.rs` | ✅ |
| L1: CapabilityScore entity + insert + latest_scores | `src-tauri/src/memory/l1.rs` | ✅ |
| L1: Artifact entity + CRUD | `src-tauri/src/memory/l1.rs` | ✅ |
| Capability Engine (score computation + AI narrative) | `src-tauri/src/engines/capability.rs` | ✅ |
| Artifact Engine (project page + portfolio export) | `src-tauri/src/engines/artifact.rs` | ✅ |
| Engines mod: export new engines | `src-tauri/src/engines/mod.rs` | ✅ |
| AppState: capability + artifact engines | `src-tauri/src/lib.rs` | ✅ |
| IPC types: capability + artifact request types | `src-tauri/src/ipc/types.rs` | ✅ |
| IPC routes: 3 capability + 4 artifact endpoints | `src-tauri/src/ipc/routes.rs` | ✅ |
| IPC router: all 7 new routes wired | `src-tauri/src/ipc/mod.rs` | ✅ |
| API client: CapabilityScore + Artifact types + fns | `src/api.ts` | ✅ |
| Capability page (scores + narrative) | `src/pages/CapabilityPage.tsx` | ✅ |
| Artifacts page (list + generate + preview + download) | `src/pages/ArtifactsPage.tsx` | ✅ |
| App.tsx: capability + artifacts nav states | `src/App.tsx` | ✅ |
| Sidebar.tsx: Capability + Artifacts nav links | `src/components/Sidebar.tsx` | ✅ |
| CSS: capability + artifacts styles | `src/index.css` | ✅ |

## Built — Phase 5 Sync + Trust Polish

| Component | File(s) | Status |
| --- | --- | --- |
| Crypto: AES-256-GCM + Argon2id | `src-tauri/src/crypto/mod.rs` | ✅ |
| Sync coordinator (export/import + encryption) | `src-tauri/src/sync/mod.rs` | ✅ |
| Trust engine (health + lineage) | `src-tauri/src/engines/trust.rs` | ✅ |
| IPC: 5 new endpoints (trust + sync) | `src-tauri/src/ipc/routes.rs` | ✅ |
| Trust page (health dashboard + lineage) | `src/pages/TrustPage.tsx` | ✅ |
| Sync page (export/import + passphrase UI) | `src/pages/SyncPage.tsx` | ✅ |
| Tauri bundling enabled | `src-tauri/tauri.conf.json` | ✅ |
| ADR-0003 (sqlite-vec deferred) | `project-memory-bank/50-adrs/0003-sqlite-vec.md` | ✅ |
| Mobile scaffold | `mobile/README.md` | ✅ |

## Built — TD-02 + TD-04 + TD-05 (Tier 1 hardening)

| Component | File(s) | Status |
| --- | --- | --- |
| Ollama model config-driven (not hardcoded) | `src-tauri/src/ai/orchestrator.rs`, `src-tauri/src/lib.rs` | ✅ |
| Conversation title auto-derived from first message | `src-tauri/src/engines/mentor.rs`, `src-tauri/src/memory/l1.rs` | ✅ |
| L1: `update_conversation_title()` added | `src-tauri/src/memory/l1.rs` | ✅ |
| Input validation on all mutation endpoints | `src-tauri/src/ipc/routes.rs` | ✅ |

## Planned

| Component | Status |
| --- | --- |
| sqlite-vec native extension | ⏳ TD-01 (ADR-0003 deferred — investigate .dll sidecar first) |
| Phase 6 Mobile | ⏳ Capacitor.js + Tauri mobile — scaffold in mobile/README.md |
| Auth / RBAC | ⏳ Post-mobile |
| Cloud sync relay | ⏳ Post-mobile |

---

## Not planned

Mobile app, voice surface, browser extension, custom fine-tuning, multi-user real-time collab.

---

## Related

- [30-roadmap.md](30-roadmap.md)
- [40-current-phase.md](40-current-phase.md)
- [41-active-tasks.md](41-active-tasks.md)
