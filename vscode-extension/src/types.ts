/** Signal types emitted by the VSCode extension to the daemon. */
export type SignalType =
  | 'file_opened'
  | 'file_saved'
  | 'active_editor_changed'
  | 'debug_started'
  | 'debug_ended'
  | 'task_started'
  | 'task_ended';

/** Wire format sent over the WebSocket connection to the daemon. */
export interface WorkflowSignal {
  signal_type: SignalType;
  project_id?: string;
  file_path?: string;
  language?: string;
  payload?: Record<string, unknown>;
}
