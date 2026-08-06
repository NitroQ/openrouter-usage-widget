import { formatStatus } from "../lib/format";

type Props = { status: string; lastUpdated: string };

export function ConnectionStatus({ status, lastUpdated }: Props) {
  const s = formatStatus(status);
  return (
    <div className="flex items-center justify-between text-xs">
      <span className={`${s.color} font-mono`}>{s.symbol} {s.label}</span>
      <span className="text-gray-500">{lastUpdated}</span>
    </div>
  );
}
