# Mobile Surface — Phase 6+ Scaffold

**Status:** Planned — not yet implemented  
**Target phase:** 6+  
**Roadmap ref:** `project-memory-bank/30-roadmap.md` — Phase 6

---

## Architecture

The mobile surface re-uses the same daemon (Rust) via HTTP loopback, paired with a
Capacitor.js shell that wraps the existing React frontend. This means:

- **Zero UI duplication** — same React pages run on mobile via Capacitor WebView
- **Same API** — the daemon HTTP loopback API (`/api/v1/...`) is consumed identically
- **Same offline-first guarantee** — SQLite is bundled in the Tauri/mobile app

---

## Technology Choices

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| Shell | Capacitor.js 6.x | Bridges React WebView to native APIs |
| Daemon | Rust (Tauri Mobile) | Reuse existing daemon via Tauri 2.x mobile targets |
| DB | SQLite (same bundled) | SQLite works on iOS and Android natively |
| Sync | Cross-device export/import | Phase 5 sync mechanism works on mobile too |

---

## Tauri Mobile Support

Tauri 2.x added Android and iOS targets. The icons directory already includes:

- `icons/android/` — Android adaptive icons (mipmap-hdpi/mdpi/xhdpi/xxhdpi/xxxhdpi)
- `icons/ios/` — iOS AppIcon assets

To scaffold:
```bash
npm run tauri android init
npm run tauri ios init
```

---

## Implementation Checklist (when starting Phase 6)

- [ ] Enable Tauri Android/iOS targets in `tauri.conf.json`
- [ ] Add Capacitor plugins: Camera, Filesystem, Haptics, StatusBar
- [ ] Adapt CSS for mobile breakpoints (sidebar → bottom nav)
- [ ] Add pull-to-refresh for project list
- [ ] Test daemon startup on Android emulator
- [ ] Submit to Google Play (internal track) and TestFlight
- [ ] ADR-0005: Mobile build pipeline decision

---

## Blockers

- Phase 5 (sync) must be stable first — mobile users need cross-device sync
- Tauri mobile targets require Xcode (macOS) for iOS builds
- Android NDK setup required for Rust cross-compilation
