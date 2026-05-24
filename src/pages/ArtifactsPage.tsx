import { useCallback, useEffect, useState } from "react";
import {
  deleteArtifact,
  generateArtifact,
  listArtifacts,
  listProjects,
  type Artifact,
  type Project,
} from "../api";

const TYPE_LABELS: Record<string, string> = {
  project_page: "Project Page",
  portfolio_export: "Portfolio Export",
  capability_narrative: "Capability Narrative",
};

function formatDate(ms: number): string {
  return new Date(ms).toLocaleString();
}

function downloadMarkdown(artifact: Artifact) {
  const blob = new Blob([artifact.content], { type: "text/markdown" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${artifact.title.replace(/[^a-z0-9]/gi, "_")}.md`;
  a.click();
  URL.revokeObjectURL(url);
}

export function ArtifactsPage() {
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [selected, setSelected] = useState<Artifact | null>(null);
  const [loading, setLoading] = useState(true);
  const [generating, setGenerating] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    try {
      const [arts, projs] = await Promise.all([listArtifacts(), listProjects()]);
      setArtifacts(arts);
      setProjects(projs);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load artifacts");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  const handleGenerate = async (
    type: "project_page" | "portfolio_export",
    projectId?: string
  ) => {
    const key = `${type}:${projectId ?? "all"}`;
    setGenerating(key);
    setError(null);
    try {
      const artifact = await generateArtifact(type, projectId);
      setArtifacts((prev) => [artifact, ...prev]);
      setSelected(artifact);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Generation failed");
    } finally {
      setGenerating(null);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await deleteArtifact(id);
      setArtifacts((prev) => prev.filter((a) => a.id !== id));
      if (selected?.id === id) setSelected(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Delete failed");
    }
  };

  if (loading) {
    return <div className="page-loading">Loading artifacts…</div>;
  }

  return (
    <div className="page artifacts-layout">
      <div className="artifacts-sidebar">
        <div className="page-header" style={{ flexDirection: "column", alignItems: "stretch" }}>
          <h1 className="page-title">Artifacts</h1>
          <p className="page-subtitle">Portfolio outputs and project pages</p>
        </div>

        {error && <div className="error-banner">{error}</div>}

        <div className="generate-section">
          <div className="section-title" style={{ marginBottom: 8 }}>Generate</div>
          <button
            className="btn btn-secondary generate-btn"
            onClick={() => handleGenerate("portfolio_export")}
            disabled={generating !== null}
          >
            {generating === "portfolio_export:all" ? "Generating…" : "Portfolio Export (all projects)"}
          </button>
          {projects.map((p) => (
            <button
              key={p.id}
              className="btn btn-secondary generate-btn"
              onClick={() => handleGenerate("project_page", p.id)}
              disabled={generating !== null}
            >
              {generating === `project_page:${p.id}`
                ? "Generating…"
                : `Project Page — ${p.name}`}
            </button>
          ))}
        </div>

        <div className="artifacts-list">
          {artifacts.length === 0 ? (
            <div className="empty-state" style={{ padding: "24px 0" }}>
              <div className="empty-title">No artifacts yet</div>
              <div className="empty-body">Generate a project page or portfolio export above.</div>
            </div>
          ) : (
            artifacts.map((a) => (
              <button
                key={a.id}
                className={`artifact-item ${selected?.id === a.id ? "active" : ""}`}
                onClick={() => setSelected(a)}
              >
                <div className="artifact-type-badge">
                  {TYPE_LABELS[a.artifact_type] ?? a.artifact_type}
                </div>
                <div className="artifact-title">{a.title}</div>
                <div className="artifact-date">{formatDate(a.created_at)}</div>
              </button>
            ))
          )}
        </div>
      </div>

      <div className="artifact-preview">
        {selected ? (
          <>
            <div className="preview-header">
              <div>
                <div className="preview-title">{selected.title}</div>
                <div className="preview-meta">
                  {TYPE_LABELS[selected.artifact_type] ?? selected.artifact_type} ·{" "}
                  {formatDate(selected.created_at)}
                </div>
              </div>
              <div className="preview-actions">
                <button
                  className="btn btn-secondary"
                  onClick={() => downloadMarkdown(selected)}
                >
                  Download .md
                </button>
                <button
                  className="btn btn-danger"
                  onClick={() => handleDelete(selected.id)}
                >
                  Delete
                </button>
              </div>
            </div>
            <pre className="preview-content">{selected.content}</pre>
          </>
        ) : (
          <div className="empty-state">
            <div className="empty-icon">📄</div>
            <div className="empty-title">Select an artifact to preview</div>
            <div className="empty-body">
              Or generate a new one using the buttons on the left.
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
