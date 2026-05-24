import { useCallback, useEffect, useState } from "react";
import { StatusBanner } from "./components/StatusBanner";
import { Sidebar } from "./components/Sidebar";
import { BottomNav } from "./components/BottomNav";
import { ProjectsPage } from "./pages/ProjectsPage";
import { ProjectDetailPage } from "./pages/ProjectDetailPage";
import { MentorChatPage } from "./pages/MentorChatPage";
import { AuditLogPage } from "./pages/AuditLogPage";
import { CapabilityPage } from "./pages/CapabilityPage";
import { ArtifactsPage } from "./pages/ArtifactsPage";
import { TrustPage } from "./pages/TrustPage";
import { SyncPage } from "./pages/SyncPage";
import { listProjects, type Project } from "./api";

export type NavState =
  | { page: "projects" }
  | { page: "project-detail"; projectId: string }
  | { page: "mentor"; conversationId?: string; projectId?: string }
  | { page: "audit" }
  | { page: "capability"; projectId?: string }
  | { page: "artifacts" }
  | { page: "trust" }
  | { page: "sync" };

function useIsMobile(): boolean {
  const [mobile, setMobile] = useState(() => window.innerWidth <= 768);
  useEffect(() => {
    const fn = () => setMobile(window.innerWidth <= 768);
    window.addEventListener("resize", fn);
    return () => window.removeEventListener("resize", fn);
  }, []);
  return mobile;
}

export default function App() {
  const [nav, setNav] = useState<NavState>({ page: "projects" });
  const [projects, setProjects] = useState<Project[]>([]);
  const isMobile = useIsMobile();

  const refreshProjects = useCallback(async () => {
    try {
      setProjects(await listProjects());
    } catch {
      // daemon not yet ready — StatusBanner will surface the state
    }
  }, []);

  useEffect(() => {
    refreshProjects();
  }, [refreshProjects]);

  return (
    <>
      <StatusBanner />
      <div className="app-shell">
        {!isMobile && (
          <Sidebar nav={nav} projects={projects} onNav={setNav} />
        )}
        <div className={`main-content${isMobile ? " main-content--mobile" : ""}`}>
          {nav.page === "projects" && (
            <ProjectsPage
              projects={projects}
              onRefresh={refreshProjects}
              onOpenProject={(id) => setNav({ page: "project-detail", projectId: id })}
              onOpenChat={(projectId) => setNav({ page: "mentor", projectId })}
            />
          )}
          {nav.page === "project-detail" && (
            <ProjectDetailPage
              projectId={nav.projectId}
              projects={projects}
              onOpenChat={(projectId, conversationId) =>
                setNav({ page: "mentor", projectId, conversationId })
              }
              onBack={() => setNav({ page: "projects" })}
            />
          )}
          {nav.page === "mentor" && (
            <MentorChatPage
              initialConversationId={nav.conversationId}
              projectId={nav.projectId}
            />
          )}
          {nav.page === "audit" && <AuditLogPage />}
          {nav.page === "capability" && (
            <CapabilityPage projectId={nav.projectId} />
          )}
          {nav.page === "artifacts" && <ArtifactsPage />}
          {nav.page === "trust" && <TrustPage />}
          {nav.page === "sync" && <SyncPage />}
        </div>
        {isMobile && <BottomNav nav={nav} onNav={setNav} />}
      </div>
    </>
  );
}
