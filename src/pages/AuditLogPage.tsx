import { useCallback, useEffect, useState } from "react";
import { getAuditRecent, type AuditEntry } from "../api";

export function AuditLogPage() {
  const [entries, setEntries] = useState<AuditEntry[]>([]);
  const [limit, setLimit] = useState(50);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await getAuditRecent(limit);
      setEntries(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load audit log");
    } finally {
      setLoading(false);
    }
  }, [limit]);

  useEffect(() => {
    load();
  }, [load]);

  return (
    <div className="page">
      <div className="page-header">
        <div>
          <div className="page-title">Audit Log</div>
          <div className="page-subtitle">
            Every AI call, memory write, and system event recorded
          </div>
        </div>
        <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
          <select
            value={limit}
            onChange={(e) => setLimit(Number(e.target.value))}
          >
            <option value={25}>25 entries</option>
            <option value={50}>50 entries</option>
            <option value={100}>100 entries</option>
            <option value={200}>200 entries</option>
          </select>
          <button className="btn btn-ghost" onClick={load} disabled={loading}>
            {loading ? "Loading…" : "Refresh"}
          </button>
        </div>
      </div>

      {error && (
        <p style={{ color: "var(--red)", fontSize: 12, marginBottom: 12 }}>{error}</p>
      )}

      {entries.length === 0 && !loading ? (
        <div className="empty-state">
          <div className="empty-state-icon">📋</div>
          <div className="empty-state-title">No audit entries yet</div>
          <div className="empty-state-sub">
            Events are recorded when you interact with the mentor
          </div>
        </div>
      ) : (
        <table className="audit-table">
          <thead>
            <tr>
              <th>Event</th>
              <th>Correlation ID</th>
              <th>Time</th>
              <th>Metadata</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((e) => (
              <AuditRow key={e.id} entry={e} />
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

function AuditRow({ entry }: { entry: AuditEntry }) {
  const [expanded, setExpanded] = useState(false);
  const ts = new Date(entry.created_at).toLocaleTimeString();

  return (
    <tr
      title={entry.id}
      style={{ cursor: entry.metadata ? "pointer" : "default" }}
      onClick={() => entry.metadata && setExpanded((e) => !e)}
    >
      <td>
        <span className="audit-event-type">{entry.event_type}</span>
      </td>
      <td>{entry.correlation_id.slice(0, 16)}…</td>
      <td>{ts}</td>
      <td>
        {entry.metadata ? (
          expanded ? (
            <pre
              style={{
                fontSize: 10,
                color: "var(--text-secondary)",
                whiteSpace: "pre-wrap",
                maxWidth: 320,
              }}
            >
              {JSON.stringify(entry.metadata, null, 2)}
            </pre>
          ) : (
            <span style={{ color: "var(--text-muted)" }}>
              {Object.keys(entry.metadata).join(", ")}
            </span>
          )
        ) : (
          <span style={{ color: "var(--text-muted)" }}>—</span>
        )}
      </td>
    </tr>
  );
}
