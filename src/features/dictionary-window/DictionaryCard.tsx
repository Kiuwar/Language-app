import type { DictionaryLookupResult } from "../../lib/invoke";

interface DictionaryCardProps {
  result: DictionaryLookupResult;
}

export default function DictionaryCard({ result }: DictionaryCardProps) {
  const { entry, known, surface_form: surfaceForm, lemma } = result;
  const showsBaseForm = surfaceForm.toLowerCase() !== lemma;

  if (!entry) {
    return (
      <div style={{ padding: 16 }}>
        <strong>{surfaceForm}</strong>
        <p style={{ color: "var(--muted)", fontSize: 13, marginTop: 8 }}>
          Ainda não há essa palavra no dicionário.
        </p>
      </div>
    );
  }

  return (
    <div style={{ padding: 16, display: "flex", flexDirection: "column", gap: 6 }}>
      <div style={{ display: "flex", alignItems: "baseline", gap: 8 }}>
        <strong style={{ fontSize: 17 }}>{entry.lemma}</strong>
        {entry.ipa && (
          <span style={{ color: "var(--muted)", fontSize: 13 }}>
            {entry.ipa}
          </span>
        )}
        {known && (
          <span
            style={{
              fontSize: 11,
              color: "var(--accent)",
              border: "1px solid color-mix(in srgb, var(--accent) 40%, transparent)",
              borderRadius: 999,
              padding: "1px 8px",
            }}
          >
            dominada
          </span>
        )}
      </div>

      {showsBaseForm && (
        <span style={{ fontSize: 12, color: "var(--muted)" }}>
          forma original: {surfaceForm}
        </span>
      )}

      {entry.pos && (
        <span style={{ fontSize: 12, color: "var(--muted)" }}>{entry.pos}</span>
      )}

      <p style={{ margin: 0, fontSize: 15 }}>{entry.translation_pt}</p>

      {entry.examples.length > 0 && (
        <ul style={{ margin: 0, paddingLeft: 18, fontSize: 12, color: "var(--muted)" }}>
          {entry.examples.slice(0, 2).map((example) => (
            <li key={example}>{example}</li>
          ))}
        </ul>
      )}

      <span style={{ fontSize: 11, color: "var(--muted)", marginTop: 4 }}>
        Áudio e salvar palavra chegam nas próximas etapas · Esc fecha
      </span>
    </div>
  );
}
