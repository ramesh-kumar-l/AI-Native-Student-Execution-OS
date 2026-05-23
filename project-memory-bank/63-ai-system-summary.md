# 63 · AI System Summary

> One-page distillation of `15-ai-system-design.md`.

---

**Goal:** Provider-agnostic, local-first, deterministic-where-possible, auditable, resilient.

**Call shape:** Engine code calls `ai.run(TaskSpec)`. Engines specify *what the task needs* (kind, quality floor, latency budget, cost budget, context policy, safety policy), never *which model*.

**Routing:** Orchestrator picks provider based on policy + availability. Preference order: local → cloud (when user has cloud keys + policy allows). Visible degradation if a worse path is taken; never silent.

**Context assembly:** Orchestrator pulls L4 summaries first, then L3 narrative, then L2 semantic matches, then L1 state. Hard prompt budget; lowest-score items dropped if over, with audit log of what was dropped.

**Providers:**
- Local: Ollama (primary), LM Studio, llama.cpp
- Cloud (via abstraction): Claude, Gemini, OpenAI, DeepSeek, Groq

**Determinism:**
- Embeddings: deterministic per (model, input); cached by content hash
- Generations: pin seed when supported; mark non-deterministic turns in audit log
- Tests: golden cassettes by default; live tests opt-in

**Tool use:** Typed schemas; tools declare side effects; high-risk tools require user confirmation; all tool calls audited.

**Failure handling:**
- Local crash → cloud fallback (if allowed) + banner
- Cloud 5xx → retry once, then local fallback + log degradation
- Context overflow → typed error, never silent truncation
- Malformed output → corrective retry once, then typed error

**Caching:**
- Exact-match (prompt-hash, model, temperature)
- Semantic cache later, opt-in

**Cost / latency observability:** Every call records model, provider, tokens in/out, latency, estimated cost, routing reason. Surfaced in the user's "AI usage" panel and engineering dashboards.

**Safety:**
- Refusal taxonomy is *ours*, consistent across providers
- Prompt injection defense at context-assembly (user content wrapped, retrieved memory sanitized)
- PII redaction policy-driven

**Anti-patterns we reject:**
- Hardcoded vendor names in business logic
- Silent model swap
- Silent context truncation
- Cached output served past TTL without indication
