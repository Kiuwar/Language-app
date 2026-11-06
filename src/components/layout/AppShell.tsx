import { useEffect } from "react";
import { Outlet } from "react-router-dom";
import Sidebar from "./Sidebar";
import CommandPalette from "../command-palette/CommandPalette";
import { useAppStore } from "../../lib/store";

export default function AppShell() {
  const loadTheme = useAppStore((state) => state.loadTheme);

  useEffect(() => {
    loadTheme();
  }, [loadTheme]);

  return (
    <div style={{ display: "flex", height: "100vh" }}>
      <Sidebar />
      <main style={{ flex: 1, overflow: "auto" }}>
        <Outlet />
      </main>
      <CommandPalette />
    </div>
  );
}
