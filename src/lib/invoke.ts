import { invoke } from "@tauri-apps/api/core";

export function dbHealth(): Promise<string> {
  return invoke<string>("db_health");
}
