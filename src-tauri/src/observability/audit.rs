/// Structured audit log — every AI call, memory write, and sync action is recorded here.
/// "No silent failures" principle (CLAUDE.md §6): audit failure is logged but never crashes.
use chrono::Utc;
use serde_json::json;
use tracing::error;
use ulid::Ulid;

use crate::db::Db;

pub struct AuditLog {
    db: Db,
}

#[derive(Debug)]
pub struct AuditRecord {
    pub event_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub user_id: String,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: String,
}

impl AuditRecord {
    pub fn ai_call_started(
        provider: &str,
        model: &str,
        correlation_id: &str,
        user_id: &str,
    ) -> Self {
        Self {
            event_type: "ai_call_started".to_string(),
            entity_type: Some("ai_call".to_string()),
            entity_id: None,
            user_id: user_id.to_string(),
            metadata: Some(json!({ "provider": provider, "model": model })),
            correlation_id: correlation_id.to_string(),
        }
    }

    pub fn ai_call_completed(
        provider: &str,
        model: &str,
        prompt_tokens: u32,
        completion_tokens: u32,
        correlation_id: &str,
        user_id: &str,
    ) -> Self {
        Self {
            event_type: "ai_call_completed".to_string(),
            entity_type: Some("ai_call".to_string()),
            entity_id: None,
            user_id: user_id.to_string(),
            metadata: Some(json!({
                "provider": provider,
                "model": model,
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens,
            })),
            correlation_id: correlation_id.to_string(),
        }
    }

    pub fn mentor_turn_received(conversation_id: &str, correlation_id: &str, user_id: &str) -> Self {
        Self {
            event_type: "mentor_turn_received".to_string(),
            entity_type: Some("conversation".to_string()),
            entity_id: Some(conversation_id.to_string()),
            user_id: user_id.to_string(),
            metadata: None,
            correlation_id: correlation_id.to_string(),
        }
    }

    pub fn mentor_turn_completed(
        conversation_id: &str,
        message_id: &str,
        correlation_id: &str,
        user_id: &str,
    ) -> Self {
        Self {
            event_type: "mentor_turn_completed".to_string(),
            entity_type: Some("conversation".to_string()),
            entity_id: Some(conversation_id.to_string()),
            user_id: user_id.to_string(),
            metadata: Some(json!({ "message_id": message_id })),
            correlation_id: correlation_id.to_string(),
        }
    }
}

impl AuditLog {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn record(
        &self,
        record: AuditRecord,
    ) -> crate::error::Result<String> {
        let id = Ulid::new().to_string();
        let id_clone = id.clone();
        let now = Utc::now().timestamp_millis();
        let metadata_str = record
            .metadata
            .as_ref()
            .and_then(|m| serde_json::to_string(m).ok());

        let result = self
            .db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO audit_log
                        (id, event_type, entity_type, entity_id, user_id, metadata, correlation_id, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    rusqlite::params![
                        id_clone,
                        record.event_type,
                        record.entity_type,
                        record.entity_id,
                        record.user_id,
                        metadata_str,
                        record.correlation_id,
                        now,
                    ],
                )?;
                Ok(())
            })
            .await;

        if let Err(ref e) = result {
            error!(err = %e, "audit log write failed — recording to structured log only");
        }

        result?;
        Ok(id)
    }

    pub async fn recent(
        &self,
        limit: u32,
    ) -> crate::error::Result<Vec<crate::ipc::types::AuditEntryDto>> {
        let rows = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, event_type, correlation_id, created_at, metadata
                     FROM audit_log ORDER BY created_at DESC LIMIT ?1",
                )?;
                let rows = stmt.query_map(rusqlite::params![limit], |row| {
                    Ok(crate::ipc::types::AuditEntryDto {
                        id: row.get(0)?,
                        event_type: row.get(1)?,
                        correlation_id: row.get(2)?,
                        created_at: row.get(3)?,
                        metadata: row
                            .get::<_, Option<String>>(4)?
                            .and_then(|s| serde_json::from_str(&s).ok()),
                    })
                })?;
                rows.collect::<rusqlite::Result<Vec<_>>>()
            })
            .await?;
        Ok(rows)
    }
}
