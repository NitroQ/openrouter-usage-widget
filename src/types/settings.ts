export type Theme = "system" | "light" | "dark";
export type HistoryRetention = 30 | 90 | 365 | -1;

export type AppSettings = {
  configured: boolean;
  keyMode: "standard" | "management";
  refreshIntervalSeconds: number;
  alwaysOnTop: boolean;
  launchAtStartup: boolean;
  closeToTray: boolean;
  startMinimized: boolean;
  theme: Theme;
  opacity: number;
  compactMode: boolean;
  historyRetentionDays: HistoryRetention;
  historyDisplayTimezone: "utc";
  showInTaskbar: boolean;
  refreshOnLaunch: boolean;
  restorePosition: boolean;
  diagnosticLogs: boolean;
  lastUpdateCheckAt?: string | null;
  lastNotifiedVersion?: string | null;
  lastUpdateReleaseTag?: string | null;
  lastUpdateAsset?: {
    name: string;
    downloadUrl: string;
    signatureUrl: string;
    size: number;
    sha256?: string;
  } | null;
};
