const DAEMON_URL = "http://127.0.0.1:45678";

// ── Domain types ──────────────────────────────────────────────────────────────

export interface HealthStatus {
  status: string;
  version: string;
  daemon_ready: boolean;
}

export interface Project {
  id: string;
  name: string;
  description: string | null;
  status: "active" | "paused" | "completed" | "archived";
  created_at: number;
  updated_at: number;
}

export interface Task {
  id: string;
  project_id: string;
  title: string;
  description: string | null;
  status: "todo" | "in_progress" | "done";
  priority: number;
  due_at: number | null;
  created_at: number;
  updated_at: number;
}

export interface Conversation {
  id: string;
  project_id: string | null;
  title: string | null;
  created_at: number;
  updated_at: number;
}

export interface Message {
  id: string;
  conversation_id: string;
  role: "user" | "assistant" | "system";
  content: string;
  model: string | null;
  provider: string | null;
  created_at: number;
}

export interface MentorChunk {
  chunk: string;
  done: boolean;
  correlation_id: string;
  conversation_id?: string;
  error?: string;
}

export interface AuditEntry {
  id: string;
  event_type: string;
  correlation_id: string;
  created_at: number;
  metadata: Record<string, unknown> | null;
}

// ── HTTP helper ───────────────────────────────────────────────────────────────

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${DAEMON_URL}${path}`, init);
  if (res.status === 204) return undefined as T;
  const body = await res.json().catch(() => ({ error: res.statusText }));
  if (!res.ok) throw new Error((body as { error?: string }).error ?? `HTTP ${res.status}`);
  return body as T;
}

function json(body: unknown): RequestInit {
  return {
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  };
}

// ── Health ────────────────────────────────────────────────────────────────────

export function getHealth(): Promise<HealthStatus> {
  return apiFetch<HealthStatus>("/api/v1/health");
}

// ── Projects ──────────────────────────────────────────────────────────────────

export function listProjects(): Promise<Project[]> {
  return apiFetch<Project[]>("/api/v1/projects");
}

export function createProject(name: string, description?: string): Promise<Project> {
  return apiFetch<Project>("/api/v1/projects", {
    method: "POST",
    ...json({ name, description }),
  });
}

export function updateProject(
  id: string,
  patch: Partial<Pick<Project, "name" | "description" | "status">>
): Promise<Project> {
  return apiFetch<Project>(`/api/v1/projects/${id}`, {
    method: "PUT",
    ...json(patch),
  });
}

export function deleteProject(id: string): Promise<void> {
  return apiFetch<void>(`/api/v1/projects/${id}`, { method: "DELETE" });
}

// ── Tasks ─────────────────────────────────────────────────────────────────────

export function listTasks(projectId: string): Promise<Task[]> {
  return apiFetch<Task[]>(`/api/v1/projects/${projectId}/tasks`);
}

export function createTask(
  projectId: string,
  title: string,
  opts?: { description?: string; priority?: number; due_at?: number }
): Promise<Task> {
  return apiFetch<Task>(`/api/v1/projects/${projectId}/tasks`, {
    method: "POST",
    ...json({ title, ...opts }),
  });
}

export function updateTask(
  id: string,
  patch: Partial<Omit<Task, "id" | "project_id" | "created_at" | "updated_at">>
): Promise<Task> {
  return apiFetch<Task>(`/api/v1/tasks/${id}`, {
    method: "PUT",
    ...json(patch),
  });
}

export function deleteTask(id: string): Promise<void> {
  return apiFetch<void>(`/api/v1/tasks/${id}`, { method: "DELETE" });
}

// ── Conversations ─────────────────────────────────────────────────────────────

export function listConversations(limit = 20): Promise<Conversation[]> {
  return apiFetch<Conversation[]>(`/api/v1/conversations?limit=${limit}`);
}

export function getConversationMessages(
  conversationId: string,
  limit = 100
): Promise<Message[]> {
  return apiFetch<Message[]>(
    `/api/v1/conversations/${conversationId}/messages?limit=${limit}`
  );
}

// ── Mentor streaming ──────────────────────────────────────────────────────────

export async function streamMentorTurn(
  message: string,
  opts: {
    conversationId?: string;
    projectId?: string;
    routing?: "local" | "cloud" | "auto";
  },
  onChunk: (chunk: MentorChunk) => void,
  onDone: (chunk: MentorChunk) => void,
  onError: (err: Error) => void
): Promise<void> {
  let response: Response;
  try {
    response = await fetch(`${DAEMON_URL}/api/v1/mentor/turn`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        message,
        conversation_id: opts.conversationId,
        project_id: opts.projectId,
        routing: opts.routing ?? "local",
      }),
    });
  } catch (e) {
    onError(e instanceof Error ? e : new Error(String(e)));
    return;
  }

  if (!response.ok || !response.body) {
    onError(new Error(`HTTP ${response.status}`));
    return;
  }

  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let buffer = "";

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });
      const lines = buffer.split("\n");
      buffer = lines.pop() ?? "";

      for (const line of lines) {
        if (!line.startsWith("data: ")) continue;
        try {
          const chunk = JSON.parse(line.slice(6)) as MentorChunk;
          if (chunk.error) {
            onError(new Error(chunk.error));
            return;
          }
          if (chunk.done) {
            onDone(chunk);
            return;
          }
          onChunk(chunk);
        } catch {
          // malformed SSE line — skip
        }
      }
    }
    // flush remaining buffer
    if (buffer.startsWith("data: ")) {
      try {
        const chunk = JSON.parse(buffer.slice(6)) as MentorChunk;
        onDone(chunk);
      } catch {}
    }
  } catch (e) {
    onError(e instanceof Error ? e : new Error(String(e)));
  }
}

// ── Audit ─────────────────────────────────────────────────────────────────────

export function getAuditRecent(limit = 50): Promise<AuditEntry[]> {
  return apiFetch<AuditEntry[]>(`/api/v1/audit/recent?limit=${limit}`);
}
