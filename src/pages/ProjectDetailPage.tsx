import { useCallback, useEffect, useRef, useState } from "react";
import {
  createTask,
  deleteTask,
  listTasks,
  updateProject,
  updateTask,
  type Project,
  type Task,
} from "../api";

interface Props {
  projectId: string;
  projects: Project[];
  onOpenChat: (projectId: string, conversationId?: string) => void;
  onBack: () => void;
}

const PRIORITY_LABELS: Record<number, string> = { 0: "Low", 1: "Medium", 2: "High" };
const PRIORITY_CLASSES: Record<number, string> = {
  0: "task-priority-low",
  1: "task-priority-medium",
  2: "task-priority-high",
};

const STATUS_CYCLE: Record<Task["status"], Task["status"]> = {
  todo: "in_progress",
  in_progress: "done",
  done: "todo",
};

export function ProjectDetailPage({ projectId, projects, onOpenChat, onBack }: Props) {
  const project = projects.find((p) => p.id === projectId);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTitle, setNewTitle] = useState("");
  const [addPriority, setAddPriority] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const loadTasks = useCallback(async () => {
    try {
      const data = await listTasks(projectId);
      setTasks(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load tasks");
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  async function handleAddTask(e: React.FormEvent) {
    e.preventDefault();
    if (!newTitle.trim()) return;
    try {
      const task = await createTask(projectId, newTitle.trim(), { priority: addPriority });
      setTasks((t) => [task, ...t]);
      setNewTitle("");
      inputRef.current?.focus();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to create task");
    }
  }

  async function handleCycleStatus(task: Task) {
    const next = STATUS_CYCLE[task.status];
    try {
      const updated = await updateTask(task.id, { status: next });
      setTasks((ts) => ts.map((t) => (t.id === task.id ? updated : t)));
    } catch {}
  }

  async function handleDelete(id: string) {
    try {
      await deleteTask(id);
      setTasks((ts) => ts.filter((t) => t.id !== id));
    } catch {}
  }

  async function handleProjectStatus(status: Project["status"]) {
    try {
      await updateProject(projectId, { status });
    } catch {}
  }

  const todo = tasks.filter((t) => t.status === "todo");
  const inProgress = tasks.filter((t) => t.status === "in_progress");
  const done = tasks.filter((t) => t.status === "done");

  if (!project) {
    return (
      <div className="page">
        <div className="empty-state">
          <div className="empty-state-title">Project not found</div>
          <button className="btn btn-ghost" onClick={onBack}>
            ← Back to Projects
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="page">
      <div className="breadcrumb">
        <button className="breadcrumb-link" onClick={onBack}>
          Projects
        </button>
        <span>/</span>
        <span>{project.name}</span>
      </div>

      <div className="page-header">
        <div>
          <div className="page-title">{project.name}</div>
          {project.description && (
            <div className="page-subtitle">{project.description}</div>
          )}
        </div>
        <div style={{ display: "flex", gap: 8 }}>
          <select
            value={project.status}
            onChange={(e) => handleProjectStatus(e.target.value as Project["status"])}
          >
            <option value="active">Active</option>
            <option value="paused">Paused</option>
            <option value="completed">Completed</option>
            <option value="archived">Archived</option>
          </select>
          <button
            className="btn btn-primary"
            onClick={() => onOpenChat(projectId)}
          >
            💬 Mentor Chat
          </button>
        </div>
      </div>

      {error && (
        <p style={{ color: "var(--red)", fontSize: 12, marginBottom: 12 }}>{error}</p>
      )}

      {/* Add task */}
      <form className="add-task-form" onSubmit={handleAddTask}>
        <input
          ref={inputRef}
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          placeholder="Add a task…"
          maxLength={200}
        />
        <select
          value={addPriority}
          onChange={(e) => setAddPriority(Number(e.target.value))}
        >
          <option value={2}>High</option>
          <option value={1}>Medium</option>
          <option value={0}>Low</option>
        </select>
        <button
          type="submit"
          className="btn btn-primary"
          disabled={!newTitle.trim()}
        >
          Add
        </button>
      </form>

      {loading ? (
        <div style={{ color: "var(--text-muted)", fontSize: 13, padding: "16px 0" }}>
          Loading tasks…
        </div>
      ) : tasks.length === 0 ? (
        <div className="empty-state" style={{ padding: "32px 0" }}>
          <div className="empty-state-icon">✅</div>
          <div className="empty-state-title">No tasks yet</div>
          <div className="empty-state-sub">Add your first task above</div>
        </div>
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 20, marginTop: 8 }}>
          <TaskGroup
            label="In Progress"
            tasks={inProgress}
            onCycle={handleCycleStatus}
            onDelete={handleDelete}
          />
          <TaskGroup
            label="To Do"
            tasks={todo}
            onCycle={handleCycleStatus}
            onDelete={handleDelete}
          />
          <TaskGroup
            label="Done"
            tasks={done}
            onCycle={handleCycleStatus}
            onDelete={handleDelete}
          />
        </div>
      )}
    </div>
  );
}

interface TaskGroupProps {
  label: string;
  tasks: Task[];
  onCycle: (task: Task) => void;
  onDelete: (id: string) => void;
}

function TaskGroup({ label, tasks, onCycle, onDelete }: TaskGroupProps) {
  if (tasks.length === 0) return null;

  return (
    <div>
      <div className="section-header" style={{ marginBottom: 8 }}>
        <span className="section-title">{label}</span>
        <span style={{ fontSize: 11, color: "var(--text-muted)" }}>{tasks.length}</span>
      </div>
      <div className="task-list">
        {tasks.map((t) => (
          <TaskRow key={t.id} task={t} onCycle={onCycle} onDelete={onDelete} />
        ))}
      </div>
    </div>
  );
}

interface TaskRowProps {
  task: Task;
  onCycle: (task: Task) => void;
  onDelete: (id: string) => void;
}

function TaskRow({ task, onCycle, onDelete }: TaskRowProps) {
  const checkClass = `task-check task-check-${task.status}`;
  const titleClass = `task-title${task.status === "done" ? " task-title-done" : ""}`;
  const priorityClass = PRIORITY_CLASSES[task.priority] ?? "task-priority-medium";
  const priorityLabel = PRIORITY_LABELS[task.priority] ?? "Medium";

  return (
    <div className="task-item">
      <button
        className={checkClass}
        title={`Mark as ${STATUS_CYCLE[task.status]}`}
        onClick={() => onCycle(task)}
      >
        {task.status === "done" ? "✓" : task.status === "in_progress" ? "⟳" : ""}
      </button>
      <span className={titleClass}>{task.title}</span>
      <span className={priorityClass}>{priorityLabel}</span>
      {task.description && (
        <span
          style={{ fontSize: 11, color: "var(--text-muted)" }}
          title={task.description}
        >
          ℹ
        </span>
      )}
      <div className="task-actions">
        <button
          className="btn-icon"
          title="Delete task"
          onClick={() => onDelete(task.id)}
          style={{ color: "var(--red)", fontSize: 12 }}
        >
          ✕
        </button>
      </div>
    </div>
  );
}
