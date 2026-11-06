import { useEffect, useState } from "react";
import EmptyState from "../../components/ui/EmptyState";
import { dbHealth } from "../../lib/invoke";

export default function TodayScreen() {
  const [status, setStatus] = useState("verificando banco de dados...");

  useEffect(() => {
    dbHealth()
      .then((result) => setStatus(`Banco de dados ${result}`))
      .catch((err) => setStatus(`Erro ao acessar o banco: ${err}`));
  }, []);

  return (
    <EmptyState
      title="Hoje"
      description="Sua meta diária e sequência de dias aparecem aqui nas próximas etapas."
    >
      <p style={{ color: "var(--muted)", marginTop: 24, fontSize: 13 }}>
        {status}
      </p>
    </EmptyState>
  );
}
