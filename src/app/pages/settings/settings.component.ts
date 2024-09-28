import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'app-settings',
  standalone: true,
  imports: [],
  templateUrl: './settings.component.html',
  styleUrl: './settings.component.scss'
})
export class SettingsComponent {

  darkMode = false;

  async fullScan(): Promise<void>{
    try{
      const directory = prompt("Enter directory path to scan for books (it can take few minutes): ")
      if (directory) {
        await invoke("tauri_full_scan", {directory});
        alert("Scan completed successfully!") // TODO: fix alerts
      }
    }
    catch(error) {
      console.error("Error - tauri_scan", error);
      alert("Error")
    }
  }

  async quickScan(): Promise<void>{
    try{
        const directory = prompt("Enter directory path to scan for books: ")
        await invoke("tauri_quick_scan", {directory});
        alert("Scan completed successfully!")
      }
    catch(error) {
      console.error("Error - tauri_quick_scan", error);
      alert("Error")
    }
  }

  async clearDatabase(): Promise<void>{
    try{
        const consent = prompt("Are you sure you want to clear the database? This action cannot be undone. Write 'yes' to confirm.")
        if (consent !== "yes"){
          console.log("User did not confirm");
          return
        }
        await invoke("tauri_clear_db");
        alert("Database cleared successfully!")
      }
    catch(error) {
      console.error("Error - tauri_clear_db", error);
      alert("Error")
    }
  }

  toggleDarkMode(): void {
    this.darkMode = !this.darkMode;
    document.body.classList.toggle('dark', this.darkMode);
    console.log("Dark mode: ", this.darkMode);
  }
}
