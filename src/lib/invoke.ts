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
