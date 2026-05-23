import { useEffect, useState } from "react";

const DAEMON_URL = "http://127.0.0.1:45678";

interface HealthStatus {
  status: string;
  version: string;
  daemon_ready: boolean;
}

export default function App() {
  const [health, setHealth] = useState<HealthStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch(`${DAEMON_URL}/api/v1/health`)
      .then((r) => r.json())
      .then(setHealth)
      .catch((e) => setError(e.message));
  }, []);

  return (
    <main style={{ fontFamily: "monospace", padding: "2rem" }}>
      <h1>Cognition OS — Phase 1</h1>
      {error && <p style={{ color: "red" }}>Daemon unreachable: {error}</p>}
      {health && (
        <pre>{JSON.stringify(health, null, 2)}</pre>
      )}
      {!health && !error && <p>Connecting to daemon…</p>}
    </main>
  );
}
