# 23 · Scalability Guidelines

**Status:** scaffold — most scale concerns are *per-user* in this product, not cross-user, until we ship sync at scale.

---

## Per-user scale targets (multi-year horizon)

| Dimension | Target |
| --- | --- |
| Projects | 1,000+ |
| Tasks | 100,000+ |
| Mentor messages | 1,000,000+ |
| Workflow signals | 10,000,000+ |
| Artifacts | 10,000+ |
| Memory L0 events | 100,000,000+ |

These imply: SQLite + sqlite-vec is fine for L1/L2 at our scale; L0 may need partitioning (per-week tables or daily files) past a threshold.

---

## Scale-affecting decisions

- **Memory L0 retention:** unlimited by default; user-configurable archive/forget policies.
- **Embedding cost:** must be incremental; no full re-embed unless the embedding model changes (which is itself a managed migration).
- **Background workers:** must yield to user actions; never starve UI responsiveness.
- **Sync payload size:** chunked uploads; resumable; never one big blob.

---

## Cloud-side scale (post-sync ship)

- Per-user shards; no cross-user joins in the hot path.
- All cloud queries scoped by `user_id` (and eventually `tenant_id`).
- Encrypted blobs are content-addressed → naturally shardable.

---

## Related

- [22-performance-guidelines.md](22-performance-guidelines.md)
- [14-offline-first-architecture.md](14-offline-first-architecture.md)
