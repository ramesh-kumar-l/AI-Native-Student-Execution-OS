# 22 · Performance Guidelines

**Status:** scaffold — budgets formalize when we have measurement.

---

## Target budgets (initial)

| Surface | Action | Target |
| --- | --- | --- |
| Desktop UI | Cold start to first interactive | < 2s |
| Desktop UI | Hot navigation | < 100ms |
| VSCode extension | Activation overhead | < 100ms |
| Daemon | IPC round-trip (non-AI) | < 30ms p95 |
| Daemon | Memory retrieval (semantic top-k=10) | < 200ms p95 |
| AI | Mentor TTFT (local 7B) | < 1.5s p95 |
| AI | Mentor TTFT (cloud) | < 800ms p95 |

These are aspirational until measured.

---

## Principles

- **Measure first.** No performance work without a baseline.
- **Latency budgets per layer** — not just overall — so regressions are diagnosable.
- **No global locks on the hot path.** Anything that blocks UI thread is a defect.
- **Background work is queued and rate-limited**, never spinning loops.
- **Memory layer reads are the most-frequent hot path** — optimize retrieval before generation.

---

## Common pitfalls to avoid

- Re-embedding entire memory on every change (incremental only).
- Synchronous AI calls from the UI thread.
- Holding a DB write transaction across an AI call.
- Logging large payloads at `info` (downgrade to `trace`).

---

## Related

- [18-observability-architecture.md](18-observability-architecture.md)
- [23-scalability-guidelines.md](23-scalability-guidelines.md)
