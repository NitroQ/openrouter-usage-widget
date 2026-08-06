import { useState, useEffect, useCallback, useRef } from "react";
import type { DashboardData } from "../types/dashboard";
import { refreshDashboard, getCachedDashboard } from "../lib/tauri";

export function useDashboard(refreshIntervalMs: number = 60000) {
  const [data, setData] = useState<DashboardData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const backoffRef = useRef(0);
  const mountedRef = useRef(true);

  const scheduleNext = useCallback((delayMs: number) => {
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(async () => {
      if (!mountedRef.current) return;
      setLoading(true);
      setError(null);
      try {
        const result = await refreshDashboard();
        if (!mountedRef.current) return;
        setData(result);
        // Success: reset backoff, schedule at normal interval
        backoffRef.current = 0;
        scheduleNext(refreshIntervalMs);
      } catch (err) {
        if (!mountedRef.current) return;
        const msg = typeof err === "string" ? err : "Refresh failed";
        setError(msg);
        // Exponential backoff: 30s, 60s, 120s, 300s max
        backoffRef.current = Math.min(
          backoffRef.current === 0 ? 30000 : backoffRef.current * 2,
          300000
        );
        scheduleNext(backoffRef.current);
      } finally {
        setLoading(false);
      }
    }, delayMs);
  }, [refreshIntervalMs]);

  const loadCached = useCallback(async () => {
    try {
      const cached = await getCachedDashboard();
      if (cached) setData(cached);
    } catch {
      // ignore
    }
  }, []);

  useEffect(() => {
    mountedRef.current = true;
    loadCached();
    // First refresh immediately, then schedule subsequent refreshes
    setLoading(true);
    refreshDashboard()
      .then((result) => {
        if (mountedRef.current) {
          setData(result);
          backoffRef.current = 0;
          scheduleNext(refreshIntervalMs);
        }
      })
      .catch((err) => {
        if (mountedRef.current) {
          setError(typeof err === "string" ? err : "Refresh failed");
          backoffRef.current = 30000;
          scheduleNext(30000);
        }
      })
      .finally(() => {
        if (mountedRef.current) setLoading(false);
      });

    return () => {
      mountedRef.current = false;
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [loadCached, scheduleNext, refreshIntervalMs]);

  const refresh = useCallback(async () => {
    // Manual refresh: reset backoff and fetch immediately
    if (timerRef.current) clearTimeout(timerRef.current);
    setLoading(true);
    setError(null);
    try {
      const result = await refreshDashboard();
      setData(result);
      backoffRef.current = 0;
      scheduleNext(refreshIntervalMs);
    } catch (err) {
      setError(typeof err === "string" ? err : "Refresh failed");
      backoffRef.current = 30000;
      scheduleNext(30000);
    } finally {
      setLoading(false);
    }
  }, [scheduleNext, refreshIntervalMs]);

  return { data, loading, error, refresh };
}
