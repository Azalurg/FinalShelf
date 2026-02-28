import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AbsolutePath } from "../../models/absolute-paths";

@Component({
  selector: "app-settings",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./settings.component.html",
  styleUrl: "./settings.component.scss",
})
export class SettingsPageComponent {
  darkMode = false;
  selectedTheme = "default";
  themes = ["default", "dark", "light", "lsd", "night-city"];
  absolutePaths: AbsolutePath[] = [];
  selectedPath: AbsolutePath | null = null;

  ngOnInit(): void {
    this.fetchAbsolutePaths();
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
      const result = await invoke<any>("quick_scan_command");
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
      const result = await invoke<any>("full_scan_command");
      alert(
        `Full scan complete: ${result.added} added, ${result.skipped} skipped, ${result.errors.length} errors`
      );
    } catch (error) {
      console.error("Error - full_scan_command", error);
      alert("Error");
    }
  }

  async ping(): Promise<void> {
    console.log("Ping");
    try {
      await invoke("ping_command");
      console.log("Pong");
    } catch (error) {
      console.log("Error");
    }
  }

  async updatePath(event: Event): Promise<void> {
    console.log("Trying to update path");

    const selectElement = event.target as HTMLSelectElement;
    const absolutePathId = parseInt(selectElement.value, 10); // Parse the value as an integer
    console.log("Selected path ID: ", absolutePathId);

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
