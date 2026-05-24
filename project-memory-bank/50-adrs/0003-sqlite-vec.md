# ADR-0003 · Vector Store: sqlite-vec (deferred)

**Status:** ACCEPTED — deferred implementation  
**Date:** 2026-05-24  
**Deciders:** lrameshkumar126  

---

## Context

The memory layer uses L2 for semantic search (embedding-based similarity). The current implementation
uses pure-Rust cosine similarity over `BLOB`-stored f32 arrays in the `embeddings` table (TD-01).
This is correct and offline-first but does not scale beyond ~10k embeddings without performance degradation.

The original stack matrix (D-3) proposed **sqlite-vec** as the vector store.

---

## Decision

**Accept sqlite-vec as the target vector store.** The native integration is **deferred** until
the Tauri packaging pipeline can reliably bundle the sqlite-vec extension.

### Why sqlite-vec

- Zero new processes — runs inside the SQLite connection already open
- Same transaction guarantees as the rest of the L1 data
- Rust bindings via `rusqlite` with `loadable_extension` feature
- Supports ANN (approximate nearest-neighbor) via HNSW index
- Apache 2.0 licensed, maintained by the SQLite authors

### Why deferred

- On Windows + Tauri, bundling a native SQLite extension (`.dll`) requires:
  1. Enabling `loadable_extension` in the rusqlite feature flags
  2. Copying `sqlite_vec.dll` into the Tauri sidecar bundle
  3. Loading it at runtime via `conn.load_extension(...)`
- This requires NSIS/WiX installer customization and cross-compile testing
- Current volume (< 1000 embeddings for typical student usage) makes pure-Rust cosine sim correct

---

## Current Implementation (interim)

```rust
// L2: pure-Rust cosine similarity over BLOB embeddings
// embeddings table: id, entity_type, entity_id, chunk_index, content, embedding (BLOB), dimensions, model, created_at
```

Cosine similarity is computed in the `L2Store::search` method over an in-memory scan.
At ≤ 10k embeddings this is sub-millisecond. At 100k+ it degrades — trigger for migration.

---

## Migration Plan (when ready to implement)

1. Add `rusqlite = { features = ["bundled", "loadable_extension"] }` to Cargo.toml
2. Add `sqlite-vec` crate (or load the `.dll` directly via `unsafe`)
3. Create a migration: `CREATE VIRTUAL TABLE vec_embeddings USING vec0(embedding float[384])`
4. Backfill from `embeddings` table
5. Update `L2Store::search` to use `vec_embeddings` with KNN syntax
6. Update Tauri bundle to include `sqlite_vec.dll`/`.dylib`/`.so`
7. Write ADR-0004 closing this one

---

## Alternatives Considered

| Option | Pro | Con |
|--------|-----|-----|
| LanceDB | Purpose-built, Rust native | New process, higher complexity |
| Qdrant | Production-ready ANN | Separate service, not offline-first |
| Pure-Rust cosine (current) | Zero deps, offline-first | O(n) scan, no indexing |
| sqlite-vec (this ADR) | Same DB, indexed ANN | Bundling complexity on Windows |

---

## Consequences

- TD-01 remains open until sqlite-vec integration is completed
- The `embeddings` table schema is forward-compatible; migration is additive
- `nomic-embed-text` (D-7 from ADR-0002) remains the embedding model — dimensions fixed at 768
- No code change required today; only packaging work blocks the native integration
