# AI-Native Student Execution OS

> A trustworthy AI-native platform that helps students and future professionals **execute consistently**, build real projects, and compound capability over time — evolving into global cognition infrastructure for human capability building.

---

## What this is

Three form factors, one cognition layer:

| Form factor                     | Status   | Role                                                                 |
| ------------------------------- | -------- | -------------------------------------------------------------------- |
| **Desktop Execution OS**        | Primary  | Deep work, roadmap planning, project execution, cognition memory     |
| **VSCode Capability Intelligence** | Secondary | Workflow sensing, architecture awareness, capability signal extraction |
| **Ambient Mobile Cognition**    | Future   | Lightweight mentorship, reminders, reflection, voice                 |

Underneath all three sit seven engines: Cognition, Execution, Capability, Mentor, Workflow, Artifact, Trust.

---

## Operating principles (non-negotiable)

1. **Trust first** — reliability > flash
2. **Offline-first** — works with no network, no cloud LLM
3. **Local-first cognition** — user owns their data
4. **Memory-first development** — read `project-memory-bank/` before coding
5. **Token efficiency is a feature**, not an afterthought
6. **Phase-gated execution** — stop, summarize, wait between phases
7. **AI-provider agnostic** — supports Ollama / LM Studio / llama.cpp locally; Claude / Gemini / OpenAI / DeepSeek / Groq in the cloud
8. **Observability mandatory** — no silent failures

Full directive: `project-memory-bank/99-autoprompt-source.md`
Project rules for contributors and AI assistants: `CLAUDE.md`

---

## Repository layout

```
.
├── CLAUDE.md                       # Operating rules for AI assistants in this repo
├── README.md                       # You are here
├── LICENSE                         # Apache 2.0
└── project-memory-bank/            # PRIMARY CONTEXT SOURCE
    ├── README.md                   # Index of all memory-bank files
    ├── 00-04 strategic/            # Vision, mission, strategy, positioning, PMF
    ├── 10-18 architecture/         # System, services, domain, events, offline, AI, memory, security, observability
    ├── 20-25 engineering/          # Standards, testing, perf, scalability, security, AI reliability
    ├── 30-34 product/              # Roadmap, priorities, workflows, retention, PMF tracking
    ├── 40-45 execution/            # Current phase, tasks, status, risks, tech debt, next steps
    ├── 50-53 decisions/            # ADRs, technical decisions, rejected approaches, postmortems
    ├── 60-64 compressed-context/   # Architecture summary, current state, services, AI, current context
    └── 99-autoprompt-source.md     # Verbatim canonical directive
```

No application code exists yet — Phase 0 (Foundation) is in progress. See `project-memory-bank/40-current-phase.md`.

---

## Status

**Phase 0: Foundation** — memory-bank bootstrap, stack-decision preparation, ADR scaffolding.

Stack (desktop framework, local DB, vector store, sync engine, AI provider abstraction) is **awaiting decision** — see `project-memory-bank/10-system-architecture.md` and `project-memory-bank/45-next-steps.md`.

---

## License

Apache 2.0 — see `LICENSE`.
