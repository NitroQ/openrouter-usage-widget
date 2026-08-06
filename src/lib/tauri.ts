import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import type { DashboardData, ValidationResult, DailyUsagePoint, KeyMode } from "../types/dashboard";
import type { AppSettings } from "../types/settings";

export async function getAppState(): Promise<{ configured: boolean; key_mode?: string; key_label?: string }> {
  return tauriInvoke("get_app_state");
}

export async function validateAndSaveCredential(key: string, selectedMode: KeyMode): Promise<ValidationResult> {
  return tauriInvoke("validate_and_save_credential", { key, selectedMode });
}

export async function replaceCredential(key: string, selectedMode: KeyMode): Promise<ValidationResult> {
  return tauriInvoke("replace_credential", { key, selectedMode });
}

export async function forgetCredential(): Promise<void> {
  return tauriInvoke("forget_credential");
}

export async function refreshDashboard(): Promise<DashboardData> {
  return tauriInvoke("refresh_dashboard");
}

export async function getCachedDashboard(): Promise<DashboardData | null> {
  return tauriInvoke("get_cached_dashboard");
}

export async function getUsageHistory(days: number): Promise<DailyUsagePoint[]> {
  return tauriInvoke("get_usage_history", { days });
}

export async function exportUsageHistoryCsv(): Promise<string> {
  return tauriInvoke("export_usage_history_csv");
}

export async function clearUsageHistory(): Promise<void> {
  return tauriInvoke("clear_usage_history");
}

export async function getSettings(): Promise<AppSettings> {
  return tauriInvoke("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  return tauriInvoke("save_settings", { settings });
}

export async function showWidget(): Promise<void> {
  return tauriInvoke("show_widget");
}

export async function showSettings(): Promise<void> {
  return tauriInvoke("show_settings");
}

export async function quitApplication(): Promise<void> {
  return tauriInvoke("quit_application");
}
