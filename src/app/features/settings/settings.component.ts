import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AbsolutePath } from "../../models/absolute-paths";

interface ScanResult {
  added: number;
  skipped: number;
  errors: unknown[];
}

@Component({
  selector: "app-settings",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./settings.component.html",
  styleUrl: "./settings.component.scss",
})
export class SettingsPageComponent implements OnInit {
  darkMode = false;
  selectedTheme = "default";
  themes = ["default", "dark", "light", "lsd", "night-city"];
  absolutePaths: AbsolutePath[] = [];
  selectedPath: AbsolutePath | null = null;
  appVersion = "";

  ngOnInit(): void {
    this.fetchAbsolutePaths();
    this.fetchVersion();
  }

  async fetchVersion(): Promise<void> {
    try {
      this.appVersion = await invoke<string>("get_version_command");
    } catch (error) {
      console.error(error);
    }
  }

  // ----------------- Functions -----------------

  async fetchAbsolutePaths(): Promise<void> {
    try {
      const paths = await invoke<AbsolutePath[]>(
        "get_all_absolute_path_command"
      );
      this.absolutePaths = paths;
      if (paths.length > 0) {
        this.selectedPath = paths[0];
      }
    } catch (error) {
      console.error(error);
    }
  }

  async quickScan(): Promise<void> {
    try {
      const result = await invoke<ScanResult>("quick_scan_command");
      alert(
        `Quick scan complete: ${result.added} added, ${result.skipped} skipped, ${result.errors.length} errors`
      );
    } catch (error) {
      console.error("Error - quick_scan_command", error);
      alert("Error");
    }
  }

  async fullScan(): Promise<void> {
    try {
      const result = await invoke<ScanResult>("full_scan_command");
      alert(
        `Full scan complete: ${result.added} added, ${result.skipped} skipped, ${result.errors.length} errors`
      );
    } catch (error) {
      console.error("Error - full_scan_command", error);
      alert("Error");
    }
  }

  async ping(): Promise<void> {
    try {
      await invoke("ping_command");
    } catch (error) {
      console.error("Ping failed:", error);
    }
  }

  async updatePath(event: Event): Promise<void> {
    const selectElement = event.target as HTMLSelectElement;
    const absolutePathId = parseInt(selectElement.value, 10);

    try {
      await invoke("set_current_absolute_path_by_id_command", {
        absolutePathId,
      }); // Pass the ID as an integer
      alert("Path updated successfully!");
    } catch (error) {
      console.error("Error - update_path", error);
      alert("Error");
    }
  }

  async addPath(): Promise<void> {
    try {
      const absolutePath = prompt(
        "Enter path to the directory with audiobooks files: "
      );
      await invoke("add_absolute_path_command", { absolutePath });
      this.fetchAbsolutePaths();
      alert("Add new absolute path! (ok)");
    } catch (error) {
      console.error("Error - add_absolute_path_command", error);
      alert(error);
    }
  }

  updateTheme(event: Event): void {
    const selectElement = event.target as HTMLSelectElement;
    const value = selectElement.value;

    document.body.classList.remove(...this.themes);
    document.body.classList.add(value);
  }
}
