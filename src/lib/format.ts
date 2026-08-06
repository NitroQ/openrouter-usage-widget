export function formatCurrency(value: number | null): string {
  if (value === null) return "—";
  return `$${value.toFixed(2)}`;
}

export function formatTimeAgo(dateStr: string): string {
  const date = new Date(dateStr).getTime();
  const now = Date.now();
  const diffMs = now - date;
  const diffSec = Math.floor(diffMs / 1000);

  if (diffSec < 10) return "just now";
  if (diffSec < 60) return `${diffSec}s ago`;
  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;
  const diffHr = Math.floor(diffMin / 60);
  if (diffHr < 24) return `${diffHr}h ago`;
  return `${Math.floor(diffHr / 24)}d ago`;
}

export function formatStatus(status: string): { symbol: string; color: string; label: string } {
  switch (status) {
    case "live": return { symbol: "●", color: "text-green-500", label: "Live" };
    case "refreshing": return { symbol: "◐", color: "text-yellow-500", label: "Refreshing" };
    case "cached": return { symbol: "○", color: "text-gray-400", label: "Cached" };
    case "offline": return { symbol: "!", color: "text-red-500", label: "Offline" };
    case "auth_error": return { symbol: "!", color: "text-red-500", label: "Auth failed" };
    default: return { symbol: "?", color: "text-gray-400", label: "Unknown" };
  }
}
