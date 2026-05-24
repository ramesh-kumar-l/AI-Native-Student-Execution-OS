/// Artifact Engine — generates portfolio-quality outputs from project data.
/// Project pages and portfolio exports are generated via AI with an offline-first
/// template fallback so the feature works without any AI provider.
use chrono::Utc;
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
use crate::memory::l1::Artifact;
use crate::memory::MemoryStore;
use crate::observability::audit::AuditLog;

pub struct ArtifactEngine {
    memory: Arc<MemoryStore>,
    orchestrator: Arc<Orchestrator>,
    _audit: Arc<AuditLog>,
}

impl ArtifactEngine {
    pub fn new(
        memory: Arc<MemoryStore>,
        orchestrator: Arc<Orchestrator>,
        audit: Arc<AuditLog>,
    ) -> Self {
        Self { memory, orchestrator, _audit: audit }
    }

    /// Generate a portfolio-quality project page as a markdown artifact.
    pub async fn generate_project_page(
        &self,
        project_id: &str,
        correlation_id: &str,
    ) -> Result<Artifact, DaemonError> {
        let project = self
            .memory
            .l1
            .get_project(project_id)
            .await?
            .ok_or_else(|| DaemonError::Internal("project not found".into()))?;

        let tasks = self.memory.l1.list_tasks_for_project(project_id).await?;
        let signals = self.memory.l1.recent_signals(Some(project_id), 100).await?;

        let done = tasks.iter().filter(|t| t.status == "done").count();
        let in_progress = tasks.iter().filter(|t| t.status == "in_progress").count();
        let todo = tasks.iter().filter(|t| t.status == "todo").count();

        let langs: HashSet<String> = signals
            .iter()
            .filter_map(|s| s.language.clone())
            .filter(|l| !l.is_empty())
            .collect();
        let mut lang_list: Vec<String> = langs.into_iter().collect();
        lang_list.sort();
        let langs_str = if lang_list.is_empty() {
            "Not detected".to_string()
        } else {
            lang_list.join(", ")
        };

        let desc = project.description.as_deref().unwrap_or("No description provided");
        let prompt = format!(
            "Generate a concise professional portfolio project page in markdown.\n\n\
             Project: {name}\nDescription: {desc}\nStatus: {status}\n\
             Tasks: {done} done, {in_progress} in progress, {todo} todo\n\
             Technologies: {langs}\nActivity: {sig_count} workflow events captured\n\n\
             Write these sections: ## Overview, ## What I Built, ## Technologies, \
             ## Progress, ## Key Learnings. Professional tone, under 250 words.",
            name = project.name,
            desc = desc,
            status = project.status,
            done = done,
            in_progress = in_progress,
            todo = todo,
            langs = langs_str,
            sig_count = signals.len(),
        );

        let fallback = || {
            template_project_page(
                &project.name,
                project.description.as_deref(),
                &project.status,
                done,
                in_progress,
                todo,
                &langs_str,
            )
        };

        let content = self
            .call_ai_or_template(&prompt, TaskKind::ArtifactDraft, project_id, correlation_id, fallback)
            .await;

        let artifact = self
            .memory
            .l1
            .create_artifact(
                Some(project_id.to_string()),
                "project_page".to_string(),
                format!("{} — Project Page", project.name),
                content,
                "markdown".to_string(),
            )
            .await?;

        self.memory
            .l0
            .append_typed(
                &EventKind::ArtifactGenerated {
                    artifact_id: artifact.id.clone(),
                    artifact_type: "project_page".to_string(),
                    project_id: Some(project_id.to_string()),
                },
                correlation_id,
                "local",
            )
            .await
            .ok();

        info!(artifact_id = %artifact.id, project_id, "project page artifact generated");
        Ok(artifact)
    }

    /// Generate a portfolio export summarizing one or all projects.
    pub async fn generate_portfolio_export(
        &self,
        project_id: Option<&str>,
        correlation_id: &str,
    ) -> Result<Artifact, DaemonError> {
        let projects = if let Some(pid) = project_id {
            vec![self
                .memory
                .l1
                .get_project(pid)
                .await?
                .ok_or_else(|| DaemonError::Internal("project not found".into()))?]
        } else {
            self.memory.l1.list_projects().await?
        };

        let mut sections = Vec::new();
        for project in &projects {
            let tasks = self.memory.l1.list_tasks_for_project(&project.id).await?;
            let done = tasks.iter().filter(|t| t.status == "done").count();
            sections.push(format!(
                "### {}\n**Status:** {} | **Completed Tasks:** {}/{}\n\n{}\n",
                project.name,
                project.status,
                done,
                tasks.len(),
                project.description.as_deref().unwrap_or(""),
            ));
        }

        let content = format!(
            "# Portfolio Export\n\n*Generated {}*\n\n## Projects\n\n{}",
            Utc::now().format("%Y-%m-%d"),
            sections.join("\n---\n\n"),
        );

        let title = if projects.len() == 1 {
            format!("Portfolio — {}", projects[0].name)
        } else {
            format!("Portfolio Export — {} Projects", projects.len())
        };

        let artifact = self
            .memory
            .l1
            .create_artifact(
                project_id.map(str::to_string),
                "portfolio_export".to_string(),
                title,
                content,
                "markdown".to_string(),
            )
            .await?;

        self.memory
            .l0
            .append_typed(
                &EventKind::ArtifactGenerated {
                    artifact_id: artifact.id.clone(),
                    artifact_type: "portfolio_export".to_string(),
                    project_id: project_id.map(str::to_string),
                },
                correlation_id,
                "local",
            )
            .await
            .ok();

        info!(artifact_id = %artifact.id, "portfolio export artifact generated");
        Ok(artifact)
    }

    /// Calls the AI orchestrator; on any failure returns the offline template instead.
    async fn call_ai_or_template(
        &self,
        prompt: &str,
        task_kind: TaskKind,
        project_id: &str,
        correlation_id: &str,
        fallback: impl FnOnce() -> String,
    ) -> String {
        let task = TaskSpec {
            task_kind,
            inputs: TaskInputs {
                messages: vec![ChatMessage {
                    role: MessageRole::User,
                    content: prompt.to_string(),
                }],
                conversation_id: None,
                project_id: Some(project_id.to_string()),
                context: vec![],
            },
            routing_policy: RoutingPolicy::default(),
            determinism: DeterminismPolicy { temperature: 0.4, ..Default::default() },
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
                            error!(err = %e, "artifact AI stream error");
                            break;
                        }
                    }
                }
                let result = content.trim().to_string();
                if result.is_empty() {
                    fallback()
                } else {
                    result
                }
            }
            Err(e) => {
                error!(err = %e, "AI unavailable, using template artifact");
                fallback()
            }
        }
    }
}

fn template_project_page(
    name: &str,
    description: Option<&str>,
    status: &str,
    done: usize,
    in_progress: usize,
    todo: usize,
    langs: &str,
) -> String {
    format!(
        "# {name}\n\n\
         ## Overview\n{desc}\n\n\
         **Status:** {status}\n\n\
         ## Technologies\n{langs}\n\n\
         ## Progress\n\
         - ✅ {done} tasks completed\n\
         - 🔄 {in_progress} tasks in progress\n\
         - 📋 {todo} tasks todo\n\n\
         ## Key Learnings\n\
         *(Connect Ollama or a cloud provider to generate AI-powered insights)*\n",
        name = name,
        desc = description.unwrap_or("No description provided."),
        status = status,
        langs = if langs == "Not detected" { "N/A" } else { langs },
        done = done,
        in_progress = in_progress,
        todo = todo,
    )
}
