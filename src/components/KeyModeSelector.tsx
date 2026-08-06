import type { KeyMode } from "../types/dashboard";

type Props = { value: KeyMode; onChange: (mode: KeyMode) => void };

export function KeyModeSelector({ value, onChange }: Props) {
  return (
    <div className="space-y-2">
      <label className="block text-sm font-medium text-gray-700">Key type</label>
      <div className="space-y-2">
        <label className="flex items-center gap-2 cursor-pointer">
          <input type="radio" checked={value === "standard"} onChange={() => onChange("standard")} className="accent-blue-500" />
          <span className="text-sm">Standard API key</span>
        </label>
        {value === "standard" && (
          <p className="text-xs text-gray-500 ml-5">Shows usage and spending limits for this individual API key. Historical daily data is recorded locally while this app is running.</p>
        )}
        <label className="flex items-center gap-2 cursor-pointer">
          <input type="radio" checked={value === "management"} onChange={() => onChange("management")} className="accent-blue-500" />
          <span className="text-sm">Management API key</span>
        </label>
        {value === "management" && (
          <p className="text-xs text-gray-500 ml-5">Shows account-wide credits, managed API keys, and the last 30 completed UTC days of OpenRouter activity. Management keys are administrative credentials and cannot be used for model completions.</p>
        )}
      </div>
    </div>
  );
}
