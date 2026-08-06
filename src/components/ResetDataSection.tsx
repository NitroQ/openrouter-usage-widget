type ResetDataSectionProps = {
  onReset: (preserveDatabase: boolean) => void;
  resetting: boolean;
};

export function ResetDataSection({ onReset, resetting }: ResetDataSectionProps) {
  return (
    <div className="space-y-3 rounded-lg border border-red-200 bg-red-50 p-3">
      <div>
        <h3 className="text-sm font-semibold text-red-800">Reset setup</h3>
        <p className="mt-1 text-xs leading-relaxed text-red-700">
          Remove the API key from Windows Credential Manager and return the app to the setup screen.
        </p>
      </div>
      <div className="space-y-2 text-xs text-red-700">
        <p>Remove the API key and keep local SQL history.</p>
        <p>Clear the API key and local SQL history.</p>
      </div>
      <div className="flex flex-wrap gap-2">
        <button
          type="button"
          disabled={resetting}
          onClick={() => onReset(true)}
          className="rounded-lg bg-white px-3 py-2 text-sm text-red-700 ring-1 ring-inset ring-red-200 hover:bg-red-100 disabled:opacity-50"
        >
          Remove Key and Keep History
        </button>
        <button
          type="button"
          disabled={resetting}
          onClick={() => onReset(false)}
          className="rounded-lg bg-red-600 px-3 py-2 text-sm text-white hover:bg-red-700 disabled:opacity-50"
        >
          Clear Key and History
        </button>
      </div>
    </div>
  );
}
