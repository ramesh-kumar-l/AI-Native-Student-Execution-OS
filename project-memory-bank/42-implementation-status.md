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

## Planned (Phase 4+)

| Component | Status |
| --- | --- |
| Capability Engine | ⏳ Phase 4 |
| Artifact Engine | ⏳ Phase 4 |
| Sync coordinator | ⏳ Phase 5 |
| sqlite-vec native extension | ⏳ ADR-0003 (Phase 1.5 or Phase 3) |

---

## Not planned

Mobile app, voice surface, browser extension, custom fine-tuning, multi-user real-time collab.

---

## Related

- [30-roadmap.md](30-roadmap.md)
- [40-current-phase.md](40-current-phase.md)
- [41-active-tasks.md](41-active-tasks.md)
