/// Capability Engine — aggregates workflow signals + task data into skill scores.
/// Scores are computed locally (pure Rust, offline-first).
/// Narrative generation uses the AI orchestrator with a local-model fallback template.
use chrono::{DateTime, Utc};
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{error, info};
use crate::ai::task::{
    AuditContext, ChatMessage, DeterminismPolicy, MessageRole, RoutingPolicy, TaskInputs,
    TaskKind, TaskSpec,
};
use crate::ai::Orchestrator;
use crate::error::DaemonError;
use crate::memory::l0::EventKind;
use crate::memory::l1::{CapabilityScore, Task, WorkflowSignal};
use crate::memory::MemoryStore;
use crate::observability::audit::AuditLog;

pub struct CapabilityEngine {
    memory: Arc<MemoryStore>,
    orchestrator: Arc<Orchestrator>,
    _audit: Arc<AuditLog>,
}

impl CapabilityEngine {
    pub fn new(
        memory: Arc<MemoryStore>,
        orchestrator: Arc<Orchestrator>,
        audit: Arc<AuditLog>,
    ) -> Self {
        Self { memory, orchestrator, _audit: audit }
    }

    /// Compute all 5 skill scores from recent signals and tasks.
    /// Stores each score in the DB and emits L0 audit events.
    pub async fn compute_scores(
        &self,
        project_id: Option<&str>,
        correlation_id: &str,
    ) -> Result<Vec<CapabilityScore>, DaemonError> {
        let signals = self.memory.l1.recent_signals(project_id, 500).await?;
        let tasks = if let Some(pid) = project_id {
            self.memory.l1.list_tasks_for_project(pid).await?
        } else {
            vec![]
        };

        let raw = compute_raw_scores(&signals, &tasks);
        let mut stored = Vec::with_capacity(raw.len());

        for (skill, score, basis) in raw {
            let cs = self
                .memory
                .l1
                .insert_capability_score(skill.clone(), score, basis, project_id.map(str::to_string))
                .await?;

            self.memory
                .l0
                .append_typed(
                    &EventKind::CapabilityComputed {
                        skill,
                        score,
                        project_id: project_id.map(str::to_string),
                    },
                    correlation_id,
                    "local",
                )
                .await
                .ok();

            stored.push(cs);
        }

        info!(count = stored.len(), project_id = ?project_id, "capability scores computed");
        Ok(stored)
    }

    /// Generate an AI narrative explaining the current capability scores.
    /// Falls back to a structured template if the AI provider is unreachable (offline-first).
    pub async fn generate_narrative(
        &self,
        project_id: Option<&str>,
        correlation_id: &str,
    ) -> Result<String, DaemonError> {
        let scores = self.memory.l1.latest_scores(project_id).await?;

        if scores.is_empty() {
            return Ok(
                "No capability data yet. Write some code, run debug sessions, and \
                 complete tasks to build your capability profile."
                    .to_string(),
            );
        }

        let score_lines = scores
            .iter()
            .map(|s| format!("- {}: {}/100", s.skill, s.score))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "You are an AI mentor analyzing a developer's recent activity. \
             Based on these skill scores (0-100), write 2-3 short sentences: \
             highlight the top strength, note one growth area, give one specific tip. \
             Be encouraging and personal. Max 80 words.\n\nScores:\n{}",
            score_lines
        );

        let task = TaskSpec {
            task_kind: TaskKind::CapabilityScore,
            inputs: TaskInputs {
                messages: vec![ChatMessage { role: MessageRole::User, content: prompt }],
                conversation_id: None,
                project_id: project_id.map(str::to_string),
                context: vec![],
            },
            routing_policy: RoutingPolicy::default(),
            determinism: DeterminismPolicy { temperature: 0.5, ..Default::default() },
            audit: AuditContext {
                user_id: "local".to_string(),
                correlation_id: correlation_id.to_string(),
            },
        };

        match self.orchestrator.run(&task).await {
            Ok(mut stream) => {
                let mut content = String::new();
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(sc) => {
                            content.push_str(&sc.content);
                            if sc.done {
                                break;
                            }
                        }
                        Err(e) => {
                            error!(err = %e, "narrative stream error");
                            break;
                        }
                    }
                }
                let narrative = content.trim().to_string();
                if narrative.is_empty() {
                    Ok(offline_narrative(&scores))
                } else {
                    Ok(narrative)
                }
            }
            Err(e) => {
                error!(err = %e, "AI unavailable for narrative, using template");
                Ok(offline_narrative(&scores))
            }
        }
    }
}

fn offline_narrative(scores: &[CapabilityScore]) -> String {
    let best = scores.iter().max_by_key(|s| s.score);
    let weakest = scores.iter().min_by_key(|s| s.score);
    match (best, weakest) {
        (Some(b), Some(w)) if b.skill != w.skill => format!(
            "Your strongest area is {} ({}/100). Consider investing more time in {} \
             ({}/100) to grow more evenly. Keep shipping!",
            b.skill, b.score, w.skill, w.score
        ),
        (Some(b), _) => {
            format!("Your strongest area is {} ({}/100). Keep building!", b.skill, b.score)
        }
        _ => "No capability data available yet.".to_string(),
    }
}

// ── Score computation (pure Rust, offline) ────────────────────────────────────

fn compute_raw_scores(signals: &[WorkflowSignal], tasks: &[Task]) -> Vec<(String, i64, String)> {
    let now_ms = Utc::now().timestamp_millis();
    let thirty_days_ago = now_ms - 30 * 24 * 60 * 60 * 1000;
    let fourteen_days_ago = now_ms - 14 * 24 * 60 * 60 * 1000;

    let recent: Vec<&WorkflowSignal> =
        signals.iter().filter(|s| s.created_at >= thirty_days_ago).collect();

    // Debugging: debug sessions in last 30 days (10pts each, cap 100)
    let debug_count = recent.iter().filter(|s| s.signal_type == "debug_started").count();
    let debugging = (debug_count as i64 * 10).min(100);

    // Execution: task completion ratio
    let total_tasks = tasks.len();
    let done_tasks = tasks.iter().filter(|t| t.status == "done").count();
    let execution = if total_tasks > 0 {
        ((done_tasks as i64 * 100) / total_tasks as i64).min(100)
    } else {
        50 // neutral when no tasks exist yet
    };

    // Consistency: unique active days in last 14 days (max 14 = 100)
    let active_days_14: HashSet<String> = signals
        .iter()
        .filter(|s| s.created_at >= fourteen_days_ago)
        .filter_map(|s| DateTime::<Utc>::from_timestamp_millis(s.created_at))
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .collect();
    let consistency = ((active_days_14.len() as i64 * 100) / 14).min(100);

    // Breadth: unique programming languages in last 30 days (15pts each, cap 100)
    let unique_langs: HashSet<String> = recent
        .iter()
        .filter_map(|s| s.language.clone())
        .filter(|l| !l.is_empty())
        .collect();
    let breadth = (unique_langs.len() as i64 * 15).min(100);

    // Productivity: file saves per active day in last 30 days
    let saves = recent.iter().filter(|s| s.signal_type == "file_saved").count();
    let active_days_30: HashSet<String> = recent
        .iter()
        .filter_map(|s| DateTime::<Utc>::from_timestamp_millis(s.created_at))
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .collect();
    let days_count = active_days_30.len().max(1);
    let saves_per_day = saves / days_count;
    let productivity = (saves_per_day as i64 * 10).min(100);

    vec![
        (
            "debugging".to_string(),
            debugging,
            format!("{debug_count} debug sessions (30d)"),
        ),
        (
            "execution".to_string(),
            execution,
            format!("{done_tasks}/{total_tasks} tasks done"),
        ),
        (
            "consistency".to_string(),
            consistency,
            format!("{}/14 active days", active_days_14.len()),
        ),
        (
            "breadth".to_string(),
            breadth,
            format!("{} languages", unique_langs.len()),
        ),
        (
            "productivity".to_string(),
            productivity,
            format!("{saves_per_day} saves/active day"),
        ),
    ]
}
