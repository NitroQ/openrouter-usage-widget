export type Theme = "system" | "light" | "dark";
export type HistoryTimezone =
  | "gmt_plus_12"
  | "gmt_plus_11"
  | "gmt_plus_10"
  | "gmt_plus_9"
  | "gmt_plus_8"
  | "gmt_plus_7"
  | "gmt_plus_6"
  | "gmt_plus_5"
  | "gmt_plus_4"
  | "gmt_plus_3"
  | "gmt_plus_2"
  | "gmt_plus_1"
  | "utc"
  | "gmt_minus_1"
  | "gmt_minus_2"
  | "gmt_minus_3"
  | "gmt_minus_4"
  | "gmt_minus_5"
  | "gmt_minus_6"
  | "gmt_minus_7"
  | "gmt_minus_8"
  | "gmt_minus_9"
  | "gmt_minus_10"
  | "gmt_minus_11"
  | "gmt_minus_12"
  | "pacific_auckland"
  | "australia_sydney"
  | "asia_tokyo"
  | "asia_seoul"
  | "asia_shanghai"
  | "asia_singapore"
  | "asia_manila"
  | "asia_hong_kong"
  | "asia_bangkok"
  | "asia_kolkata"
  | "asia_dubai"
  | "europe_moscow"
  | "europe_istanbul"
  | "europe_berlin"
  | "europe_paris"
  | "europe_london"
  | "america_sao_paulo"
  | "america_new_york"
  | "america_chicago"
  | "america_denver"
  | "america_los_angeles"
  | "america_anchorage"
  | "pacific_honolulu";

export const TIMEZONE_OPTIONS: { value: HistoryTimezone; label: string }[] = [
  { value: "gmt_plus_12", label: "GMT+12" },
  { value: "gmt_plus_11", label: "GMT+11" },
  { value: "gmt_plus_10", label: "GMT+10" },
  { value: "gmt_plus_9", label: "GMT+9" },
  { value: "gmt_plus_8", label: "GMT+8" },
  { value: "gmt_plus_7", label: "GMT+7" },
  { value: "gmt_plus_6", label: "GMT+6" },
  { value: "gmt_plus_5", label: "GMT+5" },
  { value: "gmt_plus_4", label: "GMT+4" },
  { value: "gmt_plus_3", label: "GMT+3" },
  { value: "gmt_plus_2", label: "GMT+2" },
  { value: "gmt_plus_1", label: "GMT+1" },
  { value: "utc", label: "GMT+0 (UTC)" },
  { value: "gmt_minus_1", label: "GMT-1" },
  { value: "gmt_minus_2", label: "GMT-2" },
  { value: "gmt_minus_3", label: "GMT-3" },
  { value: "gmt_minus_4", label: "GMT-4" },
  { value: "gmt_minus_5", label: "GMT-5" },
  { value: "gmt_minus_6", label: "GMT-6" },
  { value: "gmt_minus_7", label: "GMT-7" },
  { value: "gmt_minus_8", label: "GMT-8" },
  { value: "gmt_minus_9", label: "GMT-9" },
  { value: "gmt_minus_10", label: "GMT-10" },
  { value: "gmt_minus_11", label: "GMT-11" },
  { value: "gmt_minus_12", label: "GMT-12" },
  { value: "pacific_auckland", label: "Pacific/Auckland (GMT+12)" },
  { value: "australia_sydney", label: "Australia/Sydney (GMT+10)" },
  { value: "asia_tokyo", label: "Asia/Tokyo (GMT+9)" },
  { value: "asia_seoul", label: "Asia/Seoul (GMT+9)" },
  { value: "asia_shanghai", label: "Asia/Shanghai (GMT+8)" },
  { value: "asia_singapore", label: "Asia/Singapore (GMT+8)" },
  { value: "asia_manila", label: "Asia/Manila (GMT+8)" },
  { value: "asia_hong_kong", label: "Asia/Hong_Kong (GMT+8)" },
  { value: "asia_bangkok", label: "Asia/Bangkok (GMT+7)" },
  { value: "asia_kolkata", label: "Asia/Kolkata (GMT+5:30)" },
  { value: "asia_dubai", label: "Asia/Dubai (GMT+4)" },
  { value: "europe_moscow", label: "Europe/Moscow (GMT+3)" },
  { value: "europe_istanbul", label: "Europe/Istanbul (GMT+3)" },
  { value: "europe_berlin", label: "Europe/Berlin (GMT+1)" },
  { value: "europe_paris", label: "Europe/Paris (GMT+1)" },
  { value: "europe_london", label: "Europe/London (GMT+0)" },
  { value: "america_sao_paulo", label: "America/Sao_Paulo (GMT-3)" },
  { value: "america_new_york", label: "America/New_York (GMT-5)" },
  { value: "america_chicago", label: "America/Chicago (GMT-6)" },
  { value: "america_denver", label: "America/Denver (GMT-7)" },
  { value: "america_los_angeles", label: "America/Los_Angeles (GMT-8)" },
  { value: "america_anchorage", label: "America/Anchorage (GMT-9)" },
  { value: "pacific_honolulu", label: "Pacific/Honolulu (GMT-10)" },
];

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
  historyDisplayTimezone: HistoryTimezone;
  showInTaskbar: boolean;
  refreshOnLaunch: boolean;
  restorePosition: boolean;
  diagnosticLogs: boolean;
};
