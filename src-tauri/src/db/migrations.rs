use rusqlite::Connection;

/// Run all schema migrations in order. Idempotent — safe to call on every startup.
pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    // ── L0 · append-only event log ────────────────────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS l0_events (
            id           TEXT PRIMARY KEY,
            event_type   TEXT NOT NULL,
            payload      TEXT NOT NULL,      -- JSON
            correlation_id TEXT NOT NULL,
            user_id      TEXT NOT NULL DEFAULT 'local',
            created_at   INTEGER NOT NULL    -- Unix ms
        );
        CREATE INDEX IF NOT EXISTS idx_l0_corr
            ON l0_events(correlation_id);
        CREATE INDEX IF NOT EXISTS idx_l0_type_ts
            ON l0_events(event_type, created_at DESC);",
    )?;

    // ── L1 · normalized entities ─────────────────────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS conversations (
            id         TEXT PRIMARY KEY,
            project_id TEXT,
            title      TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS messages (
            id              TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL
                REFERENCES conversations(id) ON DELETE CASCADE,
            role            TEXT NOT NULL
                CHECK(role IN ('user', 'assistant', 'system')),
            content         TEXT NOT NULL,
            model           TEXT,
            provider        TEXT,
            correlation_id  TEXT,
            created_at      INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_messages_conv
            ON messages(conversation_id, created_at);",
    )?;

    // ── L2 · vector embeddings (pure-Rust cosine similarity; ADR-0002 D-3) ──
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS embeddings (
            id          TEXT PRIMARY KEY,
            entity_type TEXT NOT NULL,
            entity_id   TEXT NOT NULL,
            chunk_index INTEGER NOT NULL DEFAULT 0,
            content     TEXT NOT NULL,
            embedding   BLOB NOT NULL,   -- little-endian f32 array
            dimensions  INTEGER NOT NULL,
            model       TEXT NOT NULL,
            created_at  INTEGER NOT NULL,
            UNIQUE(entity_type, entity_id, chunk_index)
        );
        CREATE INDEX IF NOT EXISTS idx_embed_entity
            ON embeddings(entity_type, entity_id);",
    )?;

    // ── Audit log ─────────────────────────────────────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS audit_log (
            id             TEXT PRIMARY KEY,
            event_type     TEXT NOT NULL,
            entity_type    TEXT,
            entity_id      TEXT,
            user_id        TEXT NOT NULL DEFAULT 'local',
            metadata       TEXT,           -- JSON
            correlation_id TEXT NOT NULL,
            created_at     INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_audit_corr
            ON audit_log(correlation_id);
        CREATE INDEX IF NOT EXISTS idx_audit_ts
            ON audit_log(created_at DESC);",
    )?;

    // ── Phase 2 · Projects & Tasks ────────────────────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS projects (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT,
            status      TEXT NOT NULL DEFAULT 'active'
                            CHECK(status IN ('active','paused','completed','archived')),
            created_at  INTEGER NOT NULL,
            updated_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_projects_status
            ON projects(status, updated_at DESC);

        CREATE TABLE IF NOT EXISTS tasks (
            id          TEXT PRIMARY KEY,
            project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            title       TEXT NOT NULL,
            description TEXT,
            status      TEXT NOT NULL DEFAULT 'todo'
                            CHECK(status IN ('todo','in_progress','done')),
            priority    INTEGER NOT NULL DEFAULT 1,
            due_at      INTEGER,
            created_at  INTEGER NOT NULL,
            updated_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_tasks_project
            ON tasks(project_id, status, priority DESC);
        CREATE INDEX IF NOT EXISTS idx_tasks_due
            ON tasks(due_at) WHERE due_at IS NOT NULL;",
    )?;

    // ── Phase 3 · Workflow signals from VSCode ───────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS workflow_signals (
            id          TEXT PRIMARY KEY,
            project_id  TEXT,
            signal_type TEXT NOT NULL,
            file_path   TEXT,
            language    TEXT,
            payload     TEXT NOT NULL DEFAULT '{}',
            source      TEXT NOT NULL DEFAULT 'vscode',
            created_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_signals_project
            ON workflow_signals(project_id, created_at DESC);
        CREATE INDEX IF NOT EXISTS idx_signals_recent
            ON workflow_signals(created_at DESC);",
    )?;

    // ── Phase 4 · Capability scores & Artifacts ──────────────────────────────
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS capability_scores (
            id          TEXT PRIMARY KEY,
            skill       TEXT NOT NULL,
            score       INTEGER NOT NULL,   -- 0-100
            basis       TEXT NOT NULL DEFAULT '{}',  -- JSON: which signals contributed
            project_id  TEXT,               -- NULL = cross-project aggregate
            computed_at INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_cap_scores_project
            ON capability_scores(project_id, skill, computed_at DESC);
        CREATE INDEX IF NOT EXISTS idx_cap_scores_recent
            ON capability_scores(computed_at DESC);

        CREATE TABLE IF NOT EXISTS artifacts (
            id            TEXT PRIMARY KEY,
            project_id    TEXT,
            artifact_type TEXT NOT NULL,    -- 'project_page','portfolio_export','capability_narrative'
            title         TEXT NOT NULL,
            content       TEXT NOT NULL,    -- markdown
            format        TEXT NOT NULL DEFAULT 'markdown',
            created_at    INTEGER NOT NULL,
            updated_at    INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_artifacts_project
            ON artifacts(project_id, created_at DESC);
        CREATE INDEX IF NOT EXISTS idx_artifacts_type
            ON artifacts(artifact_type, created_at DESC);",
    )?;

    Ok(())
}
