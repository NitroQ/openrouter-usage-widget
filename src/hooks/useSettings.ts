import { useState, useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import type { AppSettings } from "../types/settings";
import { getSettings, saveSettings } from "../lib/tauri";

const DEFAULT_SETTINGS: AppSettings = {
  configured: false,
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
  showInTaskbar: false,
  refreshOnLaunch: true,
  restorePosition: true,
  diagnosticLogs: false,
  lastUpdateCheckAt: null,
  lastNotifiedVersion: null,
  lastUpdateReleaseTag: null,
  lastUpdateAsset: null,
};

export function useSettings() {
  const [settings, setSettings] = useState<AppSettings>(DEFAULT_SETTINGS);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<AppSettings>("settings-updated", (event) => {
      setSettings(event.payload);
    }).then((cleanup) => {
      unlisten = cleanup;
    });

    getSettings()
      .then(setSettings)
      .catch(() => {})
      .finally(() => setLoading(false));
    return () => {
      unlisten?.();
    };
  }, []);

  const update = useCallback(async (partial: Partial<AppSettings>) => {
    const next = { ...settings, ...partial };
    setSettings(next);
    await saveSettings(next);
  }, [settings]);

  return { settings, loading, update };
}
