import * as vscode from 'vscode';
import { DaemonClient } from './daemon-client';
import { SignalType } from './types';

/** Minimum ms between `active_editor_changed` signals to avoid flooding. */
const EDITOR_THROTTLE_MS = 1_000;

/**
 * Registers VSCode event listeners and forwards workflow signals to the daemon.
 * All subscriptions are registered via `context.subscriptions` so VSCode
 * disposes them automatically when the extension deactivates.
 */
export class SignalCapture {
  private projectId: string | undefined;
  private readonly client: DaemonClient;
  private lastEditorChange = 0;

  constructor(client: DaemonClient, projectId: string | undefined) {
    this.client = client;
    this.projectId = projectId;
  }

  setProjectId(id: string | undefined): void {
    this.projectId = id;
  }

  start(context: vscode.ExtensionContext): void {
    context.subscriptions.push(
      vscode.workspace.onDidOpenTextDocument(doc => {
        if (doc.uri.scheme !== 'file') { return; }
        this.emit('file_opened', doc.uri.fsPath, doc.languageId);
      }),

      vscode.workspace.onDidSaveTextDocument(doc => {
        if (doc.uri.scheme !== 'file') { return; }
        this.emit('file_saved', doc.uri.fsPath, doc.languageId);
      }),

      vscode.window.onDidChangeActiveTextEditor(editor => {
        if (!editor || editor.document.uri.scheme !== 'file') { return; }
        const now = Date.now();
        if (now - this.lastEditorChange < EDITOR_THROTTLE_MS) { return; }
        this.lastEditorChange = now;
        this.emit('active_editor_changed', editor.document.uri.fsPath, editor.document.languageId);
      }),

      vscode.debug.onDidStartDebugSession(session => {
        this.emit('debug_started', undefined, undefined, {
          session_type: session.type,
          session_name: session.name,
        });
      }),

      vscode.debug.onDidTerminateDebugSession(session => {
        this.emit('debug_ended', undefined, undefined, {
          session_type: session.type,
          session_name: session.name,
        });
      }),

      vscode.tasks.onDidStartTask(e => {
        this.emit('task_started', undefined, undefined, {
          task_name: e.execution.task.name,
          task_type: e.execution.task.definition.type,
        });
      }),

      vscode.tasks.onDidEndTask(e => {
        this.emit('task_ended', undefined, undefined, {
          task_name: e.execution.task.name,
          task_type: e.execution.task.definition.type,
        });
      }),
    );
  }

  private emit(
    signalType: SignalType,
    filePath: string | undefined,
    language: string | undefined,
    payload?: Record<string, unknown>,
  ): void {
    this.client.send({
      signal_type: signalType,
      project_id: this.projectId,
      file_path: filePath,
      language,
      payload,
    });
  }
}
