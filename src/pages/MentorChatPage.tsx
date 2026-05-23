import { useCallback, useEffect, useRef, useState } from "react";
import {
  getConversationMessages,
  streamMentorTurn,
  type Message,
  type MentorChunk,
} from "../api";

interface Props {
  initialConversationId?: string;
  projectId?: string;
}

interface DisplayMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  model?: string;
  provider?: string;
  isStreaming?: boolean;
}

type Routing = "local" | "cloud" | "auto";

let msgCounter = 0;
function tempId() {
  return `temp-${++msgCounter}`;
}

export function MentorChatPage({ initialConversationId, projectId }: Props) {
  const [messages, setMessages] = useState<DisplayMessage[]>([]);
  const [conversationId, setConversationId] = useState<string | undefined>(
    initialConversationId
  );
  const [input, setInput] = useState("");
  const [routing, setRouting] = useState<Routing>("local");
  const [streaming, setStreaming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // Load existing conversation history when conversationId is provided
  useEffect(() => {
    if (!initialConversationId) return;
    (async () => {
      try {
        const msgs = await getConversationMessages(initialConversationId);
        setMessages(
          msgs
            .filter((m): m is Message & { role: "user" | "assistant" } =>
              m.role === "user" || m.role === "assistant"
            )
            .map((m) => ({
              id: m.id,
              role: m.role,
              content: m.content,
              model: m.model ?? undefined,
              provider: m.provider ?? undefined,
            }))
        );
      } catch {
        // conversation may not exist yet — start fresh
      }
    })();
  }, [initialConversationId]);

  // Auto-scroll on new messages
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = useCallback(async () => {
    const text = input.trim();
    if (!text || streaming) return;

    setInput("");
    setError(null);
    setStreaming(true);

    const userMsgId = tempId();
    const assistantMsgId = tempId();

    setMessages((prev) => [
      ...prev,
      { id: userMsgId, role: "user", content: text },
      { id: assistantMsgId, role: "assistant", content: "", isStreaming: true },
    ]);

    let accumulated = "";
    let resolvedConvId = conversationId;

    await streamMentorTurn(
      text,
      { conversationId, projectId, routing },
      (chunk: MentorChunk) => {
        accumulated += chunk.chunk;
        if (chunk.conversation_id && !resolvedConvId) {
          resolvedConvId = chunk.conversation_id;
        }
        setMessages((prev) =>
          prev.map((m) =>
            m.id === assistantMsgId ? { ...m, content: accumulated } : m
          )
        );
      },
      (chunk: MentorChunk) => {
        accumulated += chunk.chunk;
        if (chunk.conversation_id) {
          resolvedConvId = chunk.conversation_id;
          setConversationId(chunk.conversation_id);
        }
        setMessages((prev) =>
          prev.map((m) =>
            m.id === assistantMsgId
              ? { ...m, content: accumulated, isStreaming: false }
              : m
          )
        );
        setStreaming(false);
      },
      (err: Error) => {
        setError(err.message);
        setMessages((prev) =>
          prev.map((m) =>
            m.id === assistantMsgId
              ? { ...m, content: accumulated || "(no response)", isStreaming: false }
              : m
          )
        );
        setStreaming(false);
      }
    );
  }, [input, streaming, conversationId, projectId, routing]);

  function handleKeyDown(e: React.KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  // Auto-resize textarea
  function handleInput(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setInput(e.target.value);
    const ta = e.target;
    ta.style.height = "auto";
    ta.style.height = Math.min(ta.scrollHeight, 120) + "px";
  }

  return (
    <div className="chat-page">
      <div className="chat-header">
        <div className="chat-header-info">
          <div className="chat-header-title">Mentor</div>
          <div className="chat-header-sub">
            {conversationId
              ? `Conversation ${conversationId.slice(0, 8)}…`
              : "New conversation"}
            {projectId && ` · project ${projectId.slice(0, 8)}…`}
          </div>
        </div>
        <button
          className="btn btn-ghost"
          style={{ fontSize: 12 }}
          onClick={() => {
            setMessages([]);
            setConversationId(undefined);
            setError(null);
          }}
        >
          + New Chat
        </button>
      </div>

      <div className="chat-messages">
        {messages.length === 0 ? (
          <div className="chat-empty">
            <div className="chat-empty-icon">🧠</div>
            <div className="chat-empty-title">Start a conversation</div>
            <div className="chat-empty-sub">
              Ask me anything about your project, skills, or what to build next
            </div>
          </div>
        ) : (
          messages.map((m) => <ChatBubble key={m.id} message={m} />)
        )}
        <div ref={bottomRef} />
      </div>

      <div className="chat-input-bar">
        <div className="chat-options">
          <span>Model:</span>
          <select
            value={routing}
            onChange={(e) => setRouting(e.target.value as Routing)}
            disabled={streaming}
          >
            <option value="local">Local (Ollama)</option>
            <option value="cloud">Cloud (Claude)</option>
            <option value="auto">Auto</option>
          </select>
          {error && <span className="chat-error">{error}</span>}
        </div>
        <div className="chat-input-row">
          <textarea
            ref={textareaRef}
            className="chat-textarea"
            value={input}
            onChange={handleInput}
            onKeyDown={handleKeyDown}
            placeholder="Ask your mentor… (Enter to send, Shift+Enter for newline)"
            rows={1}
            disabled={streaming}
          />
          <button
            className="chat-send-btn"
            onClick={handleSend}
            disabled={streaming || !input.trim()}
          >
            {streaming ? "…" : "Send"}
          </button>
        </div>
      </div>
    </div>
  );
}

function ChatBubble({ message }: { message: DisplayMessage }) {
  const isUser = message.role === "user";
  const bubbleClass = `chat-bubble ${isUser ? "chat-bubble-user" : "chat-bubble-assistant"}${message.isStreaming ? " chat-bubble-streaming" : ""}`;

  return (
    <div
      className={`chat-message ${isUser ? "chat-message-user" : "chat-message-assistant"}`}
    >
      <div className={`chat-avatar ${isUser ? "chat-avatar-user" : "chat-avatar-assistant"}`}>
        {isUser ? "U" : "M"}
      </div>
      <div>
        <div className={bubbleClass}>{message.content}</div>
        {!isUser && message.model && (
          <div className="chat-message-meta">
            {message.provider ?? "unknown"} · {message.model}
          </div>
        )}
      </div>
    </div>
  );
}
