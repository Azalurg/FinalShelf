import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "app-sidebar",
  standalone: true,
  imports: [RouterModule],
  templateUrl: "./sidebar.component.html",
  styleUrl: "./sidebar.component.scss",
})
export class SidebarComponent {
  async exit(): Promise<void> {
    try {
      await invoke("kill_command");
    } catch (error) {
      alert("Error");
    }
  }
}
