import * as vscode from 'vscode';
import { DaemonClient } from './daemon-client';
import { SignalCapture } from './signal-capture';

let client: DaemonClient | undefined;
let capture: SignalCapture | undefined;

export function activate(context: vscode.ExtensionContext): void {
  const cfg = vscode.workspace.getConfiguration('cognition');
  const enabled = cfg.get<boolean>('enabled', true);

  if (!enabled) {
    return;
  }

  const port = cfg.get<number>('daemonPort', 3737);
  const projectId = cfg.get<string>('projectId', '') || undefined;

  client = new DaemonClient(port);
  capture = new SignalCapture(client, projectId);

  capture.start(context);
  client.connect();

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(e => {
      if (!e.affectsConfiguration('cognition')) { return; }
      const updated = vscode.workspace.getConfiguration('cognition');

      if (e.affectsConfiguration('cognition.daemonPort')) {
        const newPort = updated.get<number>('daemonPort', 3737);
        client?.reconnectWithPort(newPort);
      }
      if (e.affectsConfiguration('cognition.projectId')) {
        const newProjectId = updated.get<string>('projectId', '') || undefined;
        capture?.setProjectId(newProjectId);
      }
    }),
  );
}

export function deactivate(): void {
  client?.disconnect();
}
