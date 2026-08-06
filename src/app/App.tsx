import { useEffect, useState } from "react";
import { useNavigate, useLocation, HashRouter, Routes, Route, Navigate } from "react-router-dom";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getAppState } from "../lib/tauri";
import { SetupPage } from "../pages/SetupPage";
import { WidgetPage } from "../pages/WidgetPage";
import { DetailsPage } from "../pages/DetailsPage";
import { SettingsPage } from "../pages/SettingsPage";

function AppRoutes() {
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();
  const location = useLocation();

  // #region debug-point route-state
  fetch("http://127.0.0.1:7777/event", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ sessionId: "white-setup-screen", runId: "pre", hypothesisId: "A", location: "src/app/App.tsx:14", msg: "[DEBUG] Route rendered", data: { pathname: location.pathname, window: getCurrentWindow().label }, ts: Date.now() }) }).catch(() => {});
  // #endregion

  useEffect(() => {
    const windowLabel = getCurrentWindow().label;

    if (windowLabel === "settings" && location.pathname !== "/setup") {
      navigate("/settings", { replace: true });
      setLoading(false);
      return;
    }

    getAppState()
      .then((state) => {
        // #region debug-point A:app-state
        fetch("http://127.0.0.1:7777/event", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ sessionId: "setup-state-exe", runId: "pre", hypothesisId: "A", location: "src/app/App.tsx:24", msg: "[DEBUG] App state resolved", data: { configured: state.configured, keyMode: state.key_mode ?? null, hasLabel: Boolean(state.key_label) }, ts: Date.now() }) }).catch(() => {});
        // #endregion
        if (!state.configured) {
          navigate(windowLabel === "main-widget" ? "/widget" : "/setup", { replace: true });
        } else if (location.pathname !== "/details" && location.pathname !== "/widget") {
          navigate("/widget", { replace: true });
        }
      })
      .catch(() => {
        // #region debug-point B:app-state-error
        fetch("http://127.0.0.1:7777/event", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ sessionId: "setup-state-exe", runId: "pre", hypothesisId: "B", location: "src/app/App.tsx:35", msg: "[DEBUG] App state rejected", data: {}, ts: Date.now() }) }).catch(() => {});
        // #endregion
        navigate("/setup", { replace: true });
      })
      .finally(() => setLoading(false));
  }, [location.pathname, navigate]);

  if (loading) {
    return (
      <div className="h-screen flex items-center justify-center bg-gray-950">
        <div className="text-gray-400 text-sm">Loading...</div>
      </div>
    );
  }

  return (
    <Routes>
      <Route path="/setup" element={<SetupPage />} />
      <Route path="/widget" element={<WidgetPage />} />
      <Route path="/details" element={<DetailsPage />} />
      <Route path="/settings" element={<SettingsPage />} />
      <Route path="*" element={<Navigate to="/setup" replace />} />
    </Routes>
  );
}

export function App() {
  return (
    <HashRouter>
      <AppRoutes />
    </HashRouter>
  );
}
