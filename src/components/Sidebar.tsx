import type { NavState } from "../App";
import type { Project } from "../api";

interface Props {
  nav: NavState;
  projects: Project[];
  onNav: (nav: NavState) => void;
}

export function Sidebar({ nav, projects, onNav }: Props) {
  const activeProjectId =
    nav.page === "project-detail" ? nav.projectId : undefined;

  return (
    <nav className="sidebar">
      <div className="sidebar-section">
        <div className="sidebar-label">Navigation</div>
        <button
          className={`sidebar-item ${nav.page === "projects" || nav.page === "project-detail" ? "active" : ""}`}
          onClick={() => onNav({ page: "projects" })}
        >
          <span className="item-icon">📁</span>
          <span className="item-label">Projects</span>
          {projects.length > 0 && (
            <span className="item-badge">{projects.length}</span>
          )}
        </button>
        <button
          className={`sidebar-item ${nav.page === "mentor" ? "active" : ""}`}
          onClick={() => onNav({ page: "mentor" })}
        >
          <span className="item-icon">💬</span>
          <span className="item-label">Mentor</span>
        </button>
        <button
          className={`sidebar-item ${nav.page === "audit" ? "active" : ""}`}
          onClick={() => onNav({ page: "audit" })}
        >
          <span className="item-icon">📋</span>
          <span className="item-label">Audit Log</span>
        </button>
        <button
          className={`sidebar-item ${nav.page === "capability" ? "active" : ""}`}
          onClick={() => onNav({ page: "capability" })}
        >
          <span className="item-icon">📈</span>
          <span className="item-label">Capability</span>
        </button>
        <button
          className={`sidebar-item ${nav.page === "artifacts" ? "active" : ""}`}
          onClick={() => onNav({ page: "artifacts" })}
        >
          <span className="item-icon">🗂️</span>
          <span className="item-label">Artifacts</span>
        </button>
        <button
          className={`sidebar-item ${nav.page === "trust" ? "active" : ""}`}
          onClick={() => onNav({ page: "trust" })}
        >
          <span className="item-icon">🔒</span>
          <span className="item-label">Trust</span>
        </button>
        <button
          className={`sidebar-item ${nav.page === "sync" ? "active" : ""}`}
          onClick={() => onNav({ page: "sync" })}
        >
          <span className="item-icon">⇅</span>
          <span className="item-label">Sync</span>
        </button>
      </div>

      {projects.length > 0 && (
        <>
          <div className="sidebar-divider" />
          <div className="sidebar-section">
            <div className="sidebar-label">Projects</div>
            {projects.map((p) => (
              <button
                key={p.id}
                className={`sidebar-item ${activeProjectId === p.id ? "active" : ""}`}
                onClick={() => onNav({ page: "project-detail", projectId: p.id })}
              >
                <span
                  className="item-icon"
                  style={{ fontSize: 9, color: statusColor(p.status) }}
                >
                  ●
                </span>
                <span className="item-label">{p.name}</span>
              </button>
            ))}
          </div>
        </>
      )}
    </nav>
  );
}

function statusColor(status: Project["status"]): string {
  switch (status) {
    case "active":
      return "var(--green)";
    case "paused":
      return "var(--yellow)";
    case "completed":
      return "var(--blue)";
    default:
      return "var(--text-muted)";
  }
}
