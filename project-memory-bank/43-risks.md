# 43 · Risks

> Known risks with severity, likelihood, and mitigation. Reviewed at each phase-gate.

**As of:** 2026-05-23

Severity = damage if it lands. Likelihood = today's read.

---

## Strategic risks

| ID | Risk | Sev | Lik | Mitigation |
| --- | --- | --- | --- | --- |
| S1 | Wedge audience too small for VC-grade scale | High | Med | Design for institutional adoption (Year 4+) without rework; `02-long-term-strategy.md` |
| S2 | AI commoditization erodes differentiation | High | High | Moat is cognition memory + workflow embeddedness, not model quality |
| S3 | Local-first becomes niche fringe | Med | Low | Keep cloud enhancement first-class; never force purist tradeoffs |
| S4 | Competitor with similar wedge launches first | Med | Med | Speed to memory-bank-anchored Phase 2 MVP |

## Engineering risks

| ID | Risk | Sev | Lik | Mitigation |
| --- | --- | --- | --- | --- |
| E1 | Memory architecture wrong → multi-month rework | High | Med | Invest early; ADR before any code writes to L0 |
| E2 | Provider abstraction leaks vendor-specific behavior | Med | High | Strict TaskSpec contract; cassette tests across providers |
| E3 | Offline-first bolt-on later | High | Low (designing it in now) | Contract enforced from Phase 1 |
| E4 | Sync conflict UX worse than no sync | Med | Med | Defer sync until Phase 5; CRDT only where needed |
| E5 | Rust learning curve slows daemon delivery | Med | Med | Recommendation is Tauri+Rust, but user can pick — alternatives documented |
| E6 | sqlite-vec maturity issues at scale | Med | Low (early stage) | Swap path to LanceDB / Qdrant documented |
| E7 | Prompt injection in retrieved memory | High | Med | Wrap user content with delimiters; sanitize retrieved memory |

## Product / PMF risks

| ID | Risk | Sev | Lik | Mitigation |
| --- | --- | --- | --- | --- |
| P1 | Mentor "feels like a chatbot" → no switching cost | High | Med | Memory + workflow embeddedness as headline differentiator |
| P2 | D30 retention doesn't materialize | High | Med | H2 hypothesis; measure cohort-over-cohort starting Phase 2 |
| P3 | Capability tracking feels inaccurate | Med | Med | Make scores explainable + correctable; H6 hypothesis |
| P4 | Onboarding too heavy | High | Med | Zero-cloud-required first run; W6 workflow quality bar |

## Trust / safety risks

| ID | Risk | Sev | Lik | Mitigation |
| --- | --- | --- | --- | --- |
| T1 | A data breach (cloud sync) breaks trust permanently | Critical | Low | End-to-end encryption; cloud sees only ciphertext |
| T2 | Prompt injection causes harmful tool use | High | Med | Typed tool schema; confirmations for high-risk tools; injection sanitization |
| T3 | Silent model swap → unexpected output | High | Low | Visible degradation contract; never silent |
| T4 | Update channel compromise | Critical | Low | Signed updates; signing key offline |

---

## Risk review cadence

- Each phase-gate: re-read the register; promote/demote.
- New risks discovered during a phase: added immediately with `discovered-in-phase-N` tag.

---

## Related

- [44-technical-debt.md](44-technical-debt.md)
- [17-security-architecture.md](17-security-architecture.md)
- [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md)
