import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";

interface MenuItem {
  path: string[];
  icon: string;
  label: string;
  isExternal?: boolean;
}

@Component({
  selector: "app-sidebar",
  standalone: true,
  imports: [RouterModule, CommonModule],
  templateUrl: "./sidebar.component.html",
  styleUrl: "./sidebar.component.scss",
})
export class SidebarComponent {
  menuItems: MenuItem[] = [
    {
      path: ["/"],
      icon: "assets/icons/house-01-svgrepo-com.svg",
      label: "Dashboard",
    },
    {
      path: ["/books"],
      icon: "assets/icons/book-svgrepo-com.svg",
      label: "Books",
    },
    {
      path: ["/authors"],
      icon: "assets/icons/user-02-svgrepo-com.svg",
      label: "Authors",
    },
    {
      path: ["/lectors"],
      icon: "assets/icons/user-voice-svgrepo-com.svg",
      label: "Lectors",
    },
    {
      path: ["/genres"],
      icon: "assets/icons/notebook-svgrepo-com.svg",
      label: "Genres",
    },
  ];

  userItems: MenuItem[] = [
    {
      path: ["/read"],
      icon: "assets/icons/bookmark-svgrepo-com.svg",
      label: "Read",
    },
    {
      path: ["/ranking"],
      icon: "assets/icons/star-svgrepo-com.svg",
      label: "Ranking",
    },
  ];

  settingItems: MenuItem[] = [
    {
      path: ["https://github.com/Azalurg/FinalShelf"],
      icon: "assets/icons/globe-svgrepo-com.svg",
      label: "Profile",
      isExternal: true,
    },
    {
      path: ["/settings"],
      icon: "assets/icons/settings-svgrepo-com.svg",
      label: "Settings",
    },
  ];

  async exit(): Promise<void> {
    try {
      await invoke("kill_command");
    } catch {
      alert("Error");
    }
  }
}
