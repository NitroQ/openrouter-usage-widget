type MetricCardProps = { label: string; value: string; className?: string };

export function MetricCard({ label, value, className = "" }: MetricCardProps) {
  return (
    <div className={`flex justify-between items-center py-1 ${className}`}>
      <span className="text-xs text-gray-400 truncate mr-2">{label}</span>
      <span className="text-sm font-semibold text-white whitespace-nowrap">{value}</span>
    </div>
  );
}
