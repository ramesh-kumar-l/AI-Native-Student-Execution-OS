# 32 · User Workflows

> Concrete user journeys the product must support well. Each maps to mission jobs in `01-product-mission.md`.

**Status:** scaffold — to be elaborated into wireframes/flows as UI design starts.

---

## W1 · Morning planning

**Trigger:** User opens app first thing in the morning.

**Flow:**
1. App shows: today's plan (auto-derived from roadmap + yesterday's status).
2. Mentor offers a one-sentence framing ("today's first move is X because Y").
3. User edits/confirms plan.
4. App enters "execute mode."

**Quality bar:** No empty states. Even on day 1, the system bootstraps with a guided onboarding plan.

---

## W2 · Deep work session

**Trigger:** User starts a work session on a task.

**Flow:**
1. Session marker created; VSCode extension begins emitting signals.
2. Mentor available in side panel; conversation pinned to session.
3. App stays out of the way (no nudges during deep work).
4. Session close → end-of-session summary + capability deltas.

**Quality bar:** Zero notifications during a session. Notifications are an anti-feature here.

---

## W3 · Mentor consultation

**Trigger:** User wants to think through a problem.

**Flow:**
1. User opens mentor.
2. Context auto-assembled from current project + relevant memory.
3. Conversation runs (local model by default, cloud if user routes).
4. Conversation close → summary, capability tag, file under project.

**Quality bar:** Mentor never starts from zero on a returning user. Memory is the difference.

---

## W4 · Capability check-in

**Trigger:** User wants to know what they're getting better at.

**Flow:**
1. Open capability surface.
2. Week-over-week deltas shown with provenance ("you got better at X because of signals A, B, C").
3. User can mark a capability narrative as "feels accurate" / "off."
4. Feedback feeds the capability scoring model.

**Quality bar:** Every claim is traceable; no opaque scores.

---

## W5 · Artifact export

**Trigger:** User wants to publish a project / portfolio entry.

**Flow:**
1. From a project, "Generate artifact" → choose format (project page, narrative, exportable bundle).
2. Artifact Engine drafts; user edits.
3. User publishes locally (file) or via sync coordinator (shareable URL).
4. Audit log records what was generated and what was published.

**Quality bar:** Artifacts are shareable-quality; not "AI-slop" templated text.

---

## W6 · Onboarding (first run)

**Trigger:** Fresh install.

**Flow:**
1. No sign-in required. Local-first by default.
2. Optional: install Ollama prompt (if not detected) — explain what it gives.
3. Bootstrap a guided "first week" plan.
4. Cloud account is an *optional* layer for sync, added later from settings.

**Quality bar:** A user can be productive in ≤5 minutes without any cloud setup.

---

## W7 · Offline session

**Trigger:** User opens the app with no network.

**Flow:**
1. Banner: "Offline — local model only."
2. All core flows continue to work.
3. Queued sync operations resume on reconnect; nothing lost.

**Quality bar:** Offline is *first-class*, not a degraded mode that feels broken.

---

## Related

- [01-product-mission.md](01-product-mission.md)
- [33-retention-strategy.md](33-retention-strategy.md)
- [30-roadmap.md](30-roadmap.md)
