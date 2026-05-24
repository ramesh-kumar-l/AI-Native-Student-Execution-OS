use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::crypto;
use crate::db::Db;
use crate::error::DaemonError;

pub struct SyncCoordinator {
    db: Arc<Db>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordCounts {
    pub projects: i64,
    pub tasks: i64,
    pub conversations: i64,
    pub messages: i64,
    pub capability_scores: i64,
    pub artifacts: i64,
}

#[derive(Debug, Serialize)]
pub struct ImportSummary {
    pub projects: usize,
    pub tasks: usize,
    pub conversations: usize,
    pub messages: usize,
    pub capability_scores: usize,
    pub artifacts: usize,
}

impl SyncCoordinator {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn record_counts(&self) -> Result<RecordCounts, DaemonError> {
        self.db
            .conn
            .call(|conn| {
                let count = |table: &str| -> rusqlite::Result<i64> {
                    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))
                };
                Ok(RecordCounts {
                    projects: count("projects")?,
                    tasks: count("tasks")?,
                    conversations: count("conversations")?,
                    messages: count("messages")?,
                    capability_scores: count("capability_scores")?,
                    artifacts: count("artifacts")?,
                })
            })
            .await
            .map_err(DaemonError::from)
    }

    /// Export all user data as a JSON payload.
    /// If `passphrase` is given, the data section is AES-256-GCM encrypted (Argon2id key).
    pub async fn export(&self, passphrase: Option<&str>) -> Result<serde_json::Value, DaemonError> {
        let data = self.collect_export_data().await?;
        let exported_at = Utc::now().timestamp_millis();

        if let Some(pass) = passphrase {
            let plaintext = serde_json::to_vec(&data)?;
            let blob = crypto::encrypt(pass, &plaintext)?;
            Ok(json!({
                "version": 1,
                "exported_at": exported_at,
                "encrypted": true,
                "salt": blob.salt,
                "nonce": blob.nonce,
                "ciphertext": blob.ciphertext,
            }))
        } else {
            Ok(json!({
                "version": 1,
                "exported_at": exported_at,
                "encrypted": false,
                "data": data,
            }))
        }
    }

    /// Import a previously exported payload, optionally decrypting first.
    /// Uses INSERT OR REPLACE so existing records are overwritten by the import.
    /// Import order respects FK constraints: projects → conversations → tasks → messages.
    pub async fn import_data(
        &self,
        payload: serde_json::Value,
        passphrase: Option<&str>,
    ) -> Result<ImportSummary, DaemonError> {
        let data: serde_json::Value = if payload["encrypted"] == true {
            let pass = passphrase.ok_or_else(|| {
                DaemonError::Internal("passphrase required for encrypted import".into())
            })?;
            let blob = crypto::EncryptedBlob {
                salt: payload["salt"].as_str().unwrap_or("").to_string(),
                nonce: payload["nonce"].as_str().unwrap_or("").to_string(),
                ciphertext: payload["ciphertext"].as_str().unwrap_or("").to_string(),
            };
            let plaintext = crypto::decrypt(pass, &blob)?;
            serde_json::from_slice(&plaintext)?
        } else {
            payload["data"].clone()
        };

        let projects = data["projects"].as_array().cloned().unwrap_or_default();
        let conversations = data["conversations"].as_array().cloned().unwrap_or_default();
        let tasks = data["tasks"].as_array().cloned().unwrap_or_default();
        let messages = data["messages"].as_array().cloned().unwrap_or_default();
        let capability_scores =
            data["capability_scores"].as_array().cloned().unwrap_or_default();
        let artifacts = data["artifacts"].as_array().cloned().unwrap_or_default();

        let summary = self
            .db
            .conn
            .call(move |conn| {
                conn.execute_batch("BEGIN;")?;

                let mut np = 0usize;
                for p in &projects {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO projects
                             (id, name, description, status, created_at, updated_at)
                             VALUES (?1,?2,?3,?4,?5,?6)",
                            rusqlite::params![
                                p["id"].as_str(),
                                p["name"].as_str(),
                                p["description"].as_str(),
                                p["status"].as_str().unwrap_or("active"),
                                p["created_at"].as_i64(),
                                p["updated_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        np += 1;
                    }
                }

                let mut nc = 0usize;
                for c in &conversations {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO conversations
                             (id, project_id, title, created_at, updated_at)
                             VALUES (?1,?2,?3,?4,?5)",
                            rusqlite::params![
                                c["id"].as_str(),
                                c["project_id"].as_str(),
                                c["title"].as_str(),
                                c["created_at"].as_i64(),
                                c["updated_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        nc += 1;
                    }
                }

                let mut nt = 0usize;
                for t in &tasks {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO tasks
                             (id, project_id, title, description, status, priority, due_at, created_at, updated_at)
                             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                            rusqlite::params![
                                t["id"].as_str(),
                                t["project_id"].as_str(),
                                t["title"].as_str(),
                                t["description"].as_str(),
                                t["status"].as_str().unwrap_or("todo"),
                                t["priority"].as_i64().unwrap_or(1),
                                t["due_at"].as_i64(),
                                t["created_at"].as_i64(),
                                t["updated_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        nt += 1;
                    }
                }

                let mut nm = 0usize;
                for m in &messages {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO messages
                             (id, conversation_id, role, content, model, provider, correlation_id, created_at)
                             VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
                            rusqlite::params![
                                m["id"].as_str(),
                                m["conversation_id"].as_str(),
                                m["role"].as_str(),
                                m["content"].as_str(),
                                m["model"].as_str(),
                                m["provider"].as_str(),
                                m["correlation_id"].as_str(),
                                m["created_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        nm += 1;
                    }
                }

                let mut ns = 0usize;
                for s in &capability_scores {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO capability_scores
                             (id, skill, score, basis, project_id, computed_at)
                             VALUES (?1,?2,?3,?4,?5,?6)",
                            rusqlite::params![
                                s["id"].as_str(),
                                s["skill"].as_str(),
                                s["score"].as_i64(),
                                s["basis"].to_string(),
                                s["project_id"].as_str(),
                                s["computed_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        ns += 1;
                    }
                }

                let mut na = 0usize;
                for a in &artifacts {
                    if conn
                        .execute(
                            "INSERT OR REPLACE INTO artifacts
                             (id, project_id, artifact_type, title, content, format, created_at, updated_at)
                             VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
                            rusqlite::params![
                                a["id"].as_str(),
                                a["project_id"].as_str(),
                                a["artifact_type"].as_str(),
                                a["title"].as_str(),
                                a["content"].as_str(),
                                a["format"].as_str().unwrap_or("markdown"),
                                a["created_at"].as_i64(),
                                a["updated_at"].as_i64(),
                            ],
                        )
                        .is_ok()
                    {
                        na += 1;
                    }
                }

                conn.execute_batch("COMMIT;")?;

                Ok(ImportSummary {
                    projects: np,
                    conversations: nc,
                    tasks: nt,
                    messages: nm,
                    capability_scores: ns,
                    artifacts: na,
                })
            })
            .await?;

        info!(
            projects = summary.projects,
            tasks = summary.tasks,
            "data import complete"
        );
        Ok(summary)
    }

    async fn collect_export_data(&self) -> Result<serde_json::Value, DaemonError> {
        self.db
            .conn
            .call(|conn| {
                // Named `collected` binding in each block forces the MappedRows borrow
                // to drop before stmt goes out of scope (same fix as l1.rs E0597).
                let projects = {
                    let mut stmt = conn.prepare(
                        "SELECT id, name, description, status, created_at, updated_at FROM projects",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "name": r.get::<_, String>(1)?,
                            "description": r.get::<_, Option<String>>(2)?,
                            "status": r.get::<_, String>(3)?,
                            "created_at": r.get::<_, i64>(4)?,
                            "updated_at": r.get::<_, i64>(5)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                let conversations = {
                    let mut stmt = conn.prepare(
                        "SELECT id, project_id, title, created_at, updated_at FROM conversations",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "project_id": r.get::<_, Option<String>>(1)?,
                            "title": r.get::<_, Option<String>>(2)?,
                            "created_at": r.get::<_, i64>(3)?,
                            "updated_at": r.get::<_, i64>(4)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                let tasks = {
                    let mut stmt = conn.prepare(
                        "SELECT id, project_id, title, description, status, priority, due_at, created_at, updated_at FROM tasks",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "project_id": r.get::<_, String>(1)?,
                            "title": r.get::<_, String>(2)?,
                            "description": r.get::<_, Option<String>>(3)?,
                            "status": r.get::<_, String>(4)?,
                            "priority": r.get::<_, i64>(5)?,
                            "due_at": r.get::<_, Option<i64>>(6)?,
                            "created_at": r.get::<_, i64>(7)?,
                            "updated_at": r.get::<_, i64>(8)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                let messages = {
                    let mut stmt = conn.prepare(
                        "SELECT id, conversation_id, role, content, model, provider, correlation_id, created_at
                         FROM messages ORDER BY created_at DESC LIMIT 50000",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "conversation_id": r.get::<_, String>(1)?,
                            "role": r.get::<_, String>(2)?,
                            "content": r.get::<_, String>(3)?,
                            "model": r.get::<_, Option<String>>(4)?,
                            "provider": r.get::<_, Option<String>>(5)?,
                            "correlation_id": r.get::<_, Option<String>>(6)?,
                            "created_at": r.get::<_, i64>(7)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                let capability_scores = {
                    let mut stmt = conn.prepare(
                        "SELECT id, skill, score, basis, project_id, computed_at FROM capability_scores",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "skill": r.get::<_, String>(1)?,
                            "score": r.get::<_, i64>(2)?,
                            "basis": r.get::<_, String>(3)?,
                            "project_id": r.get::<_, Option<String>>(4)?,
                            "computed_at": r.get::<_, i64>(5)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                let artifacts = {
                    let mut stmt = conn.prepare(
                        "SELECT id, project_id, artifact_type, title, content, format, created_at, updated_at
                         FROM artifacts",
                    )?;
                    let collected = stmt.query_map([], |r| {
                        Ok(json!({
                            "id": r.get::<_, String>(0)?,
                            "project_id": r.get::<_, Option<String>>(1)?,
                            "artifact_type": r.get::<_, String>(2)?,
                            "title": r.get::<_, String>(3)?,
                            "content": r.get::<_, String>(4)?,
                            "format": r.get::<_, String>(5)?,
                            "created_at": r.get::<_, i64>(6)?,
                            "updated_at": r.get::<_, i64>(7)?,
                        }))
                    })?.collect::<rusqlite::Result<Vec<_>>>()?;
                    collected
                };

                Ok(json!({
                    "projects": projects,
                    "conversations": conversations,
                    "tasks": tasks,
                    "messages": messages,
                    "capability_scores": capability_scores,
                    "artifacts": artifacts,
                }))
            })
            .await
            .map_err(DaemonError::from)
    }
}
