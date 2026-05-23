# Project Memory Bank

**This directory is the canonical context source for every engineering and product decision in this repo.**

Read this index. Then read `64-current-context.md` first. Then load only the files you need for the task at hand. Do not scan the whole repo to "be thorough" — that's an anti-pattern this project explicitly rejects.

---

## Read order for a new session

1. `64-current-context.md` — compressed orientation (~1 page)
2. `40-current-phase.md` — what phase we're in and why
3. `45-next-steps.md` — pending decisions and unblocking work
4. Then load files specific to the task

If `64-current-context.md` says it's stale, refresh it before proceeding.

---

## Index

### 00–04 · Strategic
| File | Purpose |
| --- | --- |
| [00-project-vision.md](00-project-vision.md) | What we're building and why it must exist |
| [01-product-mission.md](01-product-mission.md) | Daily-life mission for the product |
| [02-long-term-strategy.md](02-long-term-strategy.md) | Multi-year arc → cognition infrastructure |
| [03-market-positioning.md](03-market-positioning.md) | Who this is for, who it isn't |
| [04-pmf-hypotheses.md](04-pmf-hypotheses.md) | Testable bets about reaching PMF |

### 10–18 · Architecture
| File | Purpose |
| --- | --- |
| [10-system-architecture.md](10-system-architecture.md) | Top-level system + stack proposals |
| [11-service-boundaries.md](11-service-boundaries.md) | Logical boundaries (stack-agnostic) |
| [12-domain-models.md](12-domain-models.md) | Core entities & relationships |
| [13-event-flows.md](13-event-flows.md) | Key event flows end-to-end |
| [14-offline-first-architecture.md](14-offline-first-architecture.md) | Offline contracts & sync model |
| [15-ai-system-design.md](15-ai-system-design.md) | Provider abstraction, routing, RAG |
| [16-memory-architecture.md](16-memory-architecture.md) | Cognition memory layers & retrieval |
| [17-security-architecture.md](17-security-architecture.md) | Encryption, auth, isolation, injection defense |
| [18-observability-architecture.md](18-observability-architecture.md) | Logs / metrics / traces / audit |

### 20–25 · Engineering
| File | Purpose |
| --- | --- |
| [20-engineering-standards.md](20-engineering-standards.md) | Code, review, branching, CI |
| [21-testing-strategy.md](21-testing-strategy.md) | Levels & gates for tests |
| [22-performance-guidelines.md](22-performance-guidelines.md) | Latency / memory budgets |
| [23-scalability-guidelines.md](23-scalability-guidelines.md) | Local + sync scale concerns |
| [24-security-guidelines.md](24-security-guidelines.md) | Day-to-day secure-coding rules |
| [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md) | Determinism, retries, fallbacks |

### 30–34 · Product
| File | Purpose |
| --- | --- |
| [30-roadmap.md](30-roadmap.md) | Phased product roadmap |
| [31-feature-priorities.md](31-feature-priorities.md) | Priority matrix per phase |
| [32-user-workflows.md](32-user-workflows.md) | Core user journeys |
| [33-retention-strategy.md](33-retention-strategy.md) | Retention via embedded workflows |
| [34-pmf-tracking.md](34-pmf-tracking.md) | Signals we'll measure |

### 40–45 · Execution
| File | Purpose |
| --- | --- |
| [40-current-phase.md](40-current-phase.md) | What phase, what's in scope |
| [41-active-tasks.md](41-active-tasks.md) | Tasks underway right now |
| [42-implementation-status.md](42-implementation-status.md) | What's built vs planned |
| [43-risks.md](43-risks.md) | Known risks & mitigations |
| [44-technical-debt.md](44-technical-debt.md) | Debt log with cost & remediation |
| [45-next-steps.md](45-next-steps.md) | Pending decisions blocking progress |

### 50–53 · Decisions
| Path | Purpose |
| --- | --- |
| [50-adrs/](50-adrs/) | Architecture Decision Records |
| [51-technical-decisions/](51-technical-decisions/) | Smaller technical decisions |
| [52-rejected-approaches/](52-rejected-approaches/) | Things we considered & rejected, with reasons |
| [53-postmortems/](53-postmortems/) | Incident postmortems |

### 60–64 · Compressed Context
| File | Purpose |
| --- | --- |
| [60-architecture-summary.md](60-architecture-summary.md) | One-page architecture |
| [61-current-state-summary.md](61-current-state-summary.md) | What exists today |
| [62-service-summary.md](62-service-summary.md) | One-paragraph-per-service |
| [63-ai-system-summary.md](63-ai-system-summary.md) | AI system in one page |
| [64-current-context.md](64-current-context.md) | **Loaded first by every session** |

### Reference
| File | Purpose |
| --- | --- |
| [99-autoprompt-source.md](99-autoprompt-source.md) | Verbatim canonical AUTOPROMPT |

---

## Conventions

- Files use kebab-case prefixed with their numeric slot.
- Cross-reference with relative links: `[label](14-offline-first-architecture.md)`.
- When a file becomes stale, update it in the same PR as the change that invalidated it.
- Decisions go in `50-adrs/` as numbered ADRs; the strategic/architecture files reference the ADR, they don't replace it.

---

## How memory-bank relates to auto-memory

`project-memory-bank/` is the **shared, versioned** cognition for the project — humans and AI both read it, it's in git.

`C:\Users\lrame\.claude\projects\E--ClaudeProjects-AI-Native-Student-Execution-OS\memory\` is the **AI assistant's personal long-term memory** — user preferences, working style, pointers back to this memory-bank. It's not in git.

The auto-memory points *into* this memory-bank, not the other way around.
