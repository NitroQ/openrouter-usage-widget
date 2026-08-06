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

  useEffect(() => {
    if (getCurrentWindow().label === "settings") {
      navigate("/settings", { replace: true });
      setLoading(false);
      return;
    }

    getAppState()
      .then((state) => {
        if (!state.configured) {
          navigate("/setup", { replace: true });
        } else if (location.pathname !== "/details" && location.pathname !== "/widget") {
          navigate("/widget", { replace: true });
        }
      })
      .catch(() => {
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
