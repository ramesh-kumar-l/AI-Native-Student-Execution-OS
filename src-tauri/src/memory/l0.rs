/// L0 — append-only event log.
/// Every state change in the system is captured here first.
/// L1–L4 are derived from L0 and can be rebuilt by replay.
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ulid::Ulid;

use crate::db::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L0Event {
    pub id: String,
    pub event_type: String,
    pub payload: Value,
    pub correlation_id: String,
    pub user_id: String,
    pub created_at: i64,
}

/// Typed event kinds the system emits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EventKind {
    MentorTurnReceived {
        conversation_id: String,
        message_preview: String,
    },
    MentorTurnCompleted {
        conversation_id: String,
        message_id: String,
        provider: String,
        model: String,
        tokens_used: Option<u32>,
    },
    ConversationCreated {
        conversation_id: String,
    },
    MessageWritten {
        message_id: String,
        conversation_id: String,
        role: String,
    },
    EmbeddingIndexed {
        entity_type: String,
        entity_id: String,
        chunk_index: u32,
        model: String,
    },
    WorkflowSignalReceived {
        signal_type: String,
        source: String,
    },
    CapabilityComputed {
        skill: String,
        score: i64,
        project_id: Option<String>,
    },
    ArtifactGenerated {
        artifact_id: String,
        artifact_type: String,
        project_id: Option<String>,
    },
}

pub struct L0Store {
    db: Db,
}

impl L0Store {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn append(
        &self,
        event_type: &str,
        payload: &Value,
        correlation_id: &str,
        user_id: &str,
    ) -> crate::error::Result<String> {
        let id = Ulid::new().to_string();
        let id_clone = id.clone();
        let event_type = event_type.to_string();
        let payload_str = serde_json::to_string(payload)?;
        let correlation_id = correlation_id.to_string();
        let user_id = user_id.to_string();
        let now = Utc::now().timestamp_millis();

        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO l0_events (id, event_type, payload, correlation_id, user_id, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![id_clone, event_type, payload_str, correlation_id, user_id, now],
                )?;
                Ok(())
            })
            .await?;

        Ok(id)
    }

    pub async fn append_typed(
        &self,
        event: &EventKind,
        correlation_id: &str,
        user_id: &str,
    ) -> crate::error::Result<String> {
        let event_type = match event {
            EventKind::MentorTurnReceived { .. } => "mentor_turn_received",
            EventKind::MentorTurnCompleted { .. } => "mentor_turn_completed",
            EventKind::ConversationCreated { .. } => "conversation_created",
            EventKind::MessageWritten { .. } => "message_written",
            EventKind::EmbeddingIndexed { .. } => "embedding_indexed",
            EventKind::WorkflowSignalReceived { .. } => "workflow_signal_received",
            EventKind::CapabilityComputed { .. } => "capability_computed",
            EventKind::ArtifactGenerated { .. } => "artifact_generated",
        };
        let payload = serde_json::to_value(event)?;
        self.append(event_type, &payload, correlation_id, user_id).await
    }

    pub async fn query_by_correlation(
        &self,
        correlation_id: &str,
    ) -> crate::error::Result<Vec<L0Event>> {
        let correlation_id = correlation_id.to_string();
        let events = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, event_type, payload, correlation_id, user_id, created_at
                     FROM l0_events WHERE correlation_id = ?1 ORDER BY created_at ASC",
                )?;
                let rows = stmt.query_map(rusqlite::params![correlation_id], |row| {
                    Ok(L0Event {
                        id: row.get(0)?,
                        event_type: row.get(1)?,
                        payload: serde_json::from_str::<Value>(&row.get::<_, String>(2)?)
                            .unwrap_or(Value::Null),
                        correlation_id: row.get(3)?,
                        user_id: row.get(4)?,
                        created_at: row.get(5)?,
                    })
                })?;
                rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
            })
            .await?;
        Ok(events)
    }
}
