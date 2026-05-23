# 25 · AI Reliability Guidelines

> Reliability is a *product* requirement, not a *bonus*. These rules govern how AI-touching code is written, tested, and shipped.

**Status:** scaffold — expand alongside the orchestrator and engines.

---

## Reliability rules

1. **Every AI call must be retryable.** Idempotency via `cache_key` or correlation ID.
2. **Every AI call must have a fallback.** Either a worse-but-functional path or a typed error the UI handles.
3. **Never silently swap models.** Visible degradation only.
4. **Stream cancellation is a feature.** Users can cancel any in-flight turn.
5. **Tool calls fail closed.** Unknown tool name → refuse, log, surface to user.
6. **Context overflow returns a typed error**, never silent truncation.

---

## Determinism & testing

- Default to **golden-cassette** AI tests. Record once; replay forever.
- Re-record only with reviewer sign-off (cassette diff is part of the PR).
- Live AI tests are opt-in (CI label or local flag).
- For embeddings: pin the model version; treat the version as part of the cache key.

---

## Output validation

- When an AI is generating structured output (JSON, code, tool call), validate against a schema. Reject on parse failure, retry with a corrective prompt, then fail.
- Never trust AI output to satisfy invariants the system relies on. Validate, then enforce.

---

## Audit & explainability

- Every AI-driven decision visible in the audit log with the inputs that produced it.
- Mentor turns surface a "see context" link that shows which memory items were retrieved and why.
- Capability scores show their basis (which signals contributed).

---

## Degradation patterns

| When | Do |
| --- | --- |
| Local model unavailable | Surface; offer cloud route if allowed |
| Cloud unavailable | Surface; route to local; if quality insufficient, ask user |
| Both unavailable | Surface clearly; queue user input for retry |
| Provider returns malformed output | Retry with corrective prompt once, then escalate |

---

## Related

- [15-ai-system-design.md](15-ai-system-design.md)
- [18-observability-architecture.md](18-observability-architecture.md)
- [21-testing-strategy.md](21-testing-strategy.md)
