# 62 · Service Summary

> One paragraph per engine. For depth see `11-service-boundaries.md`, `12-domain-models.md`, and the engine-specific files.

---

**Cognition Engine** — owns the memory layers (L0 events, L1 normalized entities, L2 vector index, L3 temporal/relational graph, L4 compressed summaries). Every other engine reads/writes memory through its interface. Cognition is the moat.

**Execution Engine** — owns projects, roadmaps, tasks, sessions. The user-visible "what am I working on" surface lives here. Emits events that feed Capability and Memory.

**Capability Engine** — derives skill signals over time from Workflow + Execution + Mentor data. Produces capability scores and narratives ("what you got better at"). Every score has provenance — the user can see which signals contributed.

**Mentor Engine** — orchestrates conversational AI mentorship. Builds TaskSpecs, hands them to the Orchestrator, streams responses. Memory-aware: a returning user is not starting from zero.

**Workflow Engine** — captures signals from the desktop and the VSCode extension (active file, language, build/test events, session boundaries). Non-blocking; never interferes with user actions. Best-effort queueing on the extension side.

**Artifact Engine** — generates portfolio entries, project pages, narratives. Outputs are shareable-quality, not AI-slop. Tied to project + source data so claims are traceable.

**Trust Engine** — observes everything. Surfaces audit log (user-visible), reliability gates, explainability links (mentor "see context", capability "see why"). Read-mostly: it doesn't drive user actions, but no user action is invisible to it.

---

**AI Orchestration Layer** — provider-agnostic router. Engines never know which model served a request. Routing by policy: preferred (local/cloud/auto), quality floor, latency budget, cost budget.

**Memory Layer** — multi-mode store + retrieval API. Reads serve assembled, prompt-budget-aware context. Writes go through L0 first; derived layers update asynchronously.

**Cloud Sync (optional, Phase 5+)** — eventual-consistent, end-to-end-encrypted. The coordinator sees only ciphertext.
