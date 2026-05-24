# AISEOS Quick Starter Guide

Welcome to **AI-Native Student Execution OS**. This guide is designed for a new engineer downloading the repo for the first time. It explains the product, the architecture, the code structure, the setup steps, the current phase of development, and the best way to contribute.

It also includes four complete blog-post drafts you can publish to Medium or other platforms to highlight your technical knowledge and increase visibility for the repository.

---

## 1. What is this project?

**AI-Native Student Execution OS** is an offline-first, local-first desktop cognition layer for builders who want a trusted environment for project execution, mentorship, and AI-assisted workflow.

Key user-facing surfaces today:
- **Desktop Execution OS** — the primary product surface, built with Tauri + React + Rust.
- **VS Code Capability Intelligence** — a planned secondary surface.
- **Ambient Mobile Cognition** — future direction.

The long-term vision is to become **global cognition infrastructure for human capability building**.

### Product thesis

The project is built around the idea that AI should help people execute on real work, not just chat. It aims to combine:
- structured project and task memory
- local AI provider support
- auditability and observability
- mentor-like assistance
- reliability when network or cloud is unavailable

---

## 2. Current status

This repo is in **Phase 2 · Desktop UI MVP**. That means:
- A Rust daemon exists and implements the core backend.
- A React frontend exists with project, task, mentor chat, and audit screens.
- Memory layers are implemented in the daemon.
- AI orchestration is implemented with a provider abstraction.
- Full production polish is still in progress.

Important note: this is a working prototype / MVP. The architecture is intentionally opinionated about offline-first, provider-agnostic design.

---

## 3. What a new engineer should know first

### Why this repo is structured the way it is

The code is separated into two main domains:

1. **Frontend UI** (`src/`)
   - React pages, components, CSS, and API client.
   - Designed to run inside a Tauri desktop shell.

2. **Daemon backend** (`src-tauri/src/`)
   - Rust daemon, memory stores, AI provider abstraction, HTTP IPC server.
   - Uses Axum for HTTP and SSE to support streaming mentor chat.

There is also a **project memory bank** at `project-memory-bank/`, which is the authoritative source of strategy, architecture, design, and execution decisions.

### What is the source of truth for this project?

`project-memory-bank/64-current-context.md` is the best single-file orientation for new contributors. It summarizes the current phase, stack choices, subsystem boundaries, and key source locations.

For design and implementation insight, read these first:
- `project-memory-bank/15-ai-system-design.md`
- `project-memory-bank/16-memory-architecture.md`
- `project-memory-bank/10-system-architecture.md`
- `project-memory-bank/45-next-steps.md`
- `project-memory-bank/50-adrs/0002-phase1-stack.md`

---

## 4. What’s inside the repo

### Top-level files

- `README.md` — project landing page and high-level introduction.
- `CLAUDE.md` — AI assistant operating rules for this repo.
- `LICENSE` — Apache 2.0.
- `package.json` — frontend scripts and dependencies.
- `Cargo.toml` — workspace definition.
- `src-tauri/Cargo.toml` — Rust daemon dependencies.

### Frontend

- `src/main.tsx` — app bootstrap.
- `src/App.tsx` — navigation shell and page rendering.
- `src/api.ts` — typed API client for backend calls.
- `src/pages/` — page components:
  - `ProjectsPage.tsx`
  - `ProjectDetailPage.tsx`
  - `MentorChatPage.tsx`
  - `AuditLogPage.tsx`
- `src/components/` — UI components like `Sidebar.tsx` and `StatusBanner.tsx`.
- `src/index.css` — design tokens and styling.

### Rust backend / daemon

- `src-tauri/src/lib.rs` — daemon initialization, provider registry, run loop.
- `src-tauri/src/main.rs` — Tauri entrypoint.
- `src-tauri/src/config.rs` — runtime configuration and environment overrides.
- `src-tauri/src/ipc/` — API routes and request/response types.
- `src-tauri/src/ai/` — AI orchestration and provider abstractions.
- `src-tauri/src/memory/` — memory layers L0/L1/L2 and unified facade.
- `src-tauri/src/engines/mentor.rs` — mentor engine logic.
- `src-tauri/src/observability/` — audit logging.
- `src-tauri/tests/` — smoke tests and daemon validation.

### Docs

- `project-memory-bank/` — design docs, phase plan, architecture, ADRs, execution status.

---

## 5. Quickstart setup

### Prerequisites

- **Rust toolchain** installed. Use `rustup` and ensure `cargo` is available.
- **Node.js and npm** installed.
- **Tauri prerequisites** for Windows: Visual Studio Build Tools and the Windows SDK. See Tauri docs if you need them.
- Optionally: **Ollama** or a local provider if you want a real local AI backend.

### Install dependencies

From the repo root:

```bash
npm install
```

The frontend uses Vite, React, and TypeScript.

### Build the frontend

```bash
npm run build
```

This runs `tsc` and `vite build`.

### Build the Rust daemon

From repo root:

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

### Run tests

From repo root:

```bash
cargo test -p cognition-daemon
```

If you want to run the backend smoke test only, use the test target in `src-tauri/tests/smoke_test.rs`.

### Start the desktop app in development

```bash
npm run tauri dev
```

This launches the Tauri desktop shell and starts the Rust daemon via the app setup.

### Run the backend daemon directly

If you need to run the daemon separately for debugging:

```bash
cargo run --manifest-path src-tauri/Cargo.toml
```

### Use mock mode for local development

The daemon supports a mock AI provider mode for tests and local debugging.

```bash
set DAEMON_TEST_MODE=1
cargo run --manifest-path src-tauri/Cargo.toml
```

On PowerShell use `setx DAEMON_TEST_MODE 1` or set the variable in the session directly.

---

## 6. Configuration and environment variables

The daemon loads runtime configuration from the environment by default.

Supported environment variables:

- `DAEMON_TEST_MODE` — enables mock provider mode.
- `DAEMON_PORT` — sets the local HTTP IPC port; default is `45678`.
- `OLLAMA_BASE_URL` — local Ollama provider endpoint; default is `http://localhost:11434`.
- `ANTHROPIC_API_KEY` — Claude cloud provider key if you want cloud routing.
- `DEFAULT_LOCAL_MODEL` — default local model name; default is `llama3.2`.

The daemon writes data to a local SQLite database under the OS app data directory by default.

---

## 7. How the system works

### Frontend + Tauri + backend

The desktop app is a Tauri application. The React UI runs in a webview, and the Rust daemon runs inside the same desktop process.

The frontend communicates with the daemon over HTTP loopback APIs mounted by Axum. Main flows include:
- project CRUD
- task CRUD
- mentor chat stream via SSE
- recent audit events

### AI orchestration

The backend uses a layered AI architecture:
- `Engine` code (e.g. mentor) creates a `TaskSpec`
- `Orchestrator` resolves the right provider
- `Provider` abstraction performs `chat_stream(...)`
- responses are streamed back as SSE

The orchestrator is intentionally provider-agnostic. Engines do not select models directly.

### Local-first routing

Routing policy is configured to prefer local providers by default.
The provider registry tries local providers first, and only uses cloud if configured and allowed.

Supported provider patterns in code today:
- `OllamaProvider` for local model inference
- `ClaudeProvider` for cloud inference when `ANTHROPIC_API_KEY` exists
- `MockProvider` for tests and development

### Memory layers

The daemon has a multi-layer memory model:
- `L0` — raw event log
- `L1` — normalized relational entities like projects and tasks
- `L2` — semantic embeddings / vector index

This repo currently implements the foundational memory layers for project and task CRUD, mentor conversations, and auditability.

### Audit and observability

Every AI call is logged with a correlation ID, provider, model metadata, and completion events. The frontend can show recent audit entries.

The project is intentionally built so failures are surfaced, not hidden.

---

## 8. Developer orientation: first places to look

### If you want to understand the UI

Start with:
- `src/App.tsx` — app navigation and rendering.
- `src/pages/ProjectsPage.tsx` — project list.
- `src/pages/ProjectDetailPage.tsx` — task management.
- `src/pages/MentorChatPage.tsx` — streaming mentor chat.
- `src/components/StatusBanner.tsx` — status and daemon readiness.
- `src/api.ts` — the typed API layer.

### If you want to understand the backend

Start with:
- `src-tauri/src/lib.rs` — application state and provider setup.
- `src-tauri/src/config.rs` — runtime config and environment mapping.
- `src-tauri/src/ipc/routes.rs` — REST routes and SSE chat.
- `src-tauri/src/ipc/types.rs` — request and response DTOs.
- `src-tauri/src/ai/orchestrator.rs` — provider routing and audit integration.
- `src-tauri/src/memory/mod.rs` — memory facade.
- `src-tauri/src/engines/mentor.rs` — actual mentor turn flow.

### If you want to understand the product design

Read:
- `project-memory-bank/64-current-context.md`
- `project-memory-bank/15-ai-system-design.md`
- `project-memory-bank/16-memory-architecture.md`
- `project-memory-bank/45-next-steps.md`
- `project-memory-bank/50-adrs/0002-phase1-stack.md`

These documents explain the architecture, what has been decided, and what is still open.

---

## 9. How to contribute safely

### Read the memory bank first

This repo uses a memory-bank-first workflow. Before making changes, read the relevant docs under `project-memory-bank/`.

### Follow phase-gated rules

The project is working in explicit phases. Each change should:
1. define the goal clearly,
2. implement minimal surgical changes,
3. validate with build/tests,
4. summarize and wait before moving to the next phase.

### Keep changes small and obvious

Avoid refactoring unrelated code. If you add a feature, touch only the files needed to implement it.

### Respect offline-first design

This project prioritizes local functionality. Do not make the app depend on cloud AI or network access for core flows.

### Use auditing and observability

If you add new AI or workflow behavior, add an audit event or log entry. The product requires no silent failures.

---

## 10. Common newbie questions

### What is the expected developer workflow?

1. Read `project-memory-bank/64-current-context.md`.
2. Install dependencies: `npm install`, `cargo build --manifest-path src-tauri/Cargo.toml`.
3. Run frontend and backend with `npm run tauri dev`.
4. Validate behavior by creating a project, tasks, and opening mentor chat.
5. Fix or add features by editing the relevant frontend or backend files.
6. Run `cargo test -p cognition-daemon`.

### What does the app do today?

It supports:
- creating and listing projects
- creating and managing tasks inside a project
- opening a mentor chat for a project
- showing a recent audit log
- streaming AI mentor responses over SSE

### Is this production-ready?

Not yet. It is a Phase 2 MVP with a strong architecture and a codebase that already includes the core local-first design.

### What is the most important file for architecture?

`project-memory-bank/64-current-context.md` is the best single orientation file.

### How is AI handled?

AI calls route through an orchestrator in `src-tauri/src/ai/orchestrator.rs`. The AI provider interface is abstracted so the app can support local providers like Ollama and cloud providers like Claude.

### How does the frontend communicate with the backend?

Via HTTP APIs exposed in `src-tauri/src/ipc/routes.rs`. Mentor chat uses server-sent events (SSE) to stream partial responses.

### Where is the database?

A local SQLite database is created in the OS data directory by default. The path is derived in `src-tauri/src/config.rs`.

### What should I work on first?

Good first tasks:
- improve error handling in the frontend and backend.
- add state persistence for conversations.
- wire a new provider implementation.
- build a more complete project/task model in memory.
- add a meaningful E2E smoke test.

---

## 11. Hiring signal and technical story

This project is a great showcase for your technical credibility because it demonstrates:
- cross-stack engineering with React, TypeScript, Rust, and Tauri
- a strong architecture for AI systems
- offline-first and local-first intelligence design
- auditability, observability, and trust-first execution
- phase-gated product development with clear docs

Use the blog posts below to position yourself as the engineer who built a modern AI-native product with real systems thinking.

---

## 12. Blog post drafts

These four posts are written as complete drafts and can be published directly with minor formatting adjustments.

### Blog Post 1: Building an AI-Native Execution OS: Engineering a Trustworthy Local Cognition Layer

**Headline:** Building an AI-Native Execution OS: Engineering a Trustworthy Local Cognition Layer

**Introduction**

Modern AI products are often judged by two qualities: intelligence and trust. Building an AI-native execution platform means making those qualities work together, not just separately. In this post, I explain how I designed `AI-Native Student Execution OS` to be both intelligent and trustworthy by focusing on local-first processing, auditability, and memory-driven AI.

**Why trust matters
**

There is a gap in the AI tooling space between flashy chatbots and actual execution systems. Students and early-career professionals need a tool that helps them make progress, not just have ideas. Trust is the moat: if the system drops context or hides AI failures, users stop relying on it.

**Key design decisions**

- **Offline-first architecture.** The app must work without network access. That means local AI provider support, local SQLite storage, and a desktop-first interface.
- **Provider-agnostic orchestration.** The system does not let engines choose model names. Instead, it routes task requests through an orchestrator that picks the best available provider.
- **Memory as a first-class asset.** Memory is layered and structured. The system stores raw events, normalized entities, and semantic embeddings so AI can recall work reliably.
- **Audit logs for every AI call.** Each request gets a correlation ID, model metadata, and completion event. Visibility is not optional.

**How the solution works**

The desktop app is built with Tauri, React, and Rust. The frontend presents projects, tasks, mentor chat, and audit history. The backend daemon exposes a local HTTP API and handles:
- project/task CRUD
- mentor chat streaming via SSE
- AI routing through local and cloud providers
- memory store management
- audit logging

The orchestrator is the glue. Engines submit a `TaskSpec`, and the orchestrator decides which provider to use, whether local or cloud.

**What the implementation proves**

This architecture shows that you can build AI products with strong engineering discipline. You can support offline usage, avoid vendor lock-in, and still have a smooth streaming chat experience.

**Closing**

If you are building AI systems in 2026, trust and observability cannot be afterthoughts. `AI-Native Student Execution OS` is a practical example of how to design a product that respects the user’s data, the developer’s time, and the reality of unreliable infrastructure.


### Blog Post 2: Why Offline-First AI Product Design Wins in 2026

**Headline:** Why Offline-First AI Product Design Wins in 2026

**Introduction**

The AI industry is chasing bigger models and cloud scale, but one of the most powerful product advantages is still offline-first design. In this post, I explain why `AI-Native Student Execution OS` chooses local-first intelligence and how that decision is a competitive edge for trust, privacy, and user control.

**The problem with cloud-first assumptions**

Many AI products assume the network is always available, the cloud vendor is always responsive, and every user is willing to share their data. Those assumptions fail in real-world contexts such as:
- poor connectivity
- high latency
- privacy-sensitive workflows
- enterprise and education settings

When a product breaks because of a network issue, trust evaporates quickly.

**Offline-first as a product advantage**

Offline-first means the core experience works without network dependency. It also means:
- local data ownership
- predictable performance
- a stronger privacy story
- an ability to degrade gracefully when cloud is unavailable

In `AI-Native Student Execution OS`, the app is built around a Rust daemon with a provider abstraction. Local providers are tried first. Cloud providers are only used when configured and necessary.

**How the implementation supports it**

Key technical features that make offline-first possible:
- **Tauri desktop shell** to run a consistent UI on Windows.
- **Rust daemon** for a reliable local backend.
- **SQLite storage** for local persistence.
- **Provider registry** with local and cloud options.
- **Mock mode** for offline development and deterministic testing.

The app still supports cloud providers, but the baseline is local.

**Product and hiring signal**

Designing for offline-first is a strong signal to hiring managers and engineers because it shows a mindset that prioritizes reliability and real user needs over hype. It is especially relevant in education and productivity tooling.

**Conclusion**

AI teams should stop assuming connectivity is a given. Offline-first is not just a feature; it is a design principle that changes the way you build models, UI, storage, and APIs. `AI-Native Student Execution OS` demonstrates how to make that principle real.


### Blog Post 3: From React to Rust to AI Orchestration: Inside an AI Desktop App Built for Scale

**Headline:** From React to Rust to AI Orchestration: Inside an AI Desktop App Built for Scale

**Introduction**

Building a modern AI desktop product requires joining multiple stacks in a coherent way. In this post, I walk through the architecture of `AI-Native Student Execution OS`, showing how React, TypeScript, Rust, Tauri, Axum, and AI orchestration come together.

**Architecture overview**

The repository is divided cleanly between the frontend and the backend.

- Frontend: `src/`
- Backend daemon: `src-tauri/src/`
- Docs: `project-memory-bank/`

The frontend renders using a shell in `src/App.tsx`. It includes:
- project listing
- project detail and task management
- mentor conversation UI
- audit log

The backend daemon handles:
- HTTP API routes in `src-tauri/src/ipc/routes.rs`
- runtime config in `src-tauri/src/config.rs`
- AI orchestration in `src-tauri/src/ai/orchestrator.rs`
- memory storage in `src-tauri/src/memory/`

**Why Rust for the daemon?**

Rust provides strong safety guarantees, efficient local performance, and a clean way to manage async state. It is a natural fit for this architecture because the daemon is responsible for:
- local persistence
- provider routing
- streaming responses
- memory and audit consistency

**The AI orchestrator**

The orchestrator is a central pattern in the codebase. Engines do not call providers directly. Instead, they create a task specification and hand it to the orchestrator, which:
- selects a provider
- chooses a model
- applies routing policy
- streams output back to the UI
- logs audit events

This separation makes the system easier to extend and more robust against provider changes.

**What the code shows about scale**

Even in an MVP, the repo showcases how to keep complexity manageable:
- typed domain boundaries between frontend and backend
- a memory facade over raw events and entities
- a provider registry that supports local/cloud
- a clear startup path in `src-tauri/src/lib.rs`

**Conclusion**

This architecture is a strong example of how to build AI products that are not just prototypes. It is a blueprint for engineering teams that need to ship usable AI capability with a clean separation between UI, orchestration, and backend state.


### Blog Post 4: How I Designed Local-First, Provider-Agnostic AI with Memory, Audit, and Observability

**Headline:** How I Designed Local-First, Provider-Agnostic AI with Memory, Audit, and Observability

**Introduction**

An AI product is only as good as its ability to maintain context, explain itself, and recover from failure. In this post, I explain the engineering choices behind `AI-Native Student Execution OS`, with a focus on local-first AI, memory architecture, and transparent auditing.

**The key principles**

I designed the system around four principles:
1. **Local-first by default**
2. **Provider-agnostic orchestration**
3. **Layered memory**
4. **No silent failures**

**Local-first by default**

The app prioritizes local AI so core work can continue without network access. The provider registry is implemented to choose local providers first and defer to cloud only when configured.

**Provider-agnostic orchestration**

The core orchestrator receives task-level requirements, not model names. This makes the system easier to maintain, test, and extend. It also means the product can support multiple provider ecosystems.

**Layered memory architecture**

Real AI assistance requires remembering more than chat history. The repo uses:
- L0 for event logs
- L1 for structured project/task entities
- L2 for semantic memory

This design gives the product durable recall and enables future improvements in retrieval and summarization.

**Audit and observability**

Every AI turn is logged with a correlation ID. The audit log is not optional — it is a first-class feature. That means engineers can trace exactly which provider and model were used, and users can see a history of AI decisions.

**What this means for hiring signal**

This project is a compelling portfolio piece because it is not just about models. It is about systems engineering: building an AI product that designers, engineers, and product managers can trust.

**Closing thoughts**

The best AI products are not the ones that only make models look smart; they are the ones that make humans more capable. `AI-Native Student Execution OS` is an example of how to build that kind of product with strong architecture, local-first reliability, and clear observability.

---

## 13. Final checklist for a newbie engineer

- [x] Read `project-memory-bank/64-current-context.md`
- [x] Install Node and Rust
- [x] Run `npm install`
- [x] Run `cargo build --manifest-path src-tauri/Cargo.toml`
- [x] Run `npm run build`
- [x] Run `cargo test -p cognition-daemon`
- [x] Start the app with `npm run tauri dev`
- [x] Inspect `src/App.tsx`, `src-tauri/src/lib.rs`, `src-tauri/src/ipc/routes.rs`, and `src-tauri/src/ai/orchestrator.rs`
- [x] Read `project-memory-bank/15-ai-system-design.md` and `project-memory-bank/16-memory-architecture.md`
- [x] Use the blog drafts to show your thinking publicly

---

## 14. How to use this guide

Keep `AISEOS_QuickStarterGuide.md` as your primary onboarding reference. Use it to answer these questions quickly:
- What is the repo for?
- How do I run it?
- What is implemented now?
- What are the core design documents?
- What are the candidate blog post topics?

If anything in this guide becomes outdated, update this file together with the code change.
