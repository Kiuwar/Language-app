import { invoke } from "@tauri-apps/api/core";

export function dbHealth(): Promise<string> {
  return invoke<string>("db_health");
}

export function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>("get_setting", { key });
}

export function setSetting(key: string, value: string): Promise<void> {
  return invoke<void>("set_setting", { key, value });
}

export interface DictionaryEntry {
  lemma: string;
  pos: string | null;
  ipa: string | null;
  translation_pt: string;
  examples: string[];
}

export interface DictionaryLookupResult {
  surface_form: string;
  lemma: string;
  language_code: string | null;
  entry: DictionaryEntry | null;
  known: boolean;
}

export function dictionaryLookup(text: string): Promise<DictionaryLookupResult> {
  return invoke<DictionaryLookupResult>("dictionary_lookup", { text });
}

export function dictionarySearch(query: string): Promise<DictionaryEntry[]> {
  return invoke<DictionaryEntry[]>("dictionary_search", { query });
}

export function openDictionaryWindow(text?: string): Promise<void> {
  return invoke<void>("open_dictionary_window", { text: text ?? null });
}

export function closeDictionaryWindow(): Promise<void> {
  return invoke<void>("close_dictionary_window");
}
