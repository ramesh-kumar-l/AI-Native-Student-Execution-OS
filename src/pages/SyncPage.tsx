import { useCallback, useEffect, useRef, useState } from "react";
import {
  getSyncStatus,
  exportData,
  importData,
  type RecordCounts,
  type ImportSummary,
} from "../api";

export function SyncPage() {
  const [counts, setCounts] = useState<RecordCounts | null>(null);
  const [passphrase, setPassphrase] = useState("");
  const [exporting, setExporting] = useState(false);
  const [importing, setImporting] = useState(false);
  const [importResult, setImportResult] = useState<ImportSummary | null>(null);
  const [error, setError] = useState<string | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);

  const loadStatus = useCallback(async () => {
    try {
      setCounts(await getSyncStatus());
    } catch {
      /* daemon not ready — silently skip */
    }
  }, []);

  useEffect(() => { loadStatus(); }, [loadStatus]);

  async function handleExport() {
    setExporting(true);
    setError(null);
    try {
      const payload = await exportData(passphrase || undefined);
      const json = JSON.stringify(payload, null, 2);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `cognition-os-export-${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Export failed");
    } finally {
      setExporting(false);
    }
  }

  async function handleFileImport(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    setImporting(true);
    setError(null);
    setImportResult(null);
    try {
      const text = await file.text();
      const payload = JSON.parse(text);
      const summary = await importData(payload, passphrase || undefined);
      setImportResult(summary);
      await loadStatus();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Import failed");
    } finally {
      setImporting(false);
      if (fileRef.current) fileRef.current.value = "";
    }
  }

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">Sync &amp; Export</h1>
        <p className="page-subtitle">
          Export all your data as an encrypted JSON file. Import on any device to sync.
          Your data never leaves your machine without your explicit action.
        </p>
      </div>

      {/* Record counts */}
      {counts && (
        <div
          style={{
            background: "var(--surface-1)",
            border: "1px solid var(--border)",
            borderRadius: 10,
            padding: "1rem 1.25rem",
            marginBottom: "1.5rem",
          }}
        >
          <h2 style={{ fontSize: "0.9rem", fontWeight: 600, color: "var(--text-primary)", margin: "0 0 0.75rem" }}>
            Local Data
          </h2>
          <div style={{ display: "flex", gap: "1.5rem", flexWrap: "wrap" }}>
            <CountPill label="Projects" value={counts.projects} />
            <CountPill label="Tasks" value={counts.tasks} />
            <CountPill label="Conversations" value={counts.conversations} />
            <CountPill label="Messages" value={counts.messages} />
            <CountPill label="Scores" value={counts.capability_scores} />
            <CountPill label="Artifacts" value={counts.artifacts} />
          </div>
        </div>
      )}

      {/* Passphrase */}
      <div
        style={{
          background: "var(--surface-1)",
          border: "1px solid var(--border)",
          borderRadius: 10,
          padding: "1.25rem",
          marginBottom: "1.5rem",
        }}
      >
        <h2 style={{ fontSize: "0.9rem", fontWeight: 600, color: "var(--text-primary)", margin: "0 0 0.5rem" }}>
          Encryption Passphrase (optional)
        </h2>
        <p style={{ fontSize: "0.8rem", color: "var(--text-muted)", marginBottom: "0.75rem" }}>
          If set, the export is AES-256-GCM encrypted with an Argon2id-derived key.
          Leave blank for a plaintext JSON export.
        </p>
        <input
          type="password"
          value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          placeholder="Passphrase (leave blank for unencrypted)"
          style={{
            width: "100%",
            maxWidth: 400,
            background: "var(--surface-2)",
            border: "1px solid var(--border)",
            borderRadius: 6,
            padding: "0.5rem 0.75rem",
            color: "var(--text-primary)",
            fontSize: "0.875rem",
            boxSizing: "border-box",
          }}
        />
      </div>

      {/* Export */}
      <div
        style={{
          background: "var(--surface-1)",
          border: "1px solid var(--border)",
          borderRadius: 10,
          padding: "1.25rem",
          marginBottom: "1.5rem",
        }}
      >
        <h2 style={{ fontSize: "0.9rem", fontWeight: 600, color: "var(--text-primary)", margin: "0 0 0.4rem" }}>
          Export
        </h2>
        <p style={{ fontSize: "0.8rem", color: "var(--text-muted)", marginBottom: "0.75rem" }}>
          Downloads all projects, tasks, conversations, messages, capability scores, and artifacts.
        </p>
        <button
          className="btn btn-primary"
          onClick={handleExport}
          disabled={exporting}
        >
          {exporting ? "Exporting…" : passphrase ? "Export (encrypted)" : "Export"}
        </button>
      </div>

      {/* Import */}
      <div
        style={{
          background: "var(--surface-1)",
          border: "1px solid var(--border)",
          borderRadius: 10,
          padding: "1.25rem",
          marginBottom: "1.5rem",
        }}
      >
        <h2 style={{ fontSize: "0.9rem", fontWeight: 600, color: "var(--text-primary)", margin: "0 0 0.4rem" }}>
          Import
        </h2>
        <p style={{ fontSize: "0.8rem", color: "var(--text-muted)", marginBottom: "0.75rem" }}>
          Import a previously exported file. Existing records with the same ID are replaced.
          If the file is encrypted, enter the same passphrase above.
        </p>
        <label
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "0.5rem",
            background: "var(--surface-2)",
            border: "1px solid var(--border)",
            borderRadius: 6,
            padding: "0.45rem 0.9rem",
            cursor: importing ? "not-allowed" : "pointer",
            fontSize: "0.85rem",
            color: "var(--text-secondary)",
          }}
        >
          <span>{importing ? "Importing…" : "Choose file…"}</span>
          <input
            ref={fileRef}
            type="file"
            accept=".json"
            style={{ display: "none" }}
            disabled={importing}
            onChange={handleFileImport}
          />
        </label>

        {importResult && (
          <div
            style={{
              marginTop: "0.75rem",
              background: "var(--surface-2)",
              border: "1px solid var(--green)",
              borderRadius: 8,
              padding: "0.75rem 1rem",
              fontSize: "0.8rem",
              color: "var(--green)",
            }}
          >
            Import complete: {importResult.projects} projects, {importResult.tasks} tasks,{" "}
            {importResult.conversations} conversations, {importResult.messages} messages,{" "}
            {importResult.capability_scores} scores, {importResult.artifacts} artifacts
          </div>
        )}
      </div>

      {error && (
        <div
          style={{
            background: "var(--surface-1)",
            border: "1px solid var(--red, #f87171)",
            borderRadius: 8,
            padding: "0.75rem 1rem",
            color: "var(--red, #f87171)",
            fontSize: "0.85rem",
          }}
        >
          {error}
        </div>
      )}

      {/* Security note */}
      <div
        style={{
          marginTop: "2rem",
          padding: "1rem 1.25rem",
          background: "var(--surface-1)",
          border: "1px solid var(--border)",
          borderRadius: 10,
          fontSize: "0.78rem",
          color: "var(--text-muted)",
          lineHeight: 1.6,
        }}
      >
        <strong style={{ color: "var(--text-secondary)" }}>Local-first guarantee</strong>
        <br />
        All data is stored exclusively on this device. Exports go directly to your file system.
        No cloud relay, no telemetry. Encrypted exports use AES-256-GCM with Argon2id key derivation —
        the passphrase never leaves this machine.
      </div>
    </div>
  );
}

function CountPill({ label, value }: { label: string; value: number }) {
  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
      <span style={{ fontWeight: 700, fontSize: "1.1rem", color: "var(--text-primary)" }}>{value}</span>
      <span style={{ fontSize: "0.7rem", color: "var(--text-muted)" }}>{label}</span>
    </div>
  );
}
