import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useSettings } from "../hooks/useSettings";
import { useDashboard } from "../hooks/useDashboard";
import { replaceCredential, forgetCredential, exportUsageHistoryCsv, clearUsageHistory } from "../lib/tauri";
import { formatCurrency } from "../lib/format";
import { SettingsSection } from "../components/SettingsSection";
import { KeyModeSelector } from "../components/KeyModeSelector";
import type { KeyMode } from "../types/dashboard";
import type { HistoryTimezone } from "../types/settings";
import { TIMEZONE_OPTIONS } from "../types/settings";

export function SettingsPage() {
  const navigate = useNavigate();
  const { settings, loading: settingsLoading, update } = useSettings();
  const { data, refresh } = useDashboard(60000);

  const [replaceKey, setReplaceKey] = useState("");
  const [replaceMode, setReplaceMode] = useState<KeyMode>(settings.keyMode);
  const [replacing, setReplacing] = useState(false);
  const [forgetting, setForgetting] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [clearing, setClearing] = useState(false);
  const [rebuilding, setRebuilding] = useState(false);
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  const showMessage = (type: "success" | "error", text: string) => {
    setMessage({ type, text });
    setTimeout(() => setMessage(null), 3000);
  };

  const handleReplace = async () => {
    if (!replaceKey.trim()) return;
    setReplacing(true);
    try {
      const result = await replaceCredential(replaceKey.trim(), replaceMode);
      if (result.success) {
        showMessage("success", "Key replaced successfully");
        setReplaceKey("");
        await update({ keyMode: replaceMode });
      } else {
        showMessage("error", result.message);
      }
    } catch (err) {
      showMessage("error", typeof err === "string" ? err : "Replace failed");
    } finally {
      setReplacing(false);
    }
  };

  const handleForget = async () => {
    if (!window.confirm("Are you sure? This will remove the stored API key.")) return;
    setForgetting(true);
    try {
      await forgetCredential();
      showMessage("success", "Key removed");
      await update({ configured: false });
    } catch (err) {
      showMessage("error", typeof err === "string" ? err : "Failed to remove key");
    } finally {
      setForgetting(false);
    }
  };

  const handleExportCsv = async () => {
    setExporting(true);
    try {
      const csv = await exportUsageHistoryCsv();
      const blob = new Blob([csv], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `openrouter-usage-${new Date().toISOString().slice(0, 10)}.csv`;
      a.click();
      URL.revokeObjectURL(url);
      showMessage("success", "CSV exported");
    } catch (err) {
      showMessage("error", typeof err === "string" ? err : "Export failed");
    } finally {
      setExporting(false);
    }
  };

  const handleClearHistory = async () => {
    if (!window.confirm("Clear all local usage history? This cannot be undone.")) return;
    setClearing(true);
    try {
      await clearUsageHistory();
      showMessage("success", "History cleared");
    } catch (err) {
      showMessage("error", typeof err === "string" ? err : "Clear failed");
    } finally {
      setClearing(false);
    }
  };

  const handleRebuildHistory = async () => {
    setRebuilding(true);
    try {
      await refresh();
      showMessage("success", "Usage history refreshed from API");
    } catch (err) {
      showMessage("error", typeof err === "string" ? err : "Refresh failed");
    } finally {
      setRebuilding(false);
    }
  };

  if (settingsLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-gray-400 text-sm">Loading settings...</div>
      </div>
    );
  }

  return (
    <div className="h-screen bg-gray-50 p-6 overflow-y-auto">
      <div className="max-w-2xl mx-auto space-y-4">
        {/* Header */}
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-bold text-gray-900">Settings</h1>
          <button
            onClick={() => navigate("/widget")}
            className="text-xs text-gray-500 hover:text-gray-700 px-3 py-1.5 rounded border border-gray-300 hover:bg-gray-100 transition-colors"
          >
            &larr; Widget
          </button>
        </div>

        {/* Message Toast */}
        {message && (
          <div className={`px-4 py-2 rounded-lg text-sm font-medium ${
            message.type === "success" ? "bg-green-100 text-green-800" : "bg-red-100 text-red-800"
          }`}>
            {message.text}
          </div>
        )}

        {/* Account Section */}
        <SettingsSection title="Account">
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Key Mode</label>
              <select
                value={settings.keyMode}
                onChange={(e) => update({ keyMode: e.target.value as KeyMode })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="standard">Standard</option>
                <option value="management">Management</option>
              </select>
            </div>
            {/* Key label and status */}
            <div className="bg-gray-50 rounded-lg p-3 space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-500">Status</span>
                <span className={`font-medium ${
                  data?.status === "live" ? "text-green-600" :
                  data?.status === "auth_error" ? "text-red-600" : "text-gray-500"
                }`}>
                  {data?.status === "live" ? "Connected" :
                   data?.status === "auth_error" ? "Auth Error" :
                   data?.status === "cached" ? "Cached" : data?.status ?? "Unknown"}
                </span>
              </div>
              {data?.primaryMetric?.label && (
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-500">Primary metric</span>
                  <span className="font-medium">{data.primaryMetric.label}</span>
                </div>
              )}
            </div>
          </div>

          {/* Replace Key */}
          <div className="space-y-2 pt-2 border-t border-gray-200">
            <p className="text-xs font-medium text-gray-500 uppercase tracking-wider">Replace API Key</p>
            <KeyModeSelector value={replaceMode} onChange={setReplaceMode} />
            <input
              type="password"
              value={replaceKey}
              onChange={(e) => setReplaceKey(e.target.value)}
              placeholder="New API key"
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <div className="flex gap-2">
              <button
                onClick={handleReplace}
                disabled={!replaceKey.trim() || replacing}
                className="px-4 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
              >
                {replacing ? "Replacing..." : "Replace Key"}
              </button>
              <button
                onClick={handleForget}
                disabled={forgetting}
                className="px-4 py-2 bg-red-50 text-red-600 text-sm rounded-lg hover:bg-red-100 transition-colors disabled:opacity-50"
              >
                {forgetting ? "Removing..." : "Remove Key"}
              </button>
            </div>
          </div>
        </SettingsSection>

        {/* Widget Section */}
        <SettingsSection title="Widget">
          <div className="space-y-3">
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Always on top</span>
              <input
                type="checkbox"
                checked={settings.alwaysOnTop}
                onChange={(e) => update({ alwaysOnTop: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Show in taskbar</span>
              <input
                type="checkbox"
                checked={settings.showInTaskbar}
                onChange={(e) => update({ showInTaskbar: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Close to tray</span>
              <input
                type="checkbox"
                checked={settings.closeToTray}
                onChange={(e) => update({ closeToTray: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Compact mode</span>
              <input
                type="checkbox"
                checked={settings.compactMode}
                onChange={(e) => update({ compactMode: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Refresh Interval</label>
              <select
                value={settings.refreshIntervalSeconds}
                onChange={(e) => update({ refreshIntervalSeconds: Number(e.target.value) })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value={30}>30 seconds</option>
                <option value={60}>1 minute</option>
                <option value={120}>2 minutes</option>
                <option value={300}>5 minutes</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Opacity: {Math.round(settings.opacity * 100)}%</label>
              <input
                type="range"
                min="30"
                max="100"
                value={Math.round(settings.opacity * 100)}
                onChange={(e) => update({ opacity: Number(e.target.value) / 100 })}
                className="w-full accent-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
              <select
                value={settings.theme}
                onChange={(e) => update({ theme: e.target.value as "system" | "light" | "dark" })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="system">System</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </div>
          </div>
        </SettingsSection>

        {/* Startup Section */}
        <SettingsSection title="Startup">
          <div className="space-y-3">
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Launch with Windows</span>
              <input
                type="checkbox"
                checked={settings.launchAtStartup}
                onChange={(e) => update({ launchAtStartup: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Start minimized</span>
              <input
                type="checkbox"
                checked={settings.startMinimized}
                onChange={(e) => update({ startMinimized: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Refresh on launch</span>
              <input
                type="checkbox"
                checked={settings.refreshOnLaunch}
                onChange={(e) => update({ refreshOnLaunch: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-sm text-gray-700">Restore position</span>
              <input
                type="checkbox"
                checked={settings.restorePosition}
                onChange={(e) => update({ restorePosition: e.target.checked })}
                className="accent-blue-500"
              />
            </label>
          </div>
        </SettingsSection>

        {/* Usage History Section */}
        <SettingsSection title="Usage History">
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Data source</label>
              <p className="text-xs text-gray-500 bg-gray-50 rounded-lg p-2">
                {data?.dataSource === "network" ? "Live (network)" : "Cached (local)"}
                {data?.history?.availableDays !== undefined && data.history.availableDays > 0 &&
                  ` \u2014 ${data.history.availableDays} days recorded`}
              </p>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Display timezone</label>
              <select
                value={settings.historyDisplayTimezone}
                onChange={(e) => update({ historyDisplayTimezone: e.target.value as HistoryTimezone })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {TIMEZONE_OPTIONS.map((tz) => (
                  <option key={tz.value} value={tz.value}>{tz.label}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Retention</label>
              <select
                value={settings.historyRetentionDays}
                onChange={(e) => update({ historyRetentionDays: Number(e.target.value) as 30 | 90 | 365 | -1 })}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value={30}>30 days</option>
                <option value={90}>90 days</option>
                <option value={365}>1 year</option>
                <option value={-1}>Forever</option>
              </select>
            </div>
            <div className="flex gap-2 flex-wrap">
              <button
                onClick={handleRebuildHistory}
                disabled={rebuilding}
                className="px-4 py-2 bg-blue-50 text-blue-600 text-sm rounded-lg hover:bg-blue-100 transition-colors disabled:opacity-50"
              >
                {rebuilding ? "Refreshing..." : "Refresh Now"}
              </button>
              <button
                onClick={handleExportCsv}
                disabled={exporting}
                className="px-4 py-2 bg-gray-100 text-gray-700 text-sm rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50"
              >
                {exporting ? "Exporting..." : "Export CSV"}
              </button>
              <button
                onClick={handleClearHistory}
                disabled={clearing}
                className="px-4 py-2 bg-red-50 text-red-600 text-sm rounded-lg hover:bg-red-100 transition-colors disabled:opacity-50"
              >
                {clearing ? "Clearing..." : "Clear History"}
              </button>
            </div>
          </div>
        </SettingsSection>

        {/* Management Section (only for management mode) */}
        {settings.keyMode === "management" && (
          <SettingsSection title="Management">
            {data?.account ? (
              <div className="space-y-1">
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Account credits</span>
                  <span className="font-medium">{formatCurrency(data.account.totalCredits)}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Total usage</span>
                  <span className="font-medium">{formatCurrency(data.account.totalUsage)}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Remaining</span>
                  <span className="font-medium text-green-600">{formatCurrency(data.account.remainingCredits)}</span>
                </div>
              </div>
            ) : (
              <p className="text-xs text-gray-400">Connect to view account credits.</p>
            )}

            {data?.keys && (
              <div className="space-y-2 pt-2 border-t border-gray-200">
                <p className="text-sm font-medium text-gray-700">API Keys</p>
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div className="bg-gray-50 rounded-lg p-2 text-center">
                    <div className="text-lg font-bold text-gray-900">{data.keys.total}</div>
                    <div className="text-xs text-gray-500">Total</div>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-2 text-center">
                    <div className="text-lg font-bold text-green-600">{data.keys.active}</div>
                    <div className="text-xs text-gray-500">Active</div>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-2 text-center">
                    <div className="text-lg font-bold text-red-600">{data.keys.disabled}</div>
                    <div className="text-xs text-gray-500">Disabled</div>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-2 text-center">
                    <div className="text-lg font-bold text-amber-600">{data.keys.nearLimit}</div>
                    <div className="text-xs text-gray-500">Near limit</div>
                  </div>
                </div>
              </div>
            )}
          </SettingsSection>
        )}

        {/* Privacy Section */}
        <SettingsSection title="Privacy">
          <div className="pt-2 border-t border-gray-200">
            <p className="text-xs text-gray-400">
              Your API key is stored securely in Windows Credential Manager and is never sent to any server except OpenRouter API directly.
            </p>
          </div>
        </SettingsSection>
      </div>
    </div>
  );
}
