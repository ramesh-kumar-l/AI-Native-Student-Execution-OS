import { useState } from "react";
import { createProject, deleteProject, updateProject, type Project } from "../api";

interface Props {
  projects: Project[];
  onRefresh: () => void;
  onOpenProject: (id: string) => void;
  onOpenChat: (projectId: string) => void;
}

interface NewProjectForm {
  name: string;
  description: string;
}

export function ProjectsPage({ projects, onRefresh, onOpenProject, onOpenChat }: Props) {
  const [showModal, setShowModal] = useState(false);
  const [form, setForm] = useState<NewProjectForm>({ name: "", description: "" });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    if (!form.name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await createProject(form.name.trim(), form.description.trim() || undefined);
      setForm({ name: "", description: "" });
      setShowModal(false);
      onRefresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create project");
    } finally {
      setSaving(false);
    }
  }

  async function handleArchive(e: React.MouseEvent, id: string) {
    e.stopPropagation();
    try {
      await updateProject(id, { status: "archived" });
      onRefresh();
    } catch {}
  }

  async function handleDelete(e: React.MouseEvent, id: string) {
    e.stopPropagation();
    if (!confirm("Delete this project and all its tasks?")) return;
    try {
      await deleteProject(id);
      onRefresh();
    } catch {}
  }

  return (
    <div className="page">
      <div className="page-header">
        <div>
          <div className="page-title">Projects</div>
          <div className="page-subtitle">
            {projects.length === 0
              ? "No projects yet — create one to get started"
              : `${projects.length} active project${projects.length !== 1 ? "s" : ""}`}
          </div>
        </div>
        <button className="btn btn-primary" onClick={() => setShowModal(true)}>
          + New Project
        </button>
      </div>

      {projects.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">📁</div>
          <div className="empty-state-title">No projects yet</div>
          <div className="empty-state-sub">
            Create a project to organize your tasks and conversations
          </div>
        </div>
      ) : (
        <div className="project-grid">
          {projects.map((p) => (
            <ProjectCard
              key={p.id}
              project={p}
              onClick={() => onOpenProject(p.id)}
              onOpenChat={(e) => {
                e.stopPropagation();
                onOpenChat(p.id);
              }}
              onArchive={(e) => handleArchive(e, p.id)}
              onDelete={(e) => handleDelete(e, p.id)}
            />
          ))}
        </div>
      )}

      {showModal && (
        <div className="modal-backdrop" onClick={() => setShowModal(false)}>
          <form
            className="modal"
            onSubmit={handleCreate}
            onClick={(e) => e.stopPropagation()}
          >
            <div className="modal-title">New Project</div>
            <div className="form-field">
              <label className="form-label">Name *</label>
              <input
                autoFocus
                value={form.name}
                onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))}
                placeholder="My awesome project"
                maxLength={120}
              />
            </div>
            <div className="form-field">
              <label className="form-label">Description</label>
              <textarea
                value={form.description}
                onChange={(e) =>
                  setForm((f) => ({ ...f, description: e.target.value }))
                }
                placeholder="What are you building?"
                rows={3}
              />
            </div>
            {error && (
              <p style={{ color: "var(--red)", fontSize: 12, marginTop: -8 }}>{error}</p>
            )}
            <div className="modal-actions">
              <button
                type="button"
                className="btn btn-ghost"
                onClick={() => setShowModal(false)}
              >
                Cancel
              </button>
              <button
                type="submit"
                className="btn btn-primary"
                disabled={saving || !form.name.trim()}
              >
                {saving ? "Creating…" : "Create"}
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
}

interface CardProps {
  project: Project;
  onClick: () => void;
  onOpenChat: (e: React.MouseEvent) => void;
  onArchive: (e: React.MouseEvent) => void;
  onDelete: (e: React.MouseEvent) => void;
}

function ProjectCard({ project, onClick, onOpenChat, onArchive, onDelete }: CardProps) {
  const badgeClass = `status-badge status-badge-${project.status}`;

  return (
    <div className="project-card" onClick={onClick}>
      <div className="project-card-name">{project.name}</div>
      {project.description && (
        <div className="project-card-desc">{project.description}</div>
      )}
      <div className="project-card-footer">
        <span className={badgeClass}>{project.status}</span>
        <span style={{ flex: 1 }} />
        <button
          className="btn-icon"
          title="Open Mentor Chat"
          onClick={onOpenChat}
          style={{ fontSize: 12 }}
        >
          💬
        </button>
        {project.status !== "archived" && (
          <button
            className="btn-icon"
            title="Archive"
            onClick={onArchive}
            style={{ fontSize: 12 }}
          >
            📦
          </button>
        )}
        <button
          className="btn-icon"
          title="Delete"
          onClick={onDelete}
          style={{ fontSize: 12, color: "var(--red)" }}
        >
          ✕
        </button>
      </div>
    </div>
  );
}
