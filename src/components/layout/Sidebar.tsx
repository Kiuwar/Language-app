import { NavLink } from "react-router-dom";

const navItems = [
  { to: "/", label: "Hoje" },
  { to: "/revisar", label: "Revisar" },
  { to: "/palavras", label: "Palavras" },
  { to: "/mapa", label: "Mapa" },
];

export default function Sidebar() {
  return (
    <nav
      style={{
        width: 180,
        padding: "24px 12px",
        display: "flex",
        flexDirection: "column",
        gap: 4,
        borderRight: "1px solid color-mix(in srgb, var(--fg) 10%, transparent)",
      }}
    >
      <strong style={{ padding: "0 12px 16px", fontSize: 15 }}>LexiDesk</strong>
      {navItems.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          end={item.to === "/"}
          style={({ isActive }) => ({
            padding: "8px 12px",
            borderRadius: "var(--radius)",
            textDecoration: "none",
            color: isActive ? "var(--accent)" : "var(--fg)",
            background: isActive
              ? "color-mix(in srgb, var(--accent) 12%, transparent)"
              : "transparent",
            fontWeight: isActive ? 600 : 400,
          })}
        >
          {item.label}
        </NavLink>
      ))}

      <div style={{ flex: 1 }} />

      <NavLink
        to="/configuracoes"
        title="Configurações"
        style={({ isActive }) => ({
          padding: "8px 12px",
          borderRadius: "var(--radius)",
          textDecoration: "none",
          color: isActive ? "var(--accent)" : "var(--muted)",
          borderTop: "1px solid color-mix(in srgb, var(--fg) 10%, transparent)",
          marginTop: 4,
          paddingTop: 16,
        })}
      >
        ⚙ Configurações
      </NavLink>
    </nav>
  );
}
