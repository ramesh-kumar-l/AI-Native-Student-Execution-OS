import { useEffect, useState } from "react";
import { getHealth, type HealthStatus } from "../api";

type DaemonState = "connecting" | "online" | "offline";

export function StatusBanner() {
  const [daemonState, setDaemonState] = useState<DaemonState>("connecting");
  const [health, setHealth] = useState<HealthStatus | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function check() {
      try {
        const h = await getHealth();
        if (!cancelled) {
          setHealth(h);
          setDaemonState(h.daemon_ready ? "online" : "offline");
        }
      } catch {
        if (!cancelled) {
          setHealth(null);
          setDaemonState("offline");
        }
      }
    }

    check();
    const interval = setInterval(check, 30_000);
    return () => {
      cancelled = true;
      clearInterval(interval);
    };
  }, []);

  const dotClass =
    daemonState === "online"
      ? "dot dot-green"
      : daemonState === "connecting"
        ? "dot dot-yellow"
        : "dot dot-red";

  const label =
    daemonState === "online"
      ? "Daemon online"
      : daemonState === "connecting"
        ? "Connecting…"
        : "Daemon offline";

  return (
    <div className="status-banner">
      <span className="app-name">Cognition OS</span>
      <div className="status-tag">
        <span className={dotClass} />
        <span>{label}</span>
      </div>
      {health && (
        <div className="status-tag" style={{ color: "var(--text-muted)", fontSize: 11 }}>
          v{health.version}
        </div>
      )}
      {daemonState === "offline" && (
        <div className="status-tag" style={{ color: "var(--red)", fontSize: 11 }}>
          AI features unavailable — start the daemon to continue
        </div>
      )}
      <div className="spacer" />
    </div>
  );
}
