import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useDashboard } from "../hooks/useDashboard";
import { useSettings } from "../hooks/useSettings";
import { getUsageHistory } from "../lib/tauri";
import { formatCurrency, formatTimeAgo } from "../lib/format";
import type { DailyUsagePoint } from "../types/dashboard";
import { WidgetHeader } from "../components/WidgetHeader";
import { MetricCard } from "../components/MetricCard";
import { ConnectionStatus } from "../components/ConnectionStatus";

type ChartMode = "daily" | "monthly";

export function WidgetPage() {
  const { data, loading, refresh } = useDashboard(60000);
  const { settings } = useSettings();
  const [view, setView] = useState<"overview" | "chart">("overview");
  const [chartMode, setChartMode] = useState<ChartMode>("daily");
  const [history, setHistory] = useState<DailyUsagePoint[]>([]);

  useEffect(() => {
    if (view !== "chart") return;
    getUsageHistory(90)
      .then(setHistory)
      .catch(() => setHistory([]));
  }, [view]);

  const openSettings = async () => {
    try {
      await invoke("show_settings");
    } catch {
      window.location.href = "/settings";
    }
  };

  const primaryValue = data?.primaryMetric.unlimited
    ? "Unlimited"
    : data?.primaryMetric.value !== null && data?.primaryMetric.value !== undefined
      ? formatCurrency(data.primaryMetric.value)
      : "—";

  const hasLimit =
    data?.limit !== null && data?.limit !== undefined && data.limit > 0;
  const isLightTheme = settings.theme === "light";
  const todayUtc = new Date().toISOString().slice(0, 10);

  // Merge dashboard today into history
  const chartHistory = [...history];
  if (data?.usage.today !== null && data?.usage.today !== undefined) {
    const todayPoint = chartHistory.find((point) => point.dateUtc === todayUtc);
    if (todayPoint) {
      todayPoint.usage = data.usage.today;
    } else {
      chartHistory.push({
        dateUtc: todayUtc,
        usage: data.usage.today,
        byokUsage: data.usage.byokToday ?? 0,
        promptTokens: 0,
        completionTokens: 0,
        reasoningTokens: 0,
        requests: 0,
        source: "standard_key_snapshot",
        finality: "last_seen",
      });
    }
  }

  // Daily chart: last 7 days
  const historyByDate = new Map(chartHistory.map((point) => [point.dateUtc, point]));
  const dailyData: { key: string; label: string; usage: number; tooltip: string }[] = Array.from({ length: 7 }, (_, index) => {
    const date = new Date(`${todayUtc}T00:00:00Z`);
    date.setUTCDate(date.getUTCDate() - (6 - index));
    const dateUtc = date.toISOString().slice(0, 10);
    const point = historyByDate.get(dateUtc);
    const usage = point?.usage ?? 0;
    return { key: dateUtc, label: dateUtc.slice(5), usage, tooltip: `${dateUtc}: $${usage.toFixed(2)}` };
  });

  // Monthly chart: last 6 months
  const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  const monthlyMap = new Map<string, number>();
  for (const point of chartHistory) {
    const monthKey = point.dateUtc.slice(0, 7);
    monthlyMap.set(monthKey, (monthlyMap.get(monthKey) ?? 0) + point.usage);
  }
  const monthlyData: { key: string; label: string; usage: number; tooltip: string }[] = Array.from({ length: 6 }, (_, index) => {
    const d = new Date(`${todayUtc}T00:00:00Z`);
    d.setUTCMonth(d.getUTCMonth() - (5 - index));
    const monthKey = d.toISOString().slice(0, 7);
    const usage = monthlyMap.get(monthKey) ?? 0;
    const monthName = monthNames[d.getUTCMonth()];
    return { key: monthKey, label: monthName, usage, tooltip: `${monthName} ${d.getUTCFullYear()}: $${usage.toFixed(2)}` };
  });

  const isDaily = chartMode === "daily";
  const chartData = isDaily ? dailyData : monthlyData;
  const maxUsage = Math.max(...chartData.map((d) => d.usage), 0.01);

  return (
    <div
      className={`widget-surface h-screen w-screen flex flex-col rounded-xl overflow-hidden border ${isLightTheme ? "widget-light bg-gradient-to-b from-white to-gray-100 text-gray-900 border-gray-300" : "bg-gradient-to-b from-gray-950 to-gray-900 text-white border-gray-800/50"}`}
      style={{ opacity: settings.opacity }}
    >
      <WidgetHeader mode={data?.mode ?? "standard"} />

      <div className="flex-1 px-3 pb-2 flex flex-col justify-between min-h-0">
        {view === "overview" ? (
          <div className="space-y-0.5">
            <MetricCard label={data?.primaryMetric.label ?? "Remaining"} value={primaryValue} />
            {hasLimit && <MetricCard label="Limit" value={formatCurrency(data.limit)} />}
            <MetricCard label="Today" value={formatCurrency(data?.usage.today ?? null)} />
            <MetricCard label="This month" value={formatCurrency(data?.usage.month ?? null)} />
            {data?.mode === "management" && data?.keys && (
              <MetricCard label="API keys" value={`${data.keys.active} active`} />
            )}
          </div>
        ) : (
          <div className={`rounded-lg border p-2 ${isLightTheme ? "border-gray-300 bg-white/70" : "border-gray-800 bg-black/20"}`}>
            <div className="flex items-center justify-between mb-2">
              <span className="text-[10px] font-semibold uppercase tracking-[0.18em] opacity-70">{isDaily ? "Daily spend" : "Monthly spend"}</span>
              <button
                onClick={() => setChartMode((m) => m === "daily" ? "monthly" : "daily")}
                className={`text-[9px] px-1.5 py-0.5 rounded border transition-colors ${isLightTheme ? "border-gray-300 text-gray-600 hover:bg-gray-100" : "border-gray-700 text-gray-400 hover:bg-white/5"}`}
              >
                {isDaily ? "Last 7 days" : "Last 6 months"}
              </button>
            </div>
            {chartData.length === 0 ? (
              <div className="h-24 flex items-center justify-center text-xs opacity-60">No history available</div>
            ) : (
              <div className="h-24 flex items-end gap-1.5">
                {chartData.map((point) => (
                  <div key={point.key} className="flex-1 h-full flex flex-col items-center justify-end gap-1">
                    <span className="text-[8px] opacity-70">${point.usage.toFixed(2)}</span>
                    <div
                      className={`w-full rounded-t-sm min-h-[2px] ${isLightTheme ? "bg-blue-600" : "bg-cyan-400"}`}
                      style={{ height: `${Math.max((point.usage / maxUsage) * 68, 4)}%` }}
                      title={point.tooltip}
                    />
                    <span className="text-[8px] opacity-60">{point.label}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {/* Footer */}
        <div className={view === "chart" ? "space-y-1" : "space-y-2"}>
          {view === "overview" && (
            <ConnectionStatus
              status={data?.status ?? "offline"}
              lastUpdated={data?.refreshedAt ? formatTimeAgo(data.refreshedAt) : "never"}
            />
          )}
          <div className="flex items-center gap-1">
            <button
              onClick={() => refresh()}
              disabled={loading}
              className="text-xs text-gray-500 hover:text-white transition-colors px-2 py-1 rounded hover:bg-white/5 disabled:opacity-50"
              title="Refresh"
            >
              ↻
            </button>
            <button
              onClick={() => setView((current) => current === "overview" ? "chart" : "overview")}
              className="text-[10px] text-gray-500 hover:text-white transition-colors px-2 py-1 rounded hover:bg-white/5"
              title={view === "overview" ? "Show usage chart" : "Show overview"}
              aria-label={view === "overview" ? "Show usage chart" : "Show overview"}
            >
              {view === "overview" ? "Chart" : "Overview"}
            </button>
            <button
              onClick={openSettings}
              className="text-xs text-gray-500 hover:text-white transition-colors px-2 py-1 rounded hover:bg-white/5"
              title="Settings"
            >
              ⚙
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
