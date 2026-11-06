import { Command } from "cmdk";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAppStore } from "../../lib/store";
import { buildCommands } from "./commands";

export default function CommandPalette() {
  const [open, setOpen] = useState(false);
  const navigate = useNavigate();
  const setTheme = useAppStore((state) => state.setTheme);

  const commands = useMemo(
    () => buildCommands({ navigate, setTheme }),
    [navigate, setTheme],
  );

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "k" && (event.metaKey || event.ctrlKey)) {
        event.preventDefault();
        setOpen((value) => !value);
      }
    }
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, []);

  return (
    <Command.Dialog
      open={open}
      onOpenChange={setOpen}
      label="Paleta de comandos"
    >
      <Command.Input placeholder="Digite um comando..." />
      <Command.List>
        <Command.Empty>Nenhum comando encontrado.</Command.Empty>
        {(["Navegar", "Tema"] as const).map((group) => (
          <Command.Group key={group} heading={group}>
            {commands
              .filter((command) => command.group === group)
              .map((command) => (
                <Command.Item
                  key={command.id}
                  onSelect={() => {
                    command.run();
                    setOpen(false);
                  }}
                >
                  {command.label}
                </Command.Item>
              ))}
          </Command.Group>
        ))}
      </Command.List>
    </Command.Dialog>
  );
}
