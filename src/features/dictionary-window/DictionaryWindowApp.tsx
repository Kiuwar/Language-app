import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import DictionaryCard from "./DictionaryCard";
import SearchBox from "./SearchBox";
import {
  closeDictionaryWindow,
  dictionaryLookup,
  type DictionaryEntry,
  type DictionaryLookupResult,
} from "../../lib/invoke";

export default function DictionaryWindowApp() {
  const [result, setResult] = useState<DictionaryLookupResult | null>(null);
  const [mode, setMode] = useState<"search" | "result">("search");

  useEffect(() => {
    const unlisten = listen<string | null>("dictionary:open", (event) => {
      const text = event.payload;
      if (text && text.trim()) {
        setMode("result");
        dictionaryLookup(text)
          .then(setResult)
          .catch(() => setMode("search"));
      } else {
        setMode("search");
        setResult(null);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        closeDictionaryWindow();
      }
    }
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, []);

  function handleSelectFromSearch(entry: DictionaryEntry) {
    setMode("result");
    setResult({
      surface_form: entry.lemma,
      lemma: entry.lemma,
      language_code: "en",
      entry,
      known: false,
    });
  }

  return (
    <div
      style={{
        width: "100%",
        height: "100vh",
        background: "var(--bg)",
        color: "var(--fg)",
        border: "1px solid color-mix(in srgb, var(--fg) 12%, transparent)",
        borderRadius: "var(--radius)",
        overflow: "hidden",
        boxSizing: "border-box",
      }}
    >
      {mode === "result" && result ? (
        <DictionaryCard result={result} />
      ) : (
        <SearchBox onSelect={handleSelectFromSearch} />
      )}
    </div>
  );
}
