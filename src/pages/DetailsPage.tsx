import { useState, useEffect } from "react";
import type { DailyUsagePoint } from "../types/dashboard";
import { useDashboard } from "../hooks/useDashboard";
import { getUsageHistory } from "../lib/tauri";
import { formatCurrency } from "../lib/format";
import { UsageHistory } from "../components/UsageHistory";
import { MetricCard } from "../components/MetricCard";

export function DetailsPage() {
  const { data, refresh } = useDashboard(60000);
  const [history, setHistory] = useState<DailyUsagePoint[]>([]);

  useEffect(() => {
    getUsageHistory(30)
      .then(setHistory)
      .catch(() => {});
  }, []);

  return (
    <div className="h-screen bg-gray-950 text-white p-6 overflow-y-auto">
      <div className="max-w-2xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h1 className="text-lg font-bold">Usage Details</h1>
        </div>

        {/* Primary Metrics */}
        <div className="bg-gray-900/50 rounded-lg border border-gray-800 p-4 space-y-1">
          <MetricCard
            label={data?.primaryMetric.label ?? "Remaining"}
            value={data?.primaryMetric.unlimited ? "Unlimited" : formatCurrency(data?.primaryMetric.value ?? null)}
          />
          <MetricCard label="Today" value={formatCurrency(data?.usage.today ?? null)} />
          <MetricCard label="This week" value={formatCurrency(data?.usage.week ?? null)} />
          <MetricCard label="This month" value={formatCurrency(data?.usage.month ?? null)} />
          <MetricCard label="All time" value={formatCurrency(data?.usage.total ?? null)} />
          {data?.usage.byokToday !== null && data?.usage.byokToday !== undefined && (
            <MetricCard label="BYOK today" value={formatCurrency(data.usage.byokToday)} />
          )}
        </div>

        {/* Account Summary (management mode) */}
        {data?.mode === "management" && data?.account && (
          <div className="bg-gray-900/50 rounded-lg border border-gray-800 p-4 space-y-1">
            <h3 className="text-sm font-semibold text-gray-300 mb-2">Account</h3>
            <MetricCard label="Total credits" value={formatCurrency(data.account.totalCredits)} />
            <MetricCard label="Total usage" value={formatCurrency(data.account.totalUsage)} />
            <MetricCard label="Remaining" value={formatCurrency(data.account.remainingCredits)} />
          </div>
        )}

        {/* Key Summary (management mode) */}
        {data?.mode === "management" && data?.keys && (
          <div className="bg-gray-900/50 rounded-lg border border-gray-800 p-4 space-y-1">
            <h3 className="text-sm font-semibold text-gray-300 mb-2">API Keys</h3>
            <MetricCard label="Total" value={String(data.keys.total)} />
            <MetricCard label="Active" value={String(data.keys.active)} />
            <MetricCard label="Disabled" value={String(data.keys.disabled)} />
            <MetricCard label="Near limit" value={String(data.keys.nearLimit)} />
          </div>
        )}

        {/* Usage History Chart */}
        <div className="bg-gray-900/50 rounded-lg border border-gray-800">
          <UsageHistory data={history} />
        </div>

        {/* Refresh */}
        <div className="flex justify-center">
          <button
            onClick={() => refresh()}
            className="text-xs text-gray-400 hover:text-white px-4 py-2 rounded border border-gray-700 hover:border-gray-500 transition-colors"
          >
            ↻ Refresh
          </button>
        </div>
      </div>
    </div>
  );
}
