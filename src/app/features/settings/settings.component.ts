import { CommonModule } from "@angular/common";
import { Component, OnInit, OnDestroy } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AbsolutePath } from "../../models/absolute-paths";

interface ScanResult {
  added: number;
  skipped: number;
  errors: unknown[];
}

interface ScanProgress {
  phase: string;
  current: number;
  total: number;
  message: string;
}

const THEME_STORAGE_KEY = "finalshelf-theme";

@Component({
  selector: "app-settings",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./settings.component.html",
  styleUrl: "./settings.component.scss",
})
export class SettingsPageComponent implements OnInit, OnDestroy {
  darkMode = false;
  selectedTheme = "default";
  themes = ["default", "dark", "light", "lsd", "night-city"];
  absolutePaths: AbsolutePath[] = [];
  selectedPath: AbsolutePath | null = null;
  appVersion = "";

  // Scan progress state
  isScanning = false;
  scanProgress: ScanProgress | null = null;
  private unlistenScanProgress: UnlistenFn | null = null;

  ngOnInit(): void {
    this.fetchAbsolutePaths();
    this.fetchVersion();
    this.loadSavedTheme();
    this.setupScanProgressListener();
  }

  ngOnDestroy(): void {
    if (this.unlistenScanProgress) {
      this.unlistenScanProgress();
    }
  }

  private async setupScanProgressListener(): Promise<void> {
    this.unlistenScanProgress = await listen<ScanProgress>(
      "scan-progress",
      (event) => {
        this.scanProgress = event.payload;
        if (event.payload.phase === "complete") {
          this.isScanning = false;
        }
      }
    );
  }

  loadSavedTheme(): void {
    const savedTheme = localStorage.getItem(THEME_STORAGE_KEY);
    if (savedTheme && this.themes.includes(savedTheme)) {
      this.selectedTheme = savedTheme;
      this.applyTheme(savedTheme);
    }
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
    if (this.isScanning) return;

    this.isScanning = true;
    this.scanProgress = {
      phase: "starting",
      current: 0,
      total: 0,
      message: "Starting quick scan...",
    };

    try {
      const result = await invoke<ScanResult>("quick_scan_command");
      alert(
        `Quick scan complete: ${result.added} added, ${result.skipped} skipped, ${result.errors.length} errors`
      );
    } catch (error) {
      console.error("Error - quick_scan_command", error);
      alert("Error: " + error);
    } finally {
      this.isScanning = false;
      this.scanProgress = null;
    }
  }

  async fullScan(): Promise<void> {
    if (this.isScanning) return;

    this.isScanning = true;
    this.scanProgress = {
      phase: "starting",
      current: 0,
      total: 0,
      message: "Starting full scan...",
    };

    try {
      const result = await invoke<ScanResult>("full_scan_command");
      alert(
        `Full scan complete: ${result.added} added, ${result.skipped} skipped, ${result.errors.length} errors`
      );
    } catch (error) {
      console.error("Error - full_scan_command", error);
      alert("Error: " + error);
    } finally {
      this.isScanning = false;
      this.scanProgress = null;
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
    const theme = selectElement.value;
    this.selectedTheme = theme;
    this.applyTheme(theme);
    localStorage.setItem(THEME_STORAGE_KEY, theme);
  }

  private applyTheme(theme: string): void {
    document.body.classList.remove(...this.themes);
    document.body.classList.add(theme);
  }
}
