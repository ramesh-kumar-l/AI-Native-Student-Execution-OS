import WebSocket from 'ws';
import { WorkflowSignal } from './types';

/** Max signals buffered while disconnected. Prevents unbounded memory growth. */
const MAX_QUEUE = 100;

/**
 * Persistent WebSocket connection to the local Cognition daemon.
 * Reconnects automatically with exponential backoff (1s → 30s cap).
 * Queues outgoing signals when disconnected and flushes on reconnect.
 */
export class DaemonClient {
  private ws: WebSocket | undefined;
  private port: number;
  private reconnectTimer: ReturnType<typeof setTimeout> | undefined;
  private reconnectDelay = 1000;
  private destroyed = false;
  private queue: string[] = [];

  constructor(port: number) {
    this.port = port;
  }

  connect() {
    if (this.destroyed) {
      return;
    }
    try {
      this.ws = new WebSocket(`ws://127.0.0.1:${this.port}/api/v1/ws`);

      this.ws.on('open', () => {
        this.reconnectDelay = 1000;
        const pending = this.queue.splice(0);
        for (const msg of pending) {
          this.ws?.send(msg);
        }
      });

      this.ws.on('close', () => this.scheduleReconnect());
      this.ws.on('error', () => this.scheduleReconnect());
    } catch {
      this.scheduleReconnect();
    }
  }

  send(signal: WorkflowSignal): void {
    const msg = JSON.stringify(signal);
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(msg);
    } else if (this.queue.length < MAX_QUEUE) {
      this.queue.push(msg);
    }
  }

  reconnectWithPort(port: number): void {
    this.port = port;
    this.ws?.terminate();
    this.queue = [];
    this.reconnectDelay = 1000;
    this.connect();
  }

  disconnect(): void {
    this.destroyed = true;
    if (this.reconnectTimer !== undefined) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = undefined;
    }
    this.ws?.terminate();
  }

  private scheduleReconnect(): void {
    if (this.destroyed || this.reconnectTimer !== undefined) {
      return;
    }
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = undefined;
      this.reconnectDelay = Math.min(this.reconnectDelay * 2, 30_000);
      this.connect();
    }, this.reconnectDelay);
  }
}
