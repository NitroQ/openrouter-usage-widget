import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock the Tauri invoke function before importing the module
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
import {
  getAppState,
  validateAndSaveCredential,
  replaceCredential,
  forgetCredential,
  refreshDashboard,
  getCachedDashboard,
  getUsageHistory,
  exportUsageHistoryCsv,
  clearUsageHistory,
  getSettings,
  saveSettings,
  showWidget,
  showSettings,
  quitApplication,
} from "./tauri";

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  mockInvoke.mockReset();
});

describe("Tauri invoke wrappers", () => {
  it("getAppState calls invoke with correct command", async () => {
    mockInvoke.mockResolvedValue({ configured: true });
    const result = await getAppState();
    expect(mockInvoke).toHaveBeenCalledWith("get_app_state");
    expect(result).toEqual({ configured: true });
  });

  it("validateAndSaveCredential passes key and mode", async () => {
    mockInvoke.mockResolvedValue({ success: true, message: "Connected", detectedMode: "standard" });
    const result = await validateAndSaveCredential("sk-or-v1-test", "standard");
    expect(mockInvoke).toHaveBeenCalledWith("validate_and_save_credential", {
      key: "sk-or-v1-test", selectedMode: "standard",
    });
    expect(result.success).toBe(true);
  });

  it("replaceCredential passes key and mode", async () => {
    mockInvoke.mockResolvedValue({ success: true, message: "Replaced" });
    await replaceCredential("sk-or-v1-new", "management");
    expect(mockInvoke).toHaveBeenCalledWith("replace_credential", {
      key: "sk-or-v1-new", selectedMode: "management",
    });
  });

  it("forgetCredential calls correct command", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await forgetCredential();
    expect(mockInvoke).toHaveBeenCalledWith("forget_credential");
  });

  it("refreshDashboard calls correct command", async () => {
    const mockData = { mode: "standard", status: "live" };
    mockInvoke.mockResolvedValue(mockData);
    const result = await refreshDashboard();
    expect(mockInvoke).toHaveBeenCalledWith("refresh_dashboard");
    expect(result).toEqual(mockData);
  });

  it("getCachedDashboard calls correct command", async () => {
    mockInvoke.mockResolvedValue(null);
    const result = await getCachedDashboard();
    expect(mockInvoke).toHaveBeenCalledWith("get_cached_dashboard");
    expect(result).toBeNull();
  });

  it("getUsageHistory passes days parameter", async () => {
    mockInvoke.mockResolvedValue([]);
    await getUsageHistory(30);
    expect(mockInvoke).toHaveBeenCalledWith("get_usage_history", { days: 30 });
  });

  it("exportUsageHistoryCsv calls correct command", async () => {
    mockInvoke.mockResolvedValue("Date,Usage\n");
    const result = await exportUsageHistoryCsv();
    expect(mockInvoke).toHaveBeenCalledWith("export_usage_history_csv");
    expect(result).toBe("Date,Usage\n");
  });

  it("clearUsageHistory calls correct command", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await clearUsageHistory();
    expect(mockInvoke).toHaveBeenCalledWith("clear_usage_history");
  });

  it("getSettings calls correct command", async () => {
    mockInvoke.mockResolvedValue({ configured: true, theme: "dark" });
    const result = await getSettings();
    expect(mockInvoke).toHaveBeenCalledWith("get_settings");
    expect(result.theme).toBe("dark");
  });

  it("saveSettings passes settings object", async () => {
    mockInvoke.mockResolvedValue(undefined);
    const settings = { configured: true, theme: "dark" };
    await saveSettings(settings as any);
    expect(mockInvoke).toHaveBeenCalledWith("save_settings", { settings });
  });

  it("showWidget calls correct command", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await showWidget();
    expect(mockInvoke).toHaveBeenCalledWith("show_widget");
  });

  it("showSettings calls correct command", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await showSettings();
    expect(mockInvoke).toHaveBeenCalledWith("show_settings");
  });

  it("quitApplication calls correct command", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await quitApplication();
    expect(mockInvoke).toHaveBeenCalledWith("quit_application");
  });

  it("propagates errors from invoke", async () => {
    mockInvoke.mockRejectedValue("Command failed");
    await expect(refreshDashboard()).rejects.toBe("Command failed");
  });
});
