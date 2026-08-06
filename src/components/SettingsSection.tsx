type Props = { title: string; children: React.ReactNode };

export function SettingsSection({ title, children }: Props) {
  return (
    <div className="border border-gray-200 rounded-lg p-4 mb-4">
      <h3 className="text-sm font-semibold text-gray-800 mb-3">{title}</h3>
      <div className="space-y-3">{children}</div>
    </div>
  );
}
