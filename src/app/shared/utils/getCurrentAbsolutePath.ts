import { invoke } from "@tauri-apps/api/core";
import { AbsolutePath } from "../../models/absolute-paths";

export async function getCurrentAbsolutePath(): Promise<string> {
  const absolute_path: AbsolutePath = await invoke(
    "get_current_absolute_path_command",
  );
  return absolute_path.absolute_path;
}
