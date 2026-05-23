/// L1 — normalized entity store.
/// Structured, queryable entities derived from L0 events.
use chrono::Utc;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::db::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub project_id: Option<String>,
    pub title: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub correlation_id: Option<String>,
    pub created_at: i64,
}

pub struct L1Store {
    db: Db,
}

impl L1Store {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    // ── Conversations ────────────────────────────────────────────────────────

    pub async fn create_conversation(
        &self,
        project_id: Option<String>,
    ) -> crate::error::Result<Conversation> {
        let id = Ulid::new().to_string();
        let now = Utc::now().timestamp_millis();
        let conv = Conversation {
            id: id.clone(),
            project_id,
            title: None,
            created_at: now,
            updated_at: now,
        };
        let conv_clone = conv.clone();
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO conversations (id, project_id, title, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![
                        conv_clone.id,
                        conv_clone.project_id,
                        conv_clone.title,
                        conv_clone.created_at,
                        conv_clone.updated_at
                    ],
                )?;
                Ok(())
            })
            .await?;
        Ok(conv)
    }

    pub async fn get_conversation(
        &self,
        id: &str,
    ) -> crate::error::Result<Option<Conversation>> {
        let id = id.to_string();
        let result = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, project_id, title, created_at, updated_at
                     FROM conversations WHERE id = ?1",
                )?;
                let mut rows = stmt.query_map(rusqlite::params![id], |row| {
                    Ok(Conversation {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })?;
                Ok(rows.next().transpose()?)
            })
            .await?;
        Ok(result)
    }

    pub async fn touch_conversation(&self, id: &str) -> crate::error::Result<()> {
        let id = id.to_string();
        let now = Utc::now().timestamp_millis();
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
                    rusqlite::params![now, id],
                )?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    // ── Messages ─────────────────────────────────────────────────────────────

    pub async fn insert_message(&self, msg: Message) -> crate::error::Result<Message> {
        let msg_clone = msg.clone();
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO messages
                        (id, conversation_id, role, content, model, provider, correlation_id, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    rusqlite::params![
                        msg_clone.id,
                        msg_clone.conversation_id,
                        msg_clone.role,
                        msg_clone.content,
                        msg_clone.model,
                        msg_clone.provider,
                        msg_clone.correlation_id,
                        msg_clone.created_at,
                    ],
                )?;
                Ok(())
            })
            .await?;
        Ok(msg)
    }

    pub async fn messages_for_conversation(
        &self,
        conversation_id: &str,
        limit: u32,
    ) -> crate::error::Result<Vec<Message>> {
        let conv_id = conversation_id.to_string();
        let messages = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, conversation_id, role, content, model, provider, correlation_id, created_at
                     FROM messages
                     WHERE conversation_id = ?1
                     ORDER BY created_at ASC
                     LIMIT ?2",
                )?;
                let rows = stmt.query_map(rusqlite::params![conv_id, limit], |row| {
                    Ok(Message {
                        id: row.get(0)?,
                        conversation_id: row.get(1)?,
                        role: row.get(2)?,
                        content: row.get(3)?,
                        model: row.get(4)?,
                        provider: row.get(5)?,
                        correlation_id: row.get(6)?,
                        created_at: row.get(7)?,
                    })
                })?;
                rows.collect::<rusqlite::Result<Vec<_>>>()
            })
            .await?;
        Ok(messages)
    }

    /// Build `Message` with a generated ID. Callers shouldn't construct IDs manually.
    pub fn new_message(
        conversation_id: &str,
        role: &str,
        content: &str,
        model: Option<String>,
        provider: Option<String>,
        correlation_id: Option<String>,
    ) -> Message {
        Message {
            id: Ulid::new().to_string(),
            conversation_id: conversation_id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            model,
            provider,
            correlation_id,
            created_at: Utc::now().timestamp_millis(),
        }
    }
}
