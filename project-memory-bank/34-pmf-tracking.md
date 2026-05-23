# 34 · PMF Tracking

> What we'll measure, how we'll measure it, and when we'll declare a hypothesis falsified.

**Status:** scaffold — telemetry to be added as features ship.

---

## Headline metrics

| Metric | Definition | Phase first measurable |
| --- | --- | --- |
| **WAU** | Weekly-active users (any meaningful action) | Phase 2 |
| **D7 retention** | Cohort of new installs still active in week 2 | Phase 2 |
| **D30 retention** | Cohort still active at day 30 | Phase 2 (cohorts mature in Phase 3) |
| **Memory mass / user** | Median bytes (or entity count) of memory per active user | Phase 1 (instrumented), surfaces in Phase 2 |
| **Mentor turn quality** | Self-rated thumbs-up / down per turn | Phase 2 |
| **Workflow attachment** | Median minutes/day with VSCode extension active during session | Phase 3 |
| **Artifact share rate** | Artifacts generated → publicly shared | Phase 4 |
| **Local-only session share** | Sessions completed without any cloud LLM | Phase 1 (telemetry), interpretable Phase 2+ |

---

## Per-hypothesis tracking

(See `04-pmf-hypotheses.md` for the hypotheses themselves.)

| Hypothesis | Primary metric | Falsification cohort size | Falsification window |
| --- | --- | --- | --- |
| H1 · Workflow embeddedness | D7 retention: VSCode+desktop vs desktop-only | 500 WAU | 8 weeks |
| H2 · Memory creates switching cost | D30 vs D7 churn delta | 200 cohort | 12 weeks |
| H3 · Artifacts drive acquisition | Share rate; referral share of new signups | 100 artifacts generated | 8 weeks post-Phase-4 |
| H4 · Offline matters | % users with offline-session share >5%; survey importance score | 200 WAU | 8 weeks |
| H5 · Local LLM viability | % mentor turns local-only; user-rated quality parity | 100 WAU | 12 weeks |
| H6 · Capability tracking | "Accurate" / "off" feedback ratio on capability narratives | 100 WAU | 8 weeks post-Phase-4 |

---

## Review cadence

- **Weekly** during PMF push: metric review with the prior week's hypothesis status.
- **Monthly**: revisit hypotheses; mark confirmed / falsified / still-open.
- **Per-phase exit**: PMF status check is part of the phase-gate summary.

---

## Instrumentation plan

- Daemon-level event logging is the source. Each headline metric has a defined event schema.
- Aggregation is **on-device** by default; cloud aggregation requires opt-in telemetry.
- User-visible "see your own metrics" panel for transparency.

---

## Related

- [04-pmf-hypotheses.md](04-pmf-hypotheses.md)
- [33-retention-strategy.md](33-retention-strategy.md)
- [18-observability-architecture.md](18-observability-architecture.md)
