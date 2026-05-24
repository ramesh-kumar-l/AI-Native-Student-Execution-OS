import { useCallback, useEffect, useState } from "react";
import {
  computeCapabilityScores,
  getCapabilityNarrative,
  listCapabilityScores,
  type CapabilityScore,
} from "../api";

interface Props {
  projectId?: string;
}

const SKILL_LABELS: Record<string, string> = {
  debugging: "Debugging",
  execution: "Execution",
  consistency: "Consistency",
  breadth: "Breadth",
  productivity: "Productivity",
};

const SKILL_DESCRIPTIONS: Record<string, string> = {
  debugging: "Debug sessions run",
  execution: "Task completion rate",
  consistency: "Active days (14d window)",
  breadth: "Languages used",
  productivity: "File saves per active day",
};

function scoreColor(score: number): string {
  if (score >= 70) return "var(--green)";
  if (score >= 40) return "var(--yellow)";
  return "var(--text-muted)";
}

export function CapabilityPage({ projectId }: Props) {
  const [scores, setScores] = useState<CapabilityScore[]>([]);
  const [narrative, setNarrative] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [computing, setComputing] = useState(false);
  const [narrativeLoading, setNarrativeLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastComputed, setLastComputed] = useState<number | null>(null);

  const loadScores = useCallback(async () => {
    try {
      const data = await listCapabilityScores(projectId);
      setScores(data);
      if (data.length > 0) {
        setLastComputed(Math.max(...data.map((s) => s.computed_at)));
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load scores");
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  const loadNarrative = useCallback(async () => {
    setNarrativeLoading(true);
    try {
      const data = await getCapabilityNarrative(projectId);
      setNarrative(data.narrative);
    } catch {
      setNarrative("");
    } finally {
      setNarrativeLoading(false);
    }
  }, [projectId]);

  useEffect(() => {
    loadScores();
    loadNarrative();
  }, [loadScores, loadNarrative]);

  const handleCompute = async () => {
    setComputing(true);
    setError(null);
    try {
      const data = await computeCapabilityScores(projectId);
      setScores(data);
      if (data.length > 0) {
        setLastComputed(Math.max(...data.map((s) => s.computed_at)));
      }
      await loadNarrative();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Compute failed");
    } finally {
      setComputing(false);
    }
  };

  if (loading) {
    return <div className="page-loading">Loading capability data…</div>;
  }

  return (
    <div className="page">
      <div className="page-header">
        <div>
          <h1 className="page-title">Capability Profile</h1>
          <p className="page-subtitle">
            {projectId ? "Project skill scores" : "Cross-project skill scores"} · computed from
            your workflow activity
          </p>
        </div>
        <button
          className="btn btn-primary"
          onClick={handleCompute}
          disabled={computing}
        >
          {computing ? "Computing…" : "Recompute Scores"}
        </button>
      </div>

      {error && <div className="error-banner">{error}</div>}

      {lastComputed && (
        <p className="page-meta">
          Last computed: {new Date(lastComputed).toLocaleString()}
        </p>
      )}

      {scores.length === 0 ? (
        <div className="empty-state">
          <div className="empty-icon">📊</div>
          <div className="empty-title">No capability data yet</div>
          <div className="empty-body">
            Write some code, run debug sessions, and complete tasks — then click
            "Recompute Scores" to build your profile.
          </div>
        </div>
      ) : (
        <div className="capability-grid">
          {scores.map((score) => (
            <div key={score.id} className="capability-card">
              <div className="cap-header">
                <span className="cap-skill">
                  {SKILL_LABELS[score.skill] ?? score.skill}
                </span>
                <span className="cap-score" style={{ color: scoreColor(score.score) }}>
                  {score.score}/100
                </span>
              </div>
              <div className="cap-bar-track">
                <div
                  className="cap-bar-fill"
                  style={{
                    width: `${score.score}%`,
                    backgroundColor: scoreColor(score.score),
                  }}
                />
              </div>
              <div className="cap-basis">
                {SKILL_DESCRIPTIONS[score.skill] ?? ""}
              </div>
            </div>
          ))}
        </div>
      )}

      {scores.length > 0 && (
        <div className="narrative-section">
          <h2 className="section-title">AI Narrative</h2>
          {narrativeLoading ? (
            <div className="narrative-loading">Generating narrative…</div>
          ) : narrative ? (
            <p className="narrative-text">{narrative}</p>
          ) : (
            <p className="narrative-empty">
              Narrative unavailable — ensure Ollama is running or configure a cloud provider.
            </p>
          )}
        </div>
      )}
    </div>
  );
}
