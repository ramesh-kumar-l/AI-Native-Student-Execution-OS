# 24 · Security Guidelines (day-to-day)

> The architectural controls live in `17-security-architecture.md`. This file is the **operational** layer: rules engineers follow line-by-line.

**Status:** scaffold — expand as concrete subsystems land.

---

## Code-level rules

- **Never** log secrets, API keys, full user prompts at `info` or above.
- **Never** write secrets to config files. OS keychain only.
- Parameterize all SQL; never string-concat user input into SQL.
- Validate all data crossing a boundary (UI → daemon, daemon → providers, providers → daemon).
- Treat AI output as **untrusted input** when it informs downstream code paths (no `eval`, no shell exec).
- Authenticate every IPC connection (ephemeral per-launch token).

## Tool use

- Tools declare their side effects (`reads`, `writes`, `network`).
- High-risk tools require explicit per-invocation user confirmation.
- Tool args are typed; freeform-text tool calls are rejected.

## Prompt injection

- User content is wrapped in delimiters when injected into prompts.
- Retrieved memory is sanitized for known injection patterns.
- The model is instructed (system prompt) that data blocks are not instructions.
- Tool calls require structured schemas — model cannot "free-form invoke" a tool.

## Dependencies

- Run `cargo audit` / `npm audit` in CI. New vulns block merge.
- Don't auto-update major versions — review the changelog.
- Don't add a dep you can't explain the entire purpose of.

## Data deletion

- Implement true delete (not just soft-delete) for user-initiated forgetting.
- Cascade L1–L4 on delete; emit forget-event into L0; "shred" command can nuke L0 too.

## Updates

- Signed updates only.
- Update signature key is **not** in build infrastructure.
- Users can pin a version and decline updates.

## Sharing & exports

- Artifacts marked `shareable` get sanitized exports (strip non-shareable embedded references).
- Imports validate signatures and are sandboxed during processing.

---

## Related

- [17-security-architecture.md](17-security-architecture.md)
- [20-engineering-standards.md](20-engineering-standards.md)
