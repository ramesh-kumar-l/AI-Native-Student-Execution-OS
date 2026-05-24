import { useEffect, useState } from "react";
import {
  getTrustHealth,
  getTrustLineage,
  type HealthReport,
  type LineageEntry,
} from "../api";

function ts(ms: number): string {
  return new Date(ms).toLocaleString();
}

function uptimeStr(s: number): string {
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = s % 60;
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m ${sec}s`;
  return `${sec}s`;
}

export function TrustPage() {
  const [health, setHealth] = useState<HealthReport | null>(null);
  const [lineage, setLineage] = useState<LineageEntry[]>([]);
  const [lineageLimit, setLineageLimit] = useState(50);
  const [loadingHealth, setLoadingHealth] = useState(true);
  const [loadingLineage, setLoadingLineage] = useState(true);
  const [expanded, setExpanded] = useState<string | null>(null);

  async function loadHealth() {
    setLoadingHealth(true);
    try {
      setHealth(await getTrustHealth());
    } finally {
      setLoadingHealth(false);
    }
  }

  async function loadLineage() {
    setLoadingLineage(true);
    try {
      setLineage(await getTrustLineage(lineageLimit));
    } finally {
      setLoadingLineage(false);
    }
  }

  useEffect(() => { loadHealth(); }, []);
  useEffect(() => { loadLineage(); }, [lineageLimit]);

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">Trust &amp; Health</h1>
        <p className="page-subtitle">
          Full visibility into daemon state and data lineage — no hidden operations.
        </p>
      </div>

      {/* Health dashboard */}
      <section style={{ marginBottom: "2rem" }}>
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "1rem" }}>
          <h2 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", margin: 0 }}>
            System Health
          </h2>
          <button className="btn btn-secondary" onClick={loadHealth} disabled={loadingHealth}>
            {loadingHealth ? "Refreshing…" : "Refresh"}
          </button>
        </div>

        {health && (
          <>
            <div style={{ display: "flex", gap: "0.75rem", marginBottom: "1rem", flexWrap: "wrap" }}>
              <StatusChip ok={health.daemon_healthy} label="Daemon" />
              <StatusChip ok={health.db_ok} label="Database" />
              <div className="badge badge-info" style={{ fontSize: "0.7rem", padding: "0.25rem 0.6rem" }}>
                Uptime: {uptimeStr(health.uptime_s)}
              </div>
            </div>

            <div className="card-grid" style={{ gridTemplateColumns: "repeat(auto-fill, minmax(130px, 1fr))", gap: "0.75rem" }}>
              <StatCard label="Projects" value={health.projects} />
              <StatCard label="Tasks" value={health.tasks} />
              <StatCard label="Conversations" value={health.conversations} />
              <StatCard label="Messages" value={health.messages} />
              <StatCard label="Signals" value={health.signals} />
              <StatCard label="Capability Scores" value={health.capability_scores} />
              <StatCard label="Artifacts" value={health.artifacts} />
            </div>
          </>
        )}
        {loadingHealth && !health && <p style={{ color: "var(--text-muted)" }}>Loading health…</p>}
      </section>

      {/* Data lineage */}
      <section>
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "1rem" }}>
          <h2 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", margin: 0 }}>
            Data Lineage
          </h2>
          <div style={{ display: "flex", gap: "0.5rem", alignItems: "center" }}>
            <select
              value={lineageLimit}
              onChange={(e) => setLineageLimit(Number(e.target.value))}
              style={{
                background: "var(--surface-2)",
                border: "1px solid var(--border)",
                color: "var(--text-primary)",
                borderRadius: 6,
                padding: "0.3rem 0.5rem",
                fontSize: "0.8rem",
              }}
            >
              {[25, 50, 100, 200].map((n) => (
                <option key={n} value={n}>{n} events</option>
              ))}
            </select>
            <button className="btn btn-secondary" onClick={loadLineage} disabled={loadingLineage}>
              {loadingLineage ? "…" : "Refresh"}
            </button>
          </div>
        </div>

        {lineage.length === 0 && !loadingLineage && (
          <div className="empty-state">
            <div className="empty-icon">🔍</div>
            <div className="empty-title">No audit events yet</div>
            <div className="empty-body">Events appear here as you use the app.</div>
          </div>
        )}

        {lineage.map((entry) => (
          <div
            key={entry.id}
            style={{
              background: "var(--surface-1)",
              border: "1px solid var(--border)",
              borderRadius: 8,
              padding: "0.75rem 1rem",
              marginBottom: "0.5rem",
            }}
          >
            <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", gap: "1rem" }}>
              <div>
                <span
                  style={{
                    fontFamily: "monospace",
                    fontSize: "0.75rem",
                    background: "var(--surface-2)",
                    padding: "0.15rem 0.4rem",
                    borderRadius: 4,
                    color: "var(--blue)",
                    marginRight: "0.5rem",
                  }}
                >
                  {entry.event_type}
                </span>
                {entry.entity_type && (
                  <span style={{ fontSize: "0.75rem", color: "var(--text-muted)" }}>
                    {entry.entity_type}
                    {entry.entity_id && ` · ${entry.entity_id.slice(0, 8)}…`}
                  </span>
                )}
              </div>
              <span style={{ fontSize: "0.7rem", color: "var(--text-muted)", whiteSpace: "nowrap" }}>
                {ts(entry.created_at)}
              </span>
            </div>
            {entry.metadata && (
              <div style={{ marginTop: "0.5rem" }}>
                <button
                  style={{
                    background: "none",
                    border: "none",
                    color: "var(--text-muted)",
                    fontSize: "0.7rem",
                    cursor: "pointer",
                    padding: 0,
                  }}
                  onClick={() => setExpanded(expanded === entry.id ? null : entry.id)}
                >
                  {expanded === entry.id ? "▾ metadata" : "▸ metadata"}
                </button>
                {expanded === entry.id && (
                  <pre
                    style={{
                      marginTop: "0.4rem",
                      fontSize: "0.7rem",
                      color: "var(--text-muted)",
                      background: "var(--surface-2)",
                      padding: "0.5rem",
                      borderRadius: 4,
                      overflowX: "auto",
                    }}
                  >
                    {JSON.stringify(entry.metadata, null, 2)}
                  </pre>
                )}
              </div>
            )}
          </div>
        ))}
      </section>
    </div>
  );
}

function StatusChip({ ok, label }: { ok: boolean; label: string }) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: "0.35rem",
        background: "var(--surface-1)",
        border: `1px solid ${ok ? "var(--green)" : "var(--red, #f87171)"}`,
        borderRadius: 6,
        padding: "0.25rem 0.6rem",
        fontSize: "0.75rem",
        color: ok ? "var(--green)" : "var(--red, #f87171)",
      }}
    >
      <span>{ok ? "●" : "○"}</span>
      <span>{label}</span>
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: number }) {
  return (
    <div
      style={{
        background: "var(--surface-1)",
        border: "1px solid var(--border)",
        borderRadius: 8,
        padding: "0.75rem",
        textAlign: "center",
      }}
    >
      <div style={{ fontSize: "1.4rem", fontWeight: 700, color: "var(--text-primary)" }}>
        {value}
      </div>
      <div style={{ fontSize: "0.7rem", color: "var(--text-muted)", marginTop: "0.2rem" }}>
        {label}
      </div>
    </div>
  );
}
