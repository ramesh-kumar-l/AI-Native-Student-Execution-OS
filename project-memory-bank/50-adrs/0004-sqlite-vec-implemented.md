# ADR-0004 · sqlite-vec Integration — Status & Blockers

**Status:** ACCEPTED (architecture ready; DLL loading deferred)  
**Date:** 2026-05-24  
**Supersedes:** ADR-0003 (sqlite-vec deferred)

---

## Decision

The dual-path L2Store architecture is implemented (fast-path KNN + cosine fallback).
The sqlite-vec DLL loading approach is confirmed feasible but deferred by one residual blocker.

---

## What was implemented

| Component | Change | Status |
|-----------|--------|--------|
| `db/migrations.rs` | `vec_embedding_map` table + `run_vec()` for virtual table | ✅ |
| `memory/l2.rs` | `search_knn` fast-path + `search_cosine` fallback | ✅ |
| `memory/l2.rs` | `try_insert_vec` dual-write (SQLite + vec index) | ✅ |
| `Cargo.toml` | Architecture in place; no new crate needed yet | ✅ |

### Architecture

```
insert_embedding()
  └── embeddings table (always)
  └── try_insert_vec → vec_embeddings + vec_embedding_map (best-effort)

search_similar()
  ├── search_knn() ← ANN via sqlite-vec (when extension loaded at runtime)
  │     on error (vec_embeddings table missing) →
  └── search_cosine() ← pure-Rust O(n) cosine scan (fallback)
```

---

## Windows linker blocker

**Static compilation (`sqlite-vec = "0.1"` crate) failed on Windows MSVC:**

The `sqlite-vec` crate compiles the C extension and references SQLite API symbols
(`sqlite3_result_null`, `sqlite3_create_module_v2`, etc.) as direct link-time symbols.
On Windows MSVC, the single-pass linker cannot resolve these from rusqlite's bundled
`libsqlite3-sys.a` because the symbol table ordering is incompatible.

The `loadable_extension` feature on rusqlite was also tried but breaks the in-memory
test database with "SQLite API not initialized" (it changes SQLite's initialization path).

**Chosen path for production:** Runtime DLL loading via `conn.load_extension()`.

---

## Remaining work for full sqlite-vec activation

1. Download `sqlite_vec.dll` for Windows from sqlite-vec GitHub releases
2. Bundle as a Tauri sidecar in `src-tauri/tauri.conf.json`
3. In `db/mod.rs`, add a `load_vec_extension(conn, dll_path)` call using
   `rusqlite`'s `loadable_extension` feature (+ `SQLITE_ENABLE_LOAD_EXTENSION`)
4. Re-enable `rusqlite = { features = ["bundled", "loadable_extension"] }`
   (NOTE: `loadable_extension` requires SQLite initialization before any connection
   opens — document initialization order carefully)
5. Write ADR-0004b closing this one

---

## Alternative: pure-Rust HNSW

If DLL bundling remains blocked, implement HNSW with the `instant-distance` crate
(pure Rust, no C extension, Windows-compatible). This achieves the same O(log n)
ANN search without any DLL complexity. The L2Store architecture supports swapping
in this implementation with no change to callers.

---

## Current state

- **Correctness:** ✅ All searches handled correctly via cosine fallback
- **Scale limit:** O(n) scan, degrades at >10k embeddings (same as pre-Phase-6)
- **Production risk:** None — graceful degradation is in place
- **TD-01 status:** Architecture done; fast-path pending DLL or HNSW implementation
