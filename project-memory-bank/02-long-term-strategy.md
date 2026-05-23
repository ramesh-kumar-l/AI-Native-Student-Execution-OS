# 02 · Long-Term Strategy

## Horizon

**5–10 years.** This file is the arc connecting today's bootstrap to "global cognition infrastructure for human capability."

---

## Strategic phases

| Horizon | Surface area | What we're proving |
| --- | --- | --- |
| **Year 0–1 · Bootstrap** | Desktop OS + VSCode extension; AI-native engineering students | The product can be daily-used; the cognition memory creates real switching cost |
| **Year 1–2 · PMF for builders** | Same surface, broader builder audience (designers, indie devs, technical founders) | The workflow-sensing + mentor combination earns retention on its own merits |
| **Year 2–4 · Capability graph** | Public artifacts, hire-quality portfolio output, employer-side read APIs | Employability transfer — measurable evidence that capability tracked here moves career outcomes |
| **Year 4–7 · Cognition infrastructure** | Multi-tenant orgs (bootcamps, universities, employers); ambient mobile layer; voice | Institutional adoption without compromising local-first guarantees |
| **Year 7–10 · Platform** | Third-party engines, capability standards, federated cognition | The platform survives any single vendor including us |

---

## Strategic moats

We deliberately build for these compounding advantages:

1. **Cognition memory ownership.** Every day a user uses the product, switching cost grows because their memory is here. *Mitigation against churn.*
2. **Workflow embeddedness.** We live in the IDE and the desktop, not in a tab. *Mitigation against attention competition.*
3. **Provider abstraction.** Any AI vendor going hostile, dark, or expensive does not break the product. *Mitigation against vendor risk.*
4. **Offline-first design.** Connectivity issues, censorship, or cloud outages do not block users. *Mitigation against infrastructure risk.*
5. **Local-first data ownership.** Users keep their data — this is a moat *against trust collapse* in centralized AI.

None of these are flashy. All of them compound.

---

## What we deliberately defer

- Mobile is **deferred until desktop + VSCode loops retain.** Cognition infrastructure with weak retention on the primary surface won't recover by adding more surfaces.
- Voice is **deferred until the text-mode mentor is genuinely useful.** Voice on top of a mediocre mentor is a worse mentor.
- Multi-tenant institutional features are **deferred until PMF among individual builders.** Individual product-market fit is the foundation; institutional sales without it is doomed.
- Third-party engines are **deferred until our own engines stabilize.** Premature platforms ossify around the wrong primitives.

---

## What we deliberately invest in early (against speed)

- Memory architecture. *(See `16-memory-architecture.md`.)* Wrong here, everything compounds wrong.
- AI provider abstraction. *(See `15-ai-system-design.md`.)* Coupling here is a multi-year bug.
- Offline-first sync. *(See `14-offline-first-architecture.md`.)* Bolting this on later is multi-month work.
- Observability + auditability. *(See `18-observability-architecture.md`.)* You cannot retrofit trust.

These are the four areas where we accept slower Year-1 velocity in exchange for not having to rebuild in Year 3.

---

## Strategic risks

See `43-risks.md` for the full register. Top three from a *strategic* (not engineering) lens:

1. **Builder audience too small for VC-grade growth** → counter-strategy: design for institutional adoption (Year 4+) without architectural rework.
2. **AI commoditization eats our differentiation** → counter-strategy: differentiation is *cognition memory + workflow embeddedness*, not model quality.
3. **Local-first becomes a niche / fringe story** → counter-strategy: keep cloud enhancement first-class so we never force users into purist tradeoffs.

---

## Related

- [00-project-vision.md](00-project-vision.md)
- [03-market-positioning.md](03-market-positioning.md)
- [30-roadmap.md](30-roadmap.md)
