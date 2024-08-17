import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'app-settings',
  standalone: true,
  imports: [],
  templateUrl: './settings.component.html',
  styleUrl: './settings.component.css'
})
export class SettingsComponent {
  async scanDirectory(): Promise<void>{
    try{
      const directory = prompt("Enter directory path to scan for books: ")
      if (directory) {
        await invoke("tauri_scan", {directory});
        alert("Scan completed successfully!")
      }
    }
    catch(error) {
      console.error("Error - tauri_scan", error);
      alert("Error")
    }
  }

  async clearDatabase(): Promise<void>{
    try{
        await invoke("tauri_clear_db");
        alert("Scan completed successfully!")
      }
    catch(error) {
      console.error("Error - tauri_clear_db", error);
      alert("Error")
    }
  }
}
