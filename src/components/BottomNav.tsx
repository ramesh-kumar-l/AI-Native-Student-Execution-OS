import type { NavState } from "../App";

interface Props {
  nav: NavState;
  onNav: (nav: NavState) => void;
}

const ITEMS = [
  { page: "projects" as const, icon: "📁", label: "Projects" },
  { page: "mentor" as const, icon: "💬", label: "Mentor" },
  { page: "capability" as const, icon: "📈", label: "Skills" },
  { page: "artifacts" as const, icon: "🗂️", label: "Artifacts" },
  { page: "sync" as const, icon: "⇅", label: "Sync" },
];

export function BottomNav({ nav, onNav }: Props) {
  return (
    <nav className="bottom-nav" aria-label="Primary navigation">
      {ITEMS.map((item) => {
        const active =
          nav.page === item.page ||
          (item.page === "projects" && nav.page === "project-detail");
        return (
          <button
            key={item.page}
            className={`bottom-nav-item${active ? " active" : ""}`}
            onClick={() => onNav({ page: item.page })}
            aria-current={active ? "page" : undefined}
          >
            <span className="bottom-nav-icon" aria-hidden="true">
              {item.icon}
            </span>
            <span className="bottom-nav-label">{item.label}</span>
          </button>
        );
      })}
    </nav>
  );
}
