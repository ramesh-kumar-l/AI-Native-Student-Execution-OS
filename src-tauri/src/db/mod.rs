use std::{path::{Path, PathBuf}, sync::Arc};
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
        if let Some(path) = sqlite_vec_extension_path() {
            let ext_path = path.clone();
            if let Err(e) = self
                .conn
                .call(move |conn| {
                    unsafe {
                        conn.load_extension(ext_path.to_string_lossy().as_ref(), None)?;
                    }
                    Ok(())
                })
                .await
            {
                warn!(
                    err = ?e,
                    path = %path.display(),
                    "sqlite-vec extension failed to load — continuing without fast-path ANN"
                );
            } else {
                info!(path = %path.display(), "sqlite-vec extension loaded");
            }
        }

        if let Err(e) = self
            .conn
            .call(|conn| {
                migrations::run_vec(conn)?;
                Ok(())
            })
            .await
        {
            warn!(
                err = ?e,
                "sqlite-vec extension unavailable — vec_embeddings not created; vector search falls back to cosine scan (see ADR-0004)"
            );
        }
    }
}

fn sqlite_vec_extension_path() -> Option<PathBuf> {
    // Prefer an explicit override when one is provided. This lets devs point to a
    // downloaded sqlite_vec.dll from a known path without bundling it into the
    // application before the file is available.
    if let Ok(path) = std::env::var("SQLITE_VEC_EXTENSION_PATH") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    let filename = default_sqlite_vec_extension_filename();

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            let candidate = dir.join(filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join(filename);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn default_sqlite_vec_extension_filename() -> &'static str {
    if cfg!(target_os = "windows") {
        "sqlite_vec.dll"
    } else if cfg!(target_os = "macos") {
        "libsqlite_vec.dylib"
    } else {
        "libsqlite_vec.so"
    }
}
