# 18 · Observability Architecture

> "No silent failures" is a contractual obligation. This file describes how every part of the system surfaces what it's doing — to operators (engineering) and to users (audit + degraded-mode UX).

**Status:** scaffold — to be expanded as instrumentation ships.

---

## Three audiences

| Audience | What they see | How |
| --- | --- | --- |
| **User** | What the AI did, when sync ran, what was sent to which vendor, what's degraded right now | In-app audit log + status banners + settings telemetry panel |
| **Engineering (local)** | Logs, metrics, traces from a running daemon | Local log files + a debug HUD overlay (dev mode) |
| **Engineering (aggregate)** | Crash reports + opt-in telemetry from real users | Opt-in only; aggregated; never per-user without consent |

The user-facing surface is the first audience. If a thing is happening, the user can find out.

---

## Logging

- **Structured JSON** logs from every component, with correlation IDs.
- Levels: `trace`, `debug`, `info`, `warn`, `error`.
- Per-subsystem log filters configurable in settings.
- Logs rotate locally; never uploaded without explicit opt-in.

---

## Metrics

Local metrics collected and visible in the in-app diagnostics panel:

- AI calls: count, latency p50/p95, error rate, per-provider breakdown
- Memory layer: write throughput, retrieval latency, embedding queue depth
- Sync: bytes up/down, conflicts resolved, last successful sync timestamp
- Engine health: per-engine heartbeat, queue depth, error rate

---

## Tracing

- **OpenTelemetry-style spans** from UI action → daemon command → engine work → AI call → response.
- Correlation IDs propagate through all engine boundaries and into AI provider call metadata where supported.
- Trace export is local-only by default; cloud export is opt-in for users running self-hosted backends.

---

## Audit log (user-facing observability)

Distinct from engineering logs. Surfaces:

- Every AI call (model, provider, redacted-context summary, cost, latency)
- Every memory delete (what, when, cascade)
- Every sync action
- Every tool invocation
- Every settings change

Queryable + filterable from the UI. This is also part of the trust story.

---

## Failure visibility patterns

- **Degraded banners** for every degraded mode (offline, local-model down, cloud rate-limited, sync paused, etc.).
- **Health page** in settings showing engine heartbeats and any open alerts.
- **Action history** so user can see *why* the AI did what it did on any given turn.

---

## Telemetry & privacy

- All telemetry is **off by default**.
- Opt-in granular: crash reports, usage metrics, feature flags — separate toggles.
- Nothing personally identifying is sent without explicit consent on a per-category basis.
- A "see exactly what's being sent" panel shows the next telemetry batch payload before it leaves the device.

---

## SLOs (target, when we have users)

| SLO | Target |
| --- | --- |
| Local mentor turn TTFT (text) | < 1.5s p95 with local 7B model on modern hardware |
| Workflow signal write | < 50ms p95 |
| Memory retrieval (semantic, top-k=10) | < 200ms p95 |
| Sync round-trip after reconnect | < 30s p95 for typical user state |

To be revisited once we have measurement.

---

## Related

- [15-ai-system-design.md](15-ai-system-design.md)
- [17-security-architecture.md](17-security-architecture.md)
- [22-performance-guidelines.md](22-performance-guidelines.md)
- [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md)
