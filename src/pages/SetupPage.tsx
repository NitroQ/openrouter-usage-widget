import { useState } from "react";
import { useNavigate } from "react-router-dom";
import type { KeyMode, ValidationResult } from "../types/dashboard";
import type { HistoryRetention, HistoryTimezone } from "../types/settings";
import { validateAndSaveCredential, saveSettings } from "../lib/tauri";
import { KeyModeSelector } from "../components/KeyModeSelector";

type Step = "welcome" | "credential" | "result" | "preferences" | "finish";

const DEFAULT_PREFS = {
  refreshIntervalSeconds: 60,
  alwaysOnTop: true,
  launchAtStartup: true,
  closeToTray: true,
  startMinimized: true,
  historyRetentionDays: 365 as HistoryRetention,
  historyDisplayTimezone: "utc" as HistoryTimezone,
};

export function SetupPage() {
  const navigate = useNavigate();
  const [step, setStep] = useState<Step>("welcome");
  const [apiKey, setApiKey] = useState("");
  const [keyMode, setKeyMode] = useState<KeyMode>("standard");
  const [testResult, setTestResult] = useState<ValidationResult | null>(null);
  const [testing, setTesting] = useState(false);
  const [prefs, setPrefs] = useState(DEFAULT_PREFS);
  const [saving, setSaving] = useState(false);
  const [switchedMode, setSwitchedMode] = useState(false);
  const [originalMode, setOriginalMode] = useState<KeyMode>("standard");

  const handleTestConnection = async () => {
    if (!apiKey.trim()) return;
    setTesting(true);
    setSwitchedMode(false);
    setOriginalMode(keyMode);
    try {
      const result = await validateAndSaveCredential(apiKey.trim(), keyMode);
      setTestResult(result);
      // If the detected mode differs from selected, note the mismatch
      if (result.detectedMode && result.detectedMode !== keyMode) {
        setSwitchedMode(true);
      }
      if (result.detectedMode) setKeyMode(result.detectedMode);
      setStep("result");
    } catch (err: any) {
      const message = err?.message || (typeof err === "string" ? err : JSON.stringify(err));
      setTestResult({
        success: false,
        message: message || "Connection failed",
      });
      setStep("result");
    } finally {
      setTesting(false);
    }
  };

  const handleSavePreferences = async () => {
    setSaving(true);
    try {
      await saveSettings({
        configured: true,
        keyMode,
        refreshIntervalSeconds: prefs.refreshIntervalSeconds,
        alwaysOnTop: prefs.alwaysOnTop,
        launchAtStartup: prefs.launchAtStartup,
        closeToTray: prefs.closeToTray,
        startMinimized: prefs.startMinimized,
        theme: "system",
        opacity: 0.9,
        compactMode: true,
        historyRetentionDays: prefs.historyRetentionDays,
        historyDisplayTimezone: prefs.historyDisplayTimezone,
        showInTaskbar: false,
        refreshOnLaunch: true,
        restorePosition: true,
        diagnosticLogs: false,
      });
      setStep("finish");
    } catch {
      setStep("finish");
    } finally {
      setSaving(false);
    }
  };

  const handleOpenWidget = async () => {
    navigate("/widget");
  };

  return (
    <div className="min-h-screen bg-white flex items-center justify-center p-8">
      <div className="w-full max-w-md">
        {/* Step 1: Welcome */}
        {step === "welcome" && (
          <div className="text-center space-y-6">
            <div className="space-y-2">
              <h1 className="text-2xl font-bold text-gray-900">OpenRouter Widget</h1>
              <p className="text-sm text-gray-500">Monitor your OpenRouter usage right from your desktop.</p>
            </div>
            <div className="space-y-3 text-left text-sm text-gray-600 bg-gray-50 rounded-lg p-4">
              <div className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">&#10003;</span>
                <span>Track API spending in real-time</span>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">&#10003;</span>
                <span>Monitor usage by day with charts</span>
              </div>
              <div className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">&#10003;</span>
                <span>Stay on top with an always-on widget</span>
              </div>
            </div>
            <button
              onClick={() => setStep("credential")}
              className="w-full py-2.5 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
            >
              Get Started
            </button>
          </div>
        )}

        {/* Step 2: Credential */}
        {step === "credential" && (
          <div className="space-y-6">
            <div>
              <h2 className="text-xl font-bold text-gray-900">Add API Key</h2>
              <p className="text-sm text-gray-500 mt-1">Enter your OpenRouter API key to connect.</p>
            </div>
            <KeyModeSelector value={keyMode} onChange={setKeyMode} />
            <div className="space-y-2">
              <label className="block text-sm font-medium text-gray-700">API Key</label>
              <input
                type="password"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="sk-or-v1-..."
                className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                onKeyDown={(e) => e.key === "Enter" && handleTestConnection()}
              />
              <p className="text-xs text-gray-400">Your key is stored locally and never shared.</p>
            </div>
            <div className="flex gap-3">
              <button
                onClick={() => setStep("welcome")}
                className="px-4 py-2 text-sm text-gray-600 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
              >
                Back
              </button>
              <button
                onClick={handleTestConnection}
                disabled={!apiKey.trim() || testing}
                className="flex-1 py-2 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {testing ? "Testing..." : "Test Connection"}
              </button>
            </div>
          </div>
        )}

        {/* Step 3: Test Result */}
        {step === "result" && testResult && (
          <div className="space-y-6">
            <div className="text-center space-y-2">
              <div className={`text-4xl ${testResult.success ? "text-green-500" : "text-red-500"}`}>
                {testResult.success ? "\u2713" : "\u2715"}
              </div>
              <h2 className="text-xl font-bold text-gray-900">
                {testResult.success ? "Connected!" : "Connection Failed"}
              </h2>
              <p className="text-sm text-gray-500">{testResult.message}</p>
              {testResult.label && (
                <p className="text-xs text-gray-400">Key label: {testResult.label}</p>
              )}
              {testResult.detectedMode && (
                <p className="text-xs text-blue-600">
                  Detected mode: {testResult.detectedMode === "management" ? "Management" : "Standard"}
                </p>
              )}

              {/* Mismatch warning */}
              {testResult.success && switchedMode && testResult.detectedMode && (
                <div className="bg-amber-50 border border-amber-200 rounded-lg p-3 mt-2">
                  <p className="text-sm text-amber-800 font-medium">Mode mismatch detected</p>
                  <p className="text-xs text-amber-600 mt-1">
                    You selected <strong>{originalMode === "management" ? "Management" : "Standard"}</strong> but
                    this key is a <strong>{testResult.detectedMode === "management" ? "Management" : "Standard"}</strong> key.
                    The app has been set to {testResult.detectedMode === "management" ? "Management" : "Standard"} mode.
                  </p>
                </div>
              )}
            </div>
            <div className="flex gap-3">
              {testResult.success ? (
                <>
                  <button
                    onClick={() => setStep("credential")}
                    className="px-4 py-2 text-sm text-gray-600 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
                  >
                    Use Different Key
                  </button>
                  <button
                    onClick={() => setStep("preferences")}
                    className="flex-1 py-2 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
                  >
                    Continue
                  </button>
                </>
              ) : (
                <>
                  <button
                    onClick={() => setStep("credential")}
                    className="px-4 py-2 text-sm text-gray-600 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
                  >
                    Back
                  </button>
                  <button
                    onClick={handleTestConnection}
                    disabled={testing}
                    className="flex-1 py-2 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors disabled:opacity-50"
                  >
                    {testing ? "Retrying..." : "Retry"}
                  </button>
                </>
              )}
            </div>
          </div>
        )}

        {/* Step 4: Preferences */}
        {step === "preferences" && (
          <div className="space-y-6">
            <div>
              <h2 className="text-xl font-bold text-gray-900">Preferences</h2>
              <p className="text-sm text-gray-500 mt-1">Customize your widget behavior.</p>
            </div>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Refresh Interval</label>
                <select
                  value={prefs.refreshIntervalSeconds}
                  onChange={(e) => setPrefs(p => ({ ...p, refreshIntervalSeconds: Number(e.target.value) }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value={30}>Every 30 seconds</option>
                  <option value={60}>Every 1 minute</option>
                  <option value={120}>Every 2 minutes</option>
                  <option value={300}>Every 5 minutes</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={prefs.alwaysOnTop}
                    onChange={(e) => setPrefs(p => ({ ...p, alwaysOnTop: e.target.checked }))}
                    className="accent-blue-500"
                  />
                  <span className="text-sm text-gray-700">Always on top</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={prefs.launchAtStartup}
                    onChange={(e) => setPrefs(p => ({ ...p, launchAtStartup: e.target.checked }))}
                    className="accent-blue-500"
                  />
                  <span className="text-sm text-gray-700">Launch at startup</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={prefs.closeToTray}
                    onChange={(e) => setPrefs(p => ({ ...p, closeToTray: e.target.checked }))}
                    className="accent-blue-500"
                  />
                  <span className="text-sm text-gray-700">Close to tray</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={prefs.startMinimized}
                    onChange={(e) => setPrefs(p => ({ ...p, startMinimized: e.target.checked }))}
                    className="accent-blue-500"
                  />
                  <span className="text-sm text-gray-700">Start minimized</span>
                </label>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">History Retention</label>
                <select
                  value={prefs.historyRetentionDays}
                  onChange={(e) => setPrefs(p => ({ ...p, historyRetentionDays: Number(e.target.value) as HistoryRetention }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value={30}>30 days</option>
                  <option value={90}>90 days</option>
                  <option value={365}>1 year</option>
                  <option value={-1}>Forever</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">History Timezone</label>
                <select
                  value={prefs.historyDisplayTimezone}
                  onChange={(e) => setPrefs(p => ({ ...p, historyDisplayTimezone: e.target.value as HistoryTimezone }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="utc">UTC</option>
                  <option value="asia_manila">Asia/Manila</option>
                </select>
              </div>
            </div>
            <div className="flex gap-3">
              <button
                onClick={() => setStep("result")}
                className="px-4 py-2 text-sm text-gray-600 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
              >
                Back
              </button>
              <button
                onClick={handleSavePreferences}
                disabled={saving}
                className="flex-1 py-2 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors disabled:opacity-50"
              >
                {saving ? "Saving..." : "Save & Continue"}
              </button>
            </div>
          </div>
        )}

        {/* Step 5: Finish */}
        {step === "finish" && (
          <div className="text-center space-y-6">
            <div className="space-y-2">
              <div className="text-4xl text-green-500">&#10003;</div>
              <h2 className="text-xl font-bold text-gray-900">Setup Complete</h2>
              <p className="text-sm text-gray-500">Your widget is ready to use.</p>
            </div>
            <button
              onClick={handleOpenWidget}
              className="w-full py-2.5 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
            >
              Open Widget
            </button>
          </div>
        )}

        {/* Step progress indicator */}
        <div className="flex items-center justify-center gap-1.5 mt-8">
          {(["welcome", "credential", "result", "preferences", "finish"] as Step[]).map((s, i) => (
            <div
              key={s}
              className={`w-2 h-2 rounded-full transition-colors ${
                s === step ? "bg-blue-600" : i < (["welcome", "credential", "result", "preferences", "finish"] as Step[]).indexOf(step) ? "bg-blue-300" : "bg-gray-200"
              }`}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
