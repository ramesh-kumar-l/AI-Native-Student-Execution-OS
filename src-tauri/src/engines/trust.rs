/// Trust Engine — system health, data lineage, and audit visibility.
/// Gives users a transparent view of what the daemon is doing and where data lives.
use std::sync::Arc;
use std::time::Instant;

use serde::Serialize;
use tracing::warn;

use crate::db::Db;
use crate::error::DaemonError;

pub struct TrustEngine {
    db: Arc<Db>,
    started_at: Instant,
}

#[derive(Debug, Serialize)]
pub struct HealthReport {
    pub daemon_healthy: bool,
    pub db_ok: bool,
    pub uptime_s: u64,
    pub projects: i64,
    pub tasks: i64,
    pub conversations: i64,
    pub messages: i64,
    pub signals: i64,
    pub capability_scores: i64,
    pub artifacts: i64,
}

#[derive(Debug, Serialize)]
pub struct LineageEntry {
    pub id: String,
    pub event_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub correlation_id: String,
    pub created_at: i64,
    pub metadata: Option<serde_json::Value>,
}

impl TrustEngine {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db, started_at: Instant::now() }
    }

    pub async fn health_report(&self) -> HealthReport {
        let uptime_s = self.started_at.elapsed().as_secs();

        let result = self
            .db
            .conn
            .call(|conn| {
                let count = |table: &str| -> rusqlite::Result<i64> {
                    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))
                };
                Ok((
                    count("projects").unwrap_or(0),
                    count("tasks").unwrap_or(0),
                    count("conversations").unwrap_or(0),
                    count("messages").unwrap_or(0),
                    count("workflow_signals").unwrap_or(0),
                    count("capability_scores").unwrap_or(0),
                    count("artifacts").unwrap_or(0),
                ))
            })
            .await;

        match result {
            Ok((projects, tasks, conversations, messages, signals, capability_scores, artifacts)) => {
                HealthReport {
                    daemon_healthy: true,
                    db_ok: true,
                    uptime_s,
                    projects,
                    tasks,
                    conversations,
                    messages,
                    signals,
                    capability_scores,
                    artifacts,
                }
            }
            Err(e) => {
                warn!(err = %e, "health check DB query failed");
                HealthReport {
                    daemon_healthy: true,
                    db_ok: false,
                    uptime_s,
                    projects: 0,
                    tasks: 0,
                    conversations: 0,
                    messages: 0,
                    signals: 0,
                    capability_scores: 0,
                    artifacts: 0,
                }
            }
        }
    }

    pub async fn data_lineage(&self, limit: u32) -> Result<Vec<LineageEntry>, DaemonError> {
        self.db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, event_type, entity_type, entity_id, correlation_id, created_at, metadata
                     FROM audit_log
                     ORDER BY created_at DESC
                     LIMIT ?1",
                )?;
                let collected = stmt
                    .query_map(rusqlite::params![limit], |r| {
                        let meta_str: Option<String> = r.get(6)?;
                        let metadata = meta_str.and_then(|s| serde_json::from_str(&s).ok());
                        Ok(LineageEntry {
                            id: r.get(0)?,
                            event_type: r.get(1)?,
                            entity_type: r.get(2)?,
                            entity_id: r.get(3)?,
                            correlation_id: r.get(4)?,
                            created_at: r.get(5)?,
                            metadata,
                        })
                    })?
                    .collect::<rusqlite::Result<Vec<_>>>()?;
                Ok(collected)
            })
            .await
            .map_err(DaemonError::from)
    }
}
