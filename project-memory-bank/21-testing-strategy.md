# 21 · Testing Strategy

**Status:** scaffold — to be expanded once the stack is chosen and the first engines exist.

---

## Levels

| Level | Scope | Frequency |
| --- | --- | --- |
| Unit | Pure logic, single function/module | Every PR |
| Integration | Engine boundaries, DB, memory layer end-to-end | Every PR |
| AI (cassette) | Orchestrator + provider abstraction with recorded responses | Every PR |
| AI (live, opt-in) | Real provider calls; checked into CI but skipped by default | Nightly / on demand |
| End-to-end | Desktop UI / VSCode extension driving a real daemon | Pre-release |
| Smoke | Critical user paths after install | Pre-release + post-deploy |

---

## Principles

- **Test the contract, not the implementation.** Tests should survive internal refactors.
- **No mocked databases for integration tests** — use the real SQLite (in-memory or tmp file).
- **Golden cassettes for AI** — deterministic recorded responses are the default for AI tests. Live tests exist but are opt-in.
- **Failure-mode tests are non-negotiable** for offline, degraded-provider, and sync-conflict paths.
- **Tests are also documentation** — name them as English sentences that describe behavior.

---

## Coverage targets (initial — to be refined)

- Engines: ≥80% line coverage on logic modules; lower on glue code is acceptable.
- Memory layer: 100% on retrieval-path code (critical correctness).
- AI orchestrator routing logic: 100%.

We do not chase coverage for its own sake. Coverage gaps are reviewed; intentional gaps are documented in PR descriptions.

---

## Open items

- Test framework choice per language (deferred to stack ADR).
- E2E driver for desktop UI (Playwright? WebdriverIO? Tauri-specific?).
- VSCode extension test harness (`@vscode/test-electron`).
- Performance regression suite (when we have a perf baseline).

---

## Related

- [20-engineering-standards.md](20-engineering-standards.md)
- [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md)
