# CLAUDE.md — AI-Native Student Execution OS

Project-specific operating rules. These extend the user's global `~/.claude/CLAUDE.md` and the canonical AUTOPROMPT preserved at `project-memory-bank/99-autoprompt-source.md`.

When these rules conflict with general defaults, **these win**.

---

## 1. Memory-bank-first protocol

Before any non-trivial task you MUST:

1. Read `project-memory-bank/64-current-context.md` (loaded first — compressed orientation).
2. Read only the memory-bank files relevant to the task.
3. Read ADRs under `project-memory-bank/50-adrs/` that touch the area you're changing.

Do NOT scan the whole repo. Do NOT recursively grep unrelated code. Token budget is a first-class constraint.

If memory-bank content is missing or stale for the task at hand, **update it as part of the work** — don't proceed on assumptions.

---

## 2. Phase-gated execution

After completing each implementation phase you MUST stop and produce a phase-gate summary:

- What was implemented
- Architectural decisions made
- Tradeoffs accepted
- Risks introduced
- Technical debt opened
- Recommended next phase

Then wait for explicit approval before continuing. **Never auto-advance phases.**

---

## 3. Offline-first / local-first

Every feature must answer:

- Does it work with no network?
- Does it work with no cloud LLM?
- Where does user data live, and who owns it?

If the answer to any of these is "doesn't work" — that's a design failure unless the user has explicitly accepted it.

Cloud is an *enhancement layer*, never a dependency for core workflows.

---

## 4. AI-provider agnosticism

Never hardcode against a specific model vendor in business logic. All AI calls go through the provider abstraction (see `project-memory-bank/15-ai-system-design.md`). Supported targets include local (Ollama, LM Studio, llama.cpp) and cloud (Claude, Gemini, OpenAI, DeepSeek, Groq).

---

## 5. Engineering before coding

For any task more complex than a typo fix, produce this section *before* writing code:

1. Understanding
2. Relevant memory files
3. Architecture analysis
4. Proposed design
5. Risks
6. Implementation plan
7. Validation strategy
8. Technical debt
9. Rollback strategy

---

## 6. Observability is mandatory

Every module exposes structured logs, metrics, traces, and correlation IDs. **No silent failures.** If you don't know how a failure would surface to a user or operator, you haven't finished the design.

---

## 7. Surgical changes

Every changed line must trace to the requested task. No drive-by refactors, no formatting churn in unrelated code, no "while I'm here" cleanups. If you spot dead code or rot, **mention it** — don't silently edit it.

---

## 8. Trust over flash

Never optimize for demo polish at the cost of reliability, predictability, or user agency. The product thesis is "AI you can rely on for execution" — every UX moment must reinforce that.

---

## 9. Stack decisions are pending

As of 2026-05-23 the desktop framework, local DB, vector store, sync engine, and AI-provider abstraction implementations are **NOT chosen**. Treat references to specific tech in memory files as *proposed options under evaluation* unless an ADR has been merged in `project-memory-bank/50-adrs/`.

---

## 10. Failure modes to avoid

You FAIL this project's contract if you:

- Rewrite stable systems unnecessarily
- Scan irrelevant files / load full repos
- Hallucinate APIs
- Ignore offline constraints
- Ignore token efficiency
- Continue phases automatically
- Break compatibility silently
- Skip observability
- Optimize for short-term velocity at the cost of long-term integrity
