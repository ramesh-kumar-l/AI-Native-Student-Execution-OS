use std::{path::Path, sync::Arc};
use tokio_rusqlite::Connection;
use tracing::{info, warn};

pub mod migrations;

/// Thin wrapper around tokio-rusqlite Connection with initialization logic.
#[derive(Clone)]
pub struct Db {
    pub conn: Arc<Connection>,
}

impl Db {
    /// Open (or create) the SQLite database at `path`, running all migrations.
    pub async fn open(path: &Path) -> crate::error::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path).await?;
        let db = Self { conn: Arc::new(conn) };
        db.run_migrations().await?;
        db.run_vec_migrations().await;
        info!(path = %path.display(), "database opened");
        Ok(db)
    }

    /// Open an in-memory database for testing.
    pub async fn open_in_memory() -> crate::error::Result<Self> {
        let conn = Connection::open_in_memory().await?;
        let db = Self { conn: Arc::new(conn) };
        db.run_migrations().await?;
        db.run_vec_migrations().await;
        Ok(db)
    }

    async fn run_migrations(&self) -> crate::error::Result<()> {
        self.conn
            .call(|conn| {
                migrations::run(conn)?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    /// Create the vec_embeddings virtual table. Requires the sqlite-vec extension
    /// to be loaded at runtime (ADR-0004). Fails gracefully — vector search falls
    /// back to cosine scan when the extension is unavailable.
    async fn run_vec_migrations(&self) {
        if let Err(e) = self.conn
            .call(|conn| {
                migrations::run_vec(conn)?;
                Ok(())
            })
            .await
        {
            warn!(
                err = ?e,
                "sqlite-vec extension unavailable — vec_embeddings not created; \
                 vector search falls back to cosine scan (see ADR-0004)"
            );
        }
    }
}
