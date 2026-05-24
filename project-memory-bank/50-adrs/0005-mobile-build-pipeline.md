# ADR-0005 · Mobile Build Pipeline

**Status:** ACCEPTED  
**Date:** 2026-05-24  
**Deciders:** lrameshkumar126

---

## Context

Phase 6 Mobile targets Android and iOS. The existing stack is Tauri 2.x (Rust) + React.
Two options were evaluated for the mobile shell.

---

## Decision

**Dual-layer mobile architecture:**

| Layer | Technology | Role |
|-------|-----------|------|
| Rust daemon | Tauri 2.x Mobile targets | Runs on device; same SQLite + AI providers |
| React UI | Shared codebase (responsive CSS) | Same pages, mobile layout via media queries |
| Native APIs | Capacitor.js 6.x (alongside Tauri) | Camera, filesystem, haptics, status bar |

The React frontend is already responsive via Phase 6 CSS changes. Tauri 2.x provides
`tauri android init` / `tauri ios init` for native builds. Capacitor augments native API
access where Tauri's mobile plugin ecosystem is incomplete.

---

## Alternatives considered

| Option | Verdict |
|--------|---------|
| Tauri mobile only | Simpler, but Tauri mobile plugin ecosystem is maturing — some native APIs unavailable |
| Capacitor only (no Tauri mobile) | Daemon can't run Rust on mobile; would require a remote daemon, breaking offline-first |
| React Native rewrite | Duplicates all UI code; rejected |

---

## Implementation status

| Item | Status |
|------|--------|
| Responsive CSS (≤768 px) | ✅ Implemented Phase 6 |
| `BottomNav` React component | ✅ Implemented Phase 6 |
| `capacitor.config.ts` | ✅ Scaffolded |
| `npm run tauri android init` | ⏳ Requires Android SDK + NDK |
| `npm run tauri ios init` | ⏳ Requires macOS + Xcode |
| Capacitor package install | ⏳ `npm install @capacitor/core @capacitor/cli` |
| Google Play internal track | ⏳ Post-SDK-setup |
| TestFlight | ⏳ macOS only |

---

## Prerequisites for Android build

1. Install Android Studio + Android SDK (API 24+)
2. Install Android NDK for Rust cross-compilation
3. Set `ANDROID_HOME` and `NDK_HOME` env vars
4. `npm run tauri android init`
5. `npm install @capacitor/core @capacitor/cli @capacitor/app @capacitor/haptics @capacitor/keyboard @capacitor/status-bar`
6. `npx cap add android && npx cap sync`

## Prerequisites for iOS build (macOS only)

1. Xcode 14+
2. `npm run tauri ios init`
3. `npx cap add ios && npx cap sync`

---

## Consequences

- TD-03 (Tauri bundling disabled) was resolved in Phase 5. Mobile bundling is a separate concern.
- The `capacitor.config.ts` config is committed but Capacitor packages are not yet installed
  (to avoid Tauri desktop build conflicts during Phase 6 development).
- ADR-0006 will capture the final mobile release pipeline once builds are verified on device.
