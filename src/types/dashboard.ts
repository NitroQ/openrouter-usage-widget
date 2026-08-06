export type KeyMode = "standard" | "management";
export type ConnectionStatus = "live" | "cached" | "offline" | "auth_error" | "refreshing";

export type DashboardData = {
  mode: KeyMode;
  status: ConnectionStatus;
  primaryMetric: {
    label: string;
    value: number | null;
    unlimited: boolean;
  };
  usage: {
    today: number | null;
    week: number | null;
    month: number | null;
    total: number | null;
    byokToday: number | null;
  };
  account?: {
    totalCredits: number;
    totalUsage: number;
    remainingCredits: number;
  };
  keys?: {
    total: number;
    active: number;
    disabled: number;
    nearLimit: number;
  };
  limit: number | null;
  limitRemaining: number | null;
  history: {
    timezone: string;
    todayIsProvisional: boolean;
    availableDays: number;
    latest: DailyUsagePoint[];
  };
  refreshedAt: string;
  dataSource: "network" | "cache";
};

export type DailyUsagePoint = {
  dateUtc: string;
  usage: number;
  byokUsage: number;
  promptTokens: number;
  completionTokens: number;
  reasoningTokens: number;
  requests: number;
  source?: string;
  finality?: string;
};

export type ValidationResult = {
  success: boolean;
  message: string;
  detectedMode?: KeyMode;
  label?: string;
  isManagementKey?: boolean;
};
