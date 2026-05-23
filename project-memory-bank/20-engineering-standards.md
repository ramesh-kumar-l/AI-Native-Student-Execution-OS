# 20 · Engineering Standards

> Standards exist to make changes cheap and trust durable. Follow them; if you can't follow one, write an ADR explaining why.

---

## Repo & branching

- **Trunk-based.** `main` is always shippable.
- Feature branches off `main`; PR-back. No long-lived release branches in Phase 0–1.
- Tag releases with `vMAJOR.MINOR.PATCH`. Semver from v1.0; pre-v1 we use `v0.x.y` with looser guarantees.
- ADRs land as PRs into `project-memory-bank/50-adrs/`.

## Code review

- Every change goes through PR review (even if there's one engineer — review your own PR cold the next day).
- PR description must answer:
  1. **Why** is this change being made? (link to memory-bank file or issue)
  2. **What** changed, in plain language?
  3. **Risk** — what could break?
  4. **Validation** — how was this verified?
- Reviewer checks: scope discipline (no drive-by changes), test coverage, memory-bank updated if relevant.

## Commits

- Imperative present tense (`add roadmap entity`, not `added` / `adds`).
- One logical change per commit when reasonable; squash on merge.
- No commented-out code in commits.
- No "fix", "wip", "stuff" messages — write what the commit does.

## Style

- **Match existing style.** Don't reformat unless it's the focus of the change.
- Formatters and linters are gates in CI; pre-commit hooks recommended.
- For each language we adopt, document its formatter + linter in this file (deferred until stack is chosen).

## Folder structure (conventions)

- `apps/` — runnable surfaces (desktop, vscode-extension).
- `crates/` or `packages/` — daemon engines, libraries.
- `project-memory-bank/` — canonical docs (this directory).
- `scripts/` — automation; everything here must have `--help`.
- `tests/` — cross-app integration tests if any.

(Pre-stack-decision; concrete layout lands with the stack ADR.)

## Dependencies

- **Pin** dependencies (`Cargo.lock`, `package-lock.json`, etc. committed).
- Adding a runtime dep requires: (a) it's actively maintained, (b) license-compatible (Apache 2.0 or compatible), (c) cannot be replaced by a few-line implementation.
- Removing a dep is always cheaper than swapping later.

## Testing (overview — see `21-testing-strategy.md`)

- Unit tests required for non-trivial logic.
- Integration tests for engine boundaries.
- Golden cassettes for AI provider tests (deterministic; live tests opt-in).
- End-to-end smoke tests for each surface.

## CI gates

- Lint, format check, type check, unit + integration tests, security audit (`cargo audit`, `npm audit`, etc.).
- Cross-platform desktop builds (macOS / Windows / Linux) on tagged releases.
- VSCode extension package build on every PR.

## Documentation

- Every public engine interface gets a docstring describing its contract and failure modes.
- Architecture-affecting changes update the relevant memory-bank file in the same PR.
- ADRs for any decision that future engineers (or AI assistants) might second-guess.

## Tooling philosophy

- Local-first dev environment: a fresh clone + a single setup script should produce a working build.
- No "works on my machine" hidden state.
- Reproducible builds where the ecosystem supports them.

## Engineering anti-patterns (do not do)

- Adding configuration "in case we need it later."
- Wrapping a 3-line stdlib call in a "service" abstraction.
- Premature performance optimization without measurement.
- Catch-and-ignore error handling.
- Silent fallback behavior (always log + surface).
- Mocking the world in tests so the test doesn't exercise the failure modes it claims to cover.

---

## Related

- [21-testing-strategy.md](21-testing-strategy.md)
- [22-performance-guidelines.md](22-performance-guidelines.md)
- [24-security-guidelines.md](24-security-guidelines.md)
- [25-ai-reliability-guidelines.md](25-ai-reliability-guidelines.md)
