import { describe, it, expect } from "vitest";
import type { AppSettings, Theme, HistoryRetention } from "./settings";

describe("Settings types", () => {
  it("AppSettings can be constructed with all fields", () => {
    const settings: AppSettings = {
      configured: true,
      keyMode: "standard",
      refreshIntervalSeconds: 60,
      alwaysOnTop: true,
      launchAtStartup: true,
      closeToTray: true,
      startMinimized: true,
      theme: "system",
      opacity: 0.9,
      compactMode: true,
      historyRetentionDays: 365,
      historyDisplayTimezone: "utc",
      showInTaskbar: true,
      refreshOnLaunch: true,
      restorePosition: true,
      diagnosticLogs: false,
    };
    expect(settings.configured).toBe(true);
    expect(settings.refreshIntervalSeconds).toBe(60);
    expect(settings.opacity).toBe(0.9);
    expect(settings.historyRetentionDays).toBe(365);
  });

  it("Theme accepts valid values", () => {
    const themes: Theme[] = ["system", "light", "dark"];
    expect(themes.length).toBe(3);
  });

  it("HistoryRetention accepts valid values", () => {
    const retentions: HistoryRetention[] = [30, 90, 365, -1];
    expect(retentions).toContain(-1);
    expect(retentions).toContain(365);
  });

  it("AppSettings can use management key mode", () => {
    const settings: AppSettings = {
      configured: true,
      keyMode: "management",
      refreshIntervalSeconds: 60,
      alwaysOnTop: true,
      launchAtStartup: false,
      closeToTray: true,
      startMinimized: false,
      theme: "dark",
      opacity: 1.0,
      compactMode: false,
      historyRetentionDays: 90,
      historyDisplayTimezone: "utc",
      showInTaskbar: true,
      refreshOnLaunch: false,
      restorePosition: false,
      diagnosticLogs: true,
    };
    expect(settings.keyMode).toBe("management");
  });
});
