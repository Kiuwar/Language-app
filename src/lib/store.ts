import { create } from "zustand";
import { getSetting, setSetting } from "./invoke";

export type Theme = "system" | "light" | "dark";

interface AppStore {
  theme: Theme;
  themeLoaded: boolean;
  loadTheme: () => Promise<void>;
  setTheme: (theme: Theme) => Promise<void>;
}

function applyTheme(theme: Theme) {
  if (theme === "system") {
    document.documentElement.removeAttribute("data-theme");
  } else {
    document.documentElement.setAttribute("data-theme", theme);
  }
}

export const useAppStore = create<AppStore>((set) => ({
  theme: "system",
  themeLoaded: false,
  loadTheme: async () => {
    const stored = await getSetting("theme");
    const theme: Theme =
      stored === "light" || stored === "dark" ? stored : "system";
    applyTheme(theme);
    set({ theme, themeLoaded: true });
  },
  setTheme: async (theme) => {
    applyTheme(theme);
    set({ theme });
    await setSetting("theme", theme);
  },
}));
