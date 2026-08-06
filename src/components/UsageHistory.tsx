import type { DailyUsagePoint } from "../types/dashboard";

type Props = { data: DailyUsagePoint[] };

export function UsageHistory({ data }: Props) {
  if (!data.length) return <div className="text-xs text-gray-500 p-4">No history available</div>;

  const maxUsage = Math.max(...data.map(d => d.usage), 0.01);

  return (
    <div className="p-4">
      <h3 className="text-sm font-semibold mb-3">Daily Usage (UTC)</h3>
      <div className="flex items-end gap-1 h-32">
        {data.slice(-14).map((d) => (
          <div key={d.dateUtc} className="flex-1 flex flex-col items-center gap-1">
            <span className="text-[9px] text-gray-500">${d.usage.toFixed(2)}</span>
            <div
              className="w-full bg-blue-500/70 rounded-t min-h-[2px]"
              style={{ height: `${(d.usage / maxUsage) * 100}%` }}
              title={`${d.dateUtc}: $${d.usage.toFixed(2)}`}
            />
            <span className="text-[8px] text-gray-600">{d.dateUtc.slice(5)}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
