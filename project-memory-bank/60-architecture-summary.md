# 60 · Architecture Summary (one page)

> Designed to be loadable into an AI session in <500 tokens. For depth, see `10-system-architecture.md` and friends.

---

**Product:** AI-Native Student Execution OS — desktop app + VSCode extension + (future) mobile, sharing one local cognition daemon.

**Daemon model:** A single long-running local process per user/device is the source of truth. Surfaces (desktop UI, VSCode) speak to it via HTTP loopback + WebSocket. Cloud is *enhancement-only*.

**Engines (seven):**
- Cognition — owns memory layers L0–L4
- Execution — owns projects, roadmaps, tasks, sessions
- Capability — derives skill signals from workflow + execution
- Mentor — orchestrates AI mentorship turns
- Workflow — captures IDE / desktop signals
- Artifact — generates portfolio + narrative outputs
- Trust — audit, reliability gates, explainability

**AI orchestration:** Engines call `ai.run(TaskSpec)`. Orchestrator picks the provider (local Ollama / cloud) based on policy. Provider abstraction supports Ollama, LM Studio, llama.cpp, Claude, Gemini, OpenAI, DeepSeek, Groq.

**Memory layers:**
- L0 raw events (append-only, source of truth)
- L1 normalized entities (relational)
- L2 semantic index (vector)
- L3 temporal/relational graph
- L4 compressed summaries (for AI context)
Retrieval order for AI context: L4 → L3 → L2 → L1. L0 is for audit only.

**Storage (proposed, pending ADR):** SQLite + sqlite-vec + local blob dir. All swappable through the memory-layer interface.

**Offline contract:** Every core flow works with no network and no cloud LLM. Degraded modes are always visible.

**Sync (later, Phase 5):** Eventual-consistent, single-user multi-device. End-to-end encrypted. Cloud coordinator never sees plaintext.

**Security highlights:** Local DB encrypted at rest. IPC authenticated with per-launch token. Prompt injection defense at context assembly. Tools have typed schemas + audited side effects.

**Observability:** Every AI call, every memory write, every sync action logged with correlation IDs. User-facing audit log is the trust surface. Telemetry is opt-in.

**Stack baseline (proposed, pending ADR-0002):**
- Tauri 2.x (Rust daemon + webview UI)
- React + TypeScript (UI)
- TypeScript (VSCode extension)
- SQLite (libsql) + sqlite-vec
- Ollama (local) + provider abstraction (cloud)
- HTTP loopback + WebSocket IPC
- nomic-embed-text for local embeddings

See `10-system-architecture.md` for the full tradeoff matrix.
