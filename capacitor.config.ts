/**
 * Capacitor configuration — Phase 6 Mobile scaffold.
 *
 * Capacitor is used alongside Tauri 2.x mobile targets to bridge native mobile
 * APIs (filesystem, haptics, camera) to the React WebView. The Rust daemon
 * continues to run via Tauri Mobile and exposes the same HTTP loopback API.
 *
 * To activate Capacitor:
 *   npm install @capacitor/core @capacitor/cli
 *   npm install @capacitor/app @capacitor/haptics @capacitor/keyboard @capacitor/status-bar
 *   npx cap add android
 *   npx cap add ios   # macOS only
 *   npx cap sync
 *
 * Android build also requires Android SDK + NDK for Rust cross-compilation.
 * iOS build requires Xcode on macOS.
 * See mobile/README.md for full prerequisites.
 */

import type { CapacitorConfig } from "@capacitor/cli";

const config: CapacitorConfig = {
  appId: "com.cognitionos.app",
  appName: "Cognition OS",
  webDir: "dist",
  server: {
    androidScheme: "https",
  },
  plugins: {
    SplashScreen: {
      launchShowDuration: 0,
      backgroundColor: "#0f0f13", // matches --bg-base CSS variable
    },
    StatusBar: {
      style: "dark",
      backgroundColor: "#1a1a24", // matches --bg-surface
    },
    Keyboard: {
      resize: "body",
      resizeOnFullScreen: true,
    },
  },
};

export default config;
