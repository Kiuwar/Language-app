import { useEffect, useState } from "react";
import { dbHealth } from "../../lib/invoke";

export default function TodayScreen() {
  const [status, setStatus] = useState("verificando banco de dados...");

  useEffect(() => {
    dbHealth()
      .then((result) => setStatus(`Banco de dados ${result}`))
      .catch((err) => setStatus(`Erro ao acessar o banco: ${err}`));
  }, []);

  return (
    <section style={{ padding: "64px 32px" }}>
      <h1 style={{ fontSize: 24, fontWeight: 600, margin: 0 }}>Hoje</h1>
      <p style={{ color: "var(--muted)", marginTop: 8 }}>
        Sua meta diária e sequência de dias aparecem aqui nas próximas etapas.
      </p>
      <p style={{ color: "var(--muted)", marginTop: 24, fontSize: 13 }}>
        {status}
      </p>
    </section>
  );
}
