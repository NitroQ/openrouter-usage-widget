import { describe, it, expect } from "vitest";
import type { DashboardData, DailyUsagePoint, ValidationResult, KeyMode, ConnectionStatus } from "./dashboard";

describe("Dashboard types", () => {
  it("DashboardData can be constructed with standard mode", () => {
    const data: DashboardData = {
      mode: "standard",
      status: "live",
      primaryMetric: { label: "Key limit remaining", value: 74.5, unlimited: false },
      usage: { today: 1.25, week: 10.0, month: 25.5, total: 100.0, byokToday: 0.05 },
      limit: 100,
      limitRemaining: 74.5,
      history: { timezone: "UTC", todayIsProvisional: false, availableDays: 0, latest: [] },
      refreshedAt: "2026-08-06T12:00:00Z",
      dataSource: "network",
    };
    expect(data.mode).toBe("standard");
    expect(data.primaryMetric.value).toBe(74.5);
    expect(data.usage.today).toBe(1.25);
  });

  it("DashboardData can be constructed with management mode", () => {
    const data: DashboardData = {
      mode: "management",
      status: "live",
      primaryMetric: { label: "Credits remaining", value: 74.75, unlimited: false },
      usage: { today: 1.25, week: 10.0, month: 25.75, total: 100.5, byokToday: 0.0 },
      account: { totalCredits: 100.5, totalUsage: 25.75, remainingCredits: 74.75 },
      keys: { total: 8, active: 5, disabled: 2, nearLimit: 1 },
      limit: null,
      limitRemaining: null,
      history: { timezone: "UTC", todayIsProvisional: true, availableDays: 30, latest: [] },
      refreshedAt: "2026-08-06T12:00:00Z",
      dataSource: "network",
    };
    expect(data.mode).toBe("management");
    expect(data.account?.totalCredits).toBe(100.5);
    expect(data.keys?.active).toBe(5);
  });

  it("DailyUsagePoint structure", () => {
    const point: DailyUsagePoint = {
      dateUtc: "2026-08-05",
      usage: 0.25,
      byokUsage: 0.0,
      promptTokens: 1000,
      completionTokens: 500,
      reasoningTokens: 200,
      requests: 10,
      source: "openrouter_activity",
      finality: "authoritative",
    };
    expect(point.dateUtc).toBe("2026-08-05");
    expect(point.usage).toBe(0.25);
    expect(point.source).toBe("openrouter_activity");
  });

  it("ValidationResult success", () => {
    const result: ValidationResult = {
      success: true,
      message: "Connected",
      detectedMode: "standard",
      label: "test-key",
      isManagementKey: false,
    };
    expect(result.success).toBe(true);
    expect(result.detectedMode).toBe("standard");
  });

  it("ValidationResult failure", () => {
    const result: ValidationResult = {
      success: false,
      message: "Authentication failed",
    };
    expect(result.success).toBe(false);
    expect(result.detectedMode).toBeUndefined();
  });

  it("KeyMode type accepts valid values", () => {
    const standard: KeyMode = "standard";
    const management: KeyMode = "management";
    expect(standard).toBe("standard");
    expect(management).toBe("management");
  });

  it("ConnectionStatus type accepts valid values", () => {
    const statuses: ConnectionStatus[] = ["live", "cached", "offline", "auth_error", "refreshing"];
    expect(statuses.length).toBe(5);
  });
});
