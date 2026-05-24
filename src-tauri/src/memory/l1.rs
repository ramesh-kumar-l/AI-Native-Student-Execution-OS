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
                rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
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

    pub async fn list_conversations(&self, limit: u32) -> crate::error::Result<Vec<Conversation>> {
        let convs = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, project_id, title, created_at, updated_at
                     FROM conversations ORDER BY updated_at DESC LIMIT ?1",
                )?;
                let rows = stmt.query_map(rusqlite::params![limit], |row| {
                    Ok(Conversation {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })?;
                rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
            })
            .await?;
        Ok(convs)
    }
}

// ── Project entity ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ── Task entity ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: i32,
    pub due_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

// ── ProjectStore — project + task CRUD on L1Store ─────────────────────────────

impl L1Store {
    // ── Projects ──────────────────────────────────────────────────────────────

    pub async fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> crate::error::Result<Project> {
        let id = Ulid::new().to_string();
        let now = Utc::now().timestamp_millis();
        let project = Project {
            id,
            name,
            description,
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        };
        let p = project.clone();
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO projects (id, name, description, status, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![p.id, p.name, p.description, p.status, p.created_at, p.updated_at],
                )?;
                Ok(())
            })
            .await?;
        Ok(project)
    }

    pub async fn list_projects(&self) -> crate::error::Result<Vec<Project>> {
        let projects = self
            .db
            .conn
            .call(|conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, name, description, status, created_at, updated_at
                     FROM projects WHERE status != 'archived'
                     ORDER BY updated_at DESC",
                )?;
                let rows = stmt.query_map([], |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        status: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                })?;
                Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
            })
            .await?;
        Ok(projects)
    }

    pub async fn get_project(&self, id: &str) -> crate::error::Result<Option<Project>> {
        let id = id.to_string();
        let result = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, name, description, status, created_at, updated_at
                     FROM projects WHERE id = ?1",
                )?;
                let mut rows = stmt.query_map(rusqlite::params![id], |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        status: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                })?;
                Ok(rows.next().transpose()?)
            })
            .await?;
        Ok(result)
    }

    pub async fn update_project(
        &self,
        id: &str,
        name: String,
        description: Option<String>,
        status: String,
    ) -> crate::error::Result<Option<Project>> {
        let id = id.to_string();
        let now = Utc::now().timestamp_millis();
        let result = self
            .db
            .conn
            .call(move |conn| {
                conn.execute(
                    "UPDATE projects SET name=?1, description=?2, status=?3, updated_at=?4
                     WHERE id=?5",
                    rusqlite::params![name, description, status, now, id],
                )?;
                let mut stmt = conn.prepare(
                    "SELECT id, name, description, status, created_at, updated_at
                     FROM projects WHERE id = ?1",
                )?;
                let mut rows = stmt.query_map(rusqlite::params![id], |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        status: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                })?;
                Ok(rows.next().transpose()?)
            })
            .await?;
        Ok(result)
    }

    pub async fn delete_project(&self, id: &str) -> crate::error::Result<()> {
        let id = id.to_string();
        self.db
            .conn
            .call(move |conn| {
                conn.execute("DELETE FROM projects WHERE id=?1", rusqlite::params![id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    // ── Tasks ─────────────────────────────────────────────────────────────────

    pub async fn create_task(
        &self,
        project_id: String,
        title: String,
        description: Option<String>,
        priority: i32,
        due_at: Option<i64>,
    ) -> crate::error::Result<Task> {
        let id = Ulid::new().to_string();
        let now = Utc::now().timestamp_millis();
        let task = Task {
            id,
            project_id,
            title,
            description,
            status: "todo".to_string(),
            priority,
            due_at,
            created_at: now,
            updated_at: now,
        };
        let t = task.clone();
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO tasks
                        (id, project_id, title, description, status, priority, due_at, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    rusqlite::params![
                        t.id, t.project_id, t.title, t.description,
                        t.status, t.priority, t.due_at, t.created_at, t.updated_at
                    ],
                )?;
                Ok(())
            })
            .await?;
        Ok(task)
    }

    pub async fn list_tasks_for_project(
        &self,
        project_id: &str,
    ) -> crate::error::Result<Vec<Task>> {
        let project_id = project_id.to_string();
        let tasks = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, project_id, title, description, status, priority, due_at,
                            created_at, updated_at
                     FROM tasks WHERE project_id=?1
                     ORDER BY priority DESC, created_at ASC",
                )?;
                let rows = stmt.query_map(rusqlite::params![project_id], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        description: row.get(3)?,
                        status: row.get(4)?,
                        priority: row.get(5)?,
                        due_at: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                })?;
                rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
            })
            .await?;
        Ok(tasks)
    }

    pub async fn get_task(&self, id: &str) -> crate::error::Result<Option<Task>> {
        let id = id.to_string();
        let result = self
            .db
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, project_id, title, description, status, priority, due_at,
                            created_at, updated_at
                     FROM tasks WHERE id=?1",
                )?;
                let mut rows = stmt.query_map(rusqlite::params![id], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        description: row.get(3)?,
                        status: row.get(4)?,
                        priority: row.get(5)?,
                        due_at: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                })?;
                Ok(rows.next().transpose()?)
            })
            .await?;
        Ok(result)
    }

    pub async fn update_task(
        &self,
        id: &str,
        title: String,
        description: Option<String>,
        status: String,
        priority: i32,
        due_at: Option<i64>,
    ) -> crate::error::Result<Option<Task>> {
        let id = id.to_string();
        let now = Utc::now().timestamp_millis();
        let result = self
            .db
            .conn
            .call(move |conn| {
                conn.execute(
                    "UPDATE tasks
                     SET title=?1, description=?2, status=?3, priority=?4, due_at=?5, updated_at=?6
                     WHERE id=?7",
                    rusqlite::params![title, description, status, priority, due_at, now, id],
                )?;
                let mut stmt = conn.prepare(
                    "SELECT id, project_id, title, description, status, priority, due_at,
                            created_at, updated_at
                     FROM tasks WHERE id=?1",
                )?;
                let mut rows = stmt.query_map(rusqlite::params![id], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        description: row.get(3)?,
                        status: row.get(4)?,
                        priority: row.get(5)?,
                        due_at: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                })?;
                Ok(rows.next().transpose()?)
            })
            .await?;
        Ok(result)
    }

    pub async fn delete_task(&self, id: &str) -> crate::error::Result<()> {
        let id = id.to_string();
        self.db
            .conn
            .call(move |conn| {
                conn.execute("DELETE FROM tasks WHERE id=?1", rusqlite::params![id])?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}

// ── WorkflowSignal entity ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSignal {
    pub id: String,
    pub project_id: Option<String>,
    pub signal_type: String,
    pub file_path: Option<String>,
    pub language: Option<String>,
    pub payload: serde_json::Value,
    pub source: String,
    pub created_at: i64,
}

impl L1Store {
    pub async fn record_signal(
        &self,
        signal_type: String,
        project_id: Option<String>,
        file_path: Option<String>,
        language: Option<String>,
        payload: serde_json::Value,
        source: String,
    ) -> crate::error::Result<WorkflowSignal> {
        let id = Ulid::new().to_string();
        let now = Utc::now().timestamp_millis();
        let payload_str = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());
        let signal = WorkflowSignal {
            id: id.clone(),
            project_id: project_id.clone(),
            signal_type: signal_type.clone(),
            file_path: file_path.clone(),
            language: language.clone(),
            payload,
            source: source.clone(),
            created_at: now,
        };
        self.db
            .conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO workflow_signals
                        (id, project_id, signal_type, file_path, language, payload, source, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    rusqlite::params![
                        id, project_id, signal_type, file_path, language,
                        payload_str, source, now
                    ],
                )?;
                Ok(())
            })
            .await?;
        Ok(signal)
    }

    pub async fn recent_signals(
        &self,
        project_id: Option<&str>,
        limit: u32,
    ) -> crate::error::Result<Vec<WorkflowSignal>> {
        let project_id = project_id.map(|s| s.to_string());
        let signals = self
            .db
            .conn
            .call(move |conn| {
                let rows: Vec<WorkflowSignal> = if let Some(pid) = project_id {
                    let mut stmt = conn.prepare(
                        "SELECT id, project_id, signal_type, file_path, language,
                                payload, source, created_at
                         FROM workflow_signals WHERE project_id=?1
                         ORDER BY created_at DESC LIMIT ?2",
                    )?;
                    stmt.query_map(rusqlite::params![pid, limit], signal_from_row)?
                        .collect::<rusqlite::Result<Vec<_>>>()?
                } else {
                    let mut stmt = conn.prepare(
                        "SELECT id, project_id, signal_type, file_path, language,
                                payload, source, created_at
                         FROM workflow_signals
                         ORDER BY created_at DESC LIMIT ?1",
                    )?;
                    stmt.query_map(rusqlite::params![limit], signal_from_row)?
                        .collect::<rusqlite::Result<Vec<_>>>()?
                };
                Ok(rows)
            })
            .await?;
        Ok(signals)
    }
}

fn signal_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkflowSignal> {
    let payload_str: String = row.get(5)?;
    Ok(WorkflowSignal {
        id: row.get(0)?,
        project_id: row.get(1)?,
        signal_type: row.get(2)?,
        file_path: row.get(3)?,
        language: row.get(4)?,
        payload: serde_json::from_str(&payload_str)
            .unwrap_or(serde_json::Value::Object(Default::default())),
        source: row.get(6)?,
        created_at: row.get(7)?,
    })
}
