# 04 · PMF Hypotheses

These are the testable bets. Each hypothesis has a falsification condition — if we can't disprove it, we keep going; if we do disprove it, we redesign.

---

## H1 · Workflow embeddedness drives daily-active usage

**Bet:** Users who install the VSCode extension AND use the desktop OS in the same week will retain at materially higher rates (≥2×) than desktop-only users.

**Falsifies if:** After we have ≥500 weekly-active users for 8 weeks, retention curves do not separate.

**Implication if true:** VSCode is the wedge distribution channel; we double down there.
**Implication if false:** The desktop OS must stand alone; the VSCode layer is enhancement, not core.

---

## H2 · Cognition memory creates switching cost

**Bet:** Users who pass 30 days of usage will have built enough cognition memory (projects, mentor conversations, artifacts) that they refuse to start over elsewhere — measured by net churn dropping after day 30.

**Falsifies if:** Day-30+ churn is statistically indistinguishable from day-7 churn after sufficient cohort size.

**Implication if true:** Memory architecture investment is justified; export/portability is the trust mechanism, not a churn risk.
**Implication if false:** We're not as moat-y as we think; need to add more workflow-level lock-in (or accept we're a tool, not a platform).

---

## H3 · Artifact generation drives shareability → acquisition

**Bet:** Users who generate ≥1 portfolio artifact will share it publicly at a rate that produces measurable referral traffic (≥10% of new signups attributable).

**Falsifies if:** Artifact share rate is <2% of generations OR referral traffic from shared artifacts is <1% of acquisition.

**Implication if true:** Artifact engine is a growth lever, not just a retention feature.
**Implication if false:** Artifacts are a depth feature only; growth strategy must come from elsewhere.

---

## H4 · Offline-first matters to the wedge audience

**Bet:** A non-trivial fraction (≥20%) of our wedge audience will report using core features while offline (travel, intermittent connectivity, intentional focus mode) and rate this as "important" or higher.

**Falsifies if:** Survey + telemetry show offline usage <5% AND users do not list offline as a top-5 differentiator.

**Implication if true:** Offline-first is a *marketed* feature, not just an architectural choice.
**Implication if false:** Offline remains an architectural choice (for resilience) but we don't lead with it in messaging.

---

## H5 · Local LLM viability for core mentor flows

**Bet:** A modern local model (7B–14B class, e.g. Llama 3.x / Qwen / Mistral via Ollama) can serve the mentor flow well enough that ≥40% of mentor turns can run locally without users routing them to cloud.

**Falsifies if:** Even with prompt engineering + RAG, local-only mentor turns produce user-rated quality below cloud-routed turns by a margin that drives users to disable local-only mode.

**Implication if true:** Cloud is genuinely optional for the wedge audience.
**Implication if false:** Cloud remains required for quality; local is a *fallback* not a *primary*, and our messaging shifts accordingly.

---

## H6 · Capability tracking earns the "I'm getting better" feeling

**Bet:** The Capability Engine, given workflow sensing data + execution data, can surface week-over-week capability deltas that users find **accurate and motivating**.

**Falsifies if:** ≥40% of users in feedback say capability signals feel "off," "wrong," or "noise."

**Implication if true:** Capability tracking is a headline feature.
**Implication if false:** Capability tracking is an internal signal for the mentor, not a user-facing dashboard.

---

## How we'll measure

See `34-pmf-tracking.md` for the metric definitions, instrumentation plan, and review cadence.

---

## Related

- [00-project-vision.md](00-project-vision.md)
- [03-market-positioning.md](03-market-positioning.md)
- [33-retention-strategy.md](33-retention-strategy.md)
- [34-pmf-tracking.md](34-pmf-tracking.md)
