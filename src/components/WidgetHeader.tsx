import { useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

type Props = { mode: string };

export function WidgetHeader({ mode }: Props) {
  const [isPinned, setIsPinned] = useState(true);

  const togglePin = async () => {
    const win = getCurrentWindow();
    await win.setAlwaysOnTop(!isPinned);
    setIsPinned(!isPinned);
  };

  return (
    <div data-tauri-drag-region className="flex items-center justify-between px-3 py-2 cursor-grab active:cursor-grabbing">
      <span data-tauri-drag-region className="text-xs font-semibold text-white/80">OpenRouter</span>
      <div className="flex items-center gap-2">
        <span className={`text-[10px] px-1.5 py-0.5 rounded font-medium ${mode === "management" ? "bg-purple-500/20 text-purple-300" : "bg-blue-500/20 text-blue-300"}`}>
          {mode === "management" ? "MGMT" : "STD"}
        </span>
        <button onClick={togglePin} className="text-gray-400 hover:text-white text-xs" title={isPinned ? "Unpin" : "Pin on top"}>
          {isPinned ? "📌" : "📍"}
        </button>
      </div>
    </div>
  );
}
