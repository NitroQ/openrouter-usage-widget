type SetupRequiredStateProps = {
  onOpenSettings: () => void;
};

export function SetupRequiredState({ onOpenSettings }: SetupRequiredStateProps) {
  return (
    <div className="min-h-0 flex flex-1 items-center justify-center overflow-y-auto px-3 py-2 text-center">
      <div className="space-y-2">
        <div className="mx-auto flex h-9 w-9 items-center justify-center rounded-full border border-cyan-400/30 bg-cyan-400/10 text-lg text-cyan-300">
          !
        </div>
        <div className="space-y-1">
          <h1 className="text-sm font-semibold tracking-wide">Setup required</h1>
          <p className="text-xs leading-relaxed opacity-70">
            Add your OpenRouter API key to start monitoring usage.
          </p>
        </div>
        <button
          type="button"
          onClick={onOpenSettings}
          className="rounded-md border border-cyan-400/40 bg-cyan-400/10 px-3 py-1 text-xs font-medium text-cyan-200 transition-colors hover:bg-cyan-400/20"
        >
          Open Settings
        </button>
      </div>
    </div>
  );
}
