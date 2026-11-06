import type { NavigateFunction } from "react-router-dom";
import type { Theme } from "../../lib/store";
import { openDictionaryWindow } from "../../lib/invoke";

export interface PaletteCommand {
  id: string;
  label: string;
  group: "Navegar" | "Tema" | "Dicionário";
  run: () => void;
}

interface BuildCommandsArgs {
  navigate: NavigateFunction;
  setTheme: (theme: Theme) => void;
}

export function buildCommands({
  navigate,
  setTheme,
}: BuildCommandsArgs): PaletteCommand[] {
  return [
    { id: "nav-hoje", label: "Hoje", group: "Navegar", run: () => navigate("/") },
    {
      id: "nav-revisar",
      label: "Revisar",
      group: "Navegar",
      run: () => navigate("/revisar"),
    },
    {
      id: "nav-palavras",
      label: "Palavras",
      group: "Navegar",
      run: () => navigate("/palavras"),
    },
    { id: "nav-mapa", label: "Mapa", group: "Navegar", run: () => navigate("/mapa") },
    {
      id: "nav-config",
      label: "Configurações",
      group: "Navegar",
      run: () => navigate("/configuracoes"),
    },
    {
      id: "theme-system",
      label: "Tema: Sistema",
      group: "Tema",
      run: () => setTheme("system"),
    },
    {
      id: "theme-light",
      label: "Tema: Claro",
      group: "Tema",
      run: () => setTheme("light"),
    },
    {
      id: "theme-dark",
      label: "Tema: Escuro",
      group: "Tema",
      run: () => setTheme("dark"),
    },
    {
      id: "dictionary-open",
      label: "Abrir dicionário (busca manual)",
      group: "Dicionário",
      run: () => {
        void openDictionaryWindow();
      },
    },
  ];
}
