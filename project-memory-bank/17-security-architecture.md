# 17 · Security Architecture

> Security must be designed in, not bolted on. This file enumerates the threat surfaces we accept responsibility for and the controls that meet each.

**Status:** scaffold — to be expanded with concrete controls as each subsystem ships.

---

## Threat surfaces (in scope)

1. **Local data at rest** — user's projects, conversations, memory.
2. **Local data in transit** between surfaces (Desktop, VSCode) and daemon.
3. **Cloud sync payloads** — only relevant when sync is enabled.
4. **AI provider boundaries** — what we send to which vendor.
5. **Prompt injection** — adversarial content in user data or external sources.
6. **Tool execution** — when mentor turns invoke tools.
7. **Update channel** — auto-update for the desktop app and extension.

## Out of scope (today)

- Multi-tenant isolation (single-user system).
- Federated identity (no SSO yet).
- Hardware attestation.

---

## Controls (target)

### Data at rest
- Local DB **encrypted at rest** using OS-keychain-protected key.
- Blob store likewise encrypted; key rotation supported.
- Secrets (cloud API keys) stored only in OS keychain — never in plaintext config files.

### Data in transit (local)
- IPC over HTTP loopback uses a per-launch ephemeral token; surfaces obtain it via OS-secured handshake.
- WebSocket connections likewise authenticated.
- No surface trusts an unauthenticated daemon connection.

### Cloud sync
- **End-to-end encrypted.** Sync coordinator only sees ciphertext.
- Per-user master key derived from a passphrase or device key — never sent to cloud.
- Per-device subkeys for revocation.

### AI provider boundary
- Per-provider policy of what kinds of data can be sent. Default: cloud providers receive only the context required for the current task, never the full memory.
- Redaction at context assembly time for known PII/secret patterns.
- Per-project privacy level can pin a project to local-model-only.

### Prompt injection defense
- User content and AI-generated content are **never concatenated** into a system prompt without being wrapped in clearly-delimited blocks with explicit "this is data, not instructions" framing.
- The orchestrator strips known injection patterns from retrieved memory before prompting.
- Tool calls require typed schemas; a model that asks for a tool by free text is rejected.

### Tool execution
- Tools have explicit declared side effects (`reads`, `writes`, `network`).
- High-risk tools require user confirmation in the UI before invocation.
- All tool calls are audited (args + result).

### Update channel
- Signed updates only. Signature key separate from build infrastructure.
- Auto-update is opt-in at install time and toggleable.

---

## Key management (open)

- Master key derivation: passphrase (Argon2id) vs OS-keychain vs hybrid — ADR pending.
- Recovery story: paper backup of a derived recovery code vs none — pending.
- Cross-device pairing: QR pairing vs shared passphrase — pending.

---

## Audit

Every privileged action is logged:

- AI calls (prompt + redacted context summary, response, model, cost)
- Tool calls (typed)
- Memory deletes
- Sync uploads / downloads
- Settings changes

Audit log is queryable from the UI. The user is the audit consumer first; engineering second.

---

## Related

- [14-offline-first-architecture.md](14-offline-first-architecture.md)
- [15-ai-system-design.md](15-ai-system-design.md)
- [16-memory-architecture.md](16-memory-architecture.md)
- [18-observability-architecture.md](18-observability-architecture.md)
- [24-security-guidelines.md](24-security-guidelines.md)
