import { useEffect, useState } from "react";
import { dictionarySearch, type DictionaryEntry } from "../../lib/invoke";

interface SearchBoxProps {
  onSelect: (entry: DictionaryEntry) => void;
}

export default function SearchBox({ onSelect }: SearchBoxProps) {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<DictionaryEntry[]>([]);

  useEffect(() => {
    const trimmed = query.trim();
    if (trimmed.length < 2) {
      setResults([]);
      return;
    }
    let cancelled = false;
    dictionarySearch(trimmed).then((found) => {
      if (!cancelled) setResults(found);
    });
    return () => {
      cancelled = true;
    };
  }, [query]);

  return (
    <div style={{ padding: 12, display: "flex", flexDirection: "column", gap: 8 }}>
      <input
        autoFocus
        value={query}
        onChange={(event) => setQuery(event.target.value)}
        placeholder="Buscar palavra..."
        style={{
          padding: "8px 10px",
          borderRadius: "var(--radius)",
          border: "1px solid color-mix(in srgb, var(--fg) 15%, transparent)",
          background: "transparent",
          color: "var(--fg)",
          fontSize: 14,
          outline: "none",
        }}
      />
      <div style={{ overflow: "auto", maxHeight: 160 }}>
        {results.map((entry) => (
          <button
            key={entry.lemma}
            onClick={() => onSelect(entry)}
            style={{
              display: "block",
              width: "100%",
              textAlign: "left",
              padding: "6px 8px",
              border: "none",
              borderRadius: "calc(var(--radius) - 4px)",
              background: "transparent",
              color: "var(--fg)",
              cursor: "pointer",
              fontSize: 13,
            }}
          >
            <strong>{entry.lemma}</strong>{" "}
            <span style={{ color: "var(--muted)" }}>{entry.translation_pt}</span>
          </button>
        ))}
        {query.trim().length >= 2 && results.length === 0 && (
          <p style={{ color: "var(--muted)", fontSize: 12, padding: "0 8px" }}>
            Nenhum resultado.
          </p>
        )}
      </div>
    </div>
  );
}
