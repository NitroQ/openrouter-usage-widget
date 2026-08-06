import { describe, it, expect, vi, afterEach } from "vitest";
import { formatCurrency, formatTimeAgo, formatStatus } from "./format";

describe("formatCurrency", () => {
  it("returns em dash for null", () => {
    expect(formatCurrency(null)).toBe("—");
  });

  it("formats zero", () => {
    expect(formatCurrency(0)).toBe("$0.00");
  });

  it("formats positive value", () => {
    expect(formatCurrency(74.5)).toBe("$74.50");
  });

  it("formats large value", () => {
    expect(formatCurrency(1000.0)).toBe("$1000.00");
  });

  it("formats fractional cents", () => {
    expect(formatCurrency(0.123)).toBe("$0.12");
  });

  it("formats negative value", () => {
    expect(formatCurrency(-5.5)).toBe("$-5.50");
  });
});

describe("formatTimeAgo", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("returns 'just now' for very recent times", () => {
    const now = new Date("2026-08-06T12:00:05Z");
    vi.spyOn(Date, "now").mockReturnValue(now.getTime());
    const dateStr = "2026-08-06T12:00:00Z";
    expect(formatTimeAgo(dateStr)).toBe("just now");
  });

  it("returns seconds ago", () => {
    const now = new Date("2026-08-06T12:00:30Z");
    vi.spyOn(Date, "now").mockReturnValue(now.getTime());
    const dateStr = "2026-08-06T12:00:00Z";
    expect(formatTimeAgo(dateStr)).toBe("30s ago");
  });

  it("returns minutes ago", () => {
    const now = new Date("2026-08-06T12:05:00Z");
    vi.spyOn(Date, "now").mockReturnValue(now.getTime());
    const dateStr = "2026-08-06T12:00:00Z";
    expect(formatTimeAgo(dateStr)).toBe("5m ago");
  });

  it("returns hours ago", () => {
    const now = new Date("2026-08-06T15:00:00Z");
    vi.spyOn(Date, "now").mockReturnValue(now.getTime());
    const dateStr = "2026-08-06T12:00:00Z";
    expect(formatTimeAgo(dateStr)).toBe("3h ago");
  });

  it("returns days ago", () => {
    const now = new Date("2026-08-08T12:00:00Z");
    vi.spyOn(Date, "now").mockReturnValue(now.getTime());
    const dateStr = "2026-08-06T12:00:00Z";
    expect(formatTimeAgo(dateStr)).toBe("2d ago");
  });
});

describe("formatStatus", () => {
  it("formats live status", () => {
    const result = formatStatus("live");
    expect(result.symbol).toBe("●");
    expect(result.label).toBe("Live");
    expect(result.color).toContain("green");
  });

  it("formats refreshing status", () => {
    const result = formatStatus("refreshing");
    expect(result.symbol).toBe("◐");
    expect(result.label).toBe("Refreshing");
  });

  it("formats cached status", () => {
    const result = formatStatus("cached");
    expect(result.symbol).toBe("○");
    expect(result.label).toBe("Cached");
  });

  it("formats offline status", () => {
    const result = formatStatus("offline");
    expect(result.symbol).toBe("!");
    expect(result.label).toBe("Offline");
    expect(result.color).toContain("red");
  });

  it("formats auth_error status", () => {
    const result = formatStatus("auth_error");
    expect(result.symbol).toBe("!");
    expect(result.label).toBe("Auth failed");
  });

  it("formats unknown status", () => {
    const result = formatStatus("something_else");
    expect(result.symbol).toBe("?");
    expect(result.label).toBe("Unknown");
  });
});
