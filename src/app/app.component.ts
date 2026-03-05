import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { SidebarComponent } from "./shared/components/sidebar/sidebar.component";
import { TopbarComponent } from "./shared/components/topbar/topbar.component";
import { NotificationContainerComponent } from "./shared/components/notification-container/notification-container.component";

const THEME_STORAGE_KEY = "finalshelf-theme";
const AVAILABLE_THEMES = ["default", "dark", "light", "lsd", "night-city"];

@Component({
  selector: "app-root",
  imports: [
    CommonModule,
    RouterOutlet,
    SidebarComponent,
    TopbarComponent,
    NotificationContainerComponent,
  ],
  standalone: true,
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent implements OnInit {
  ngOnInit(): void {
    this.loadSavedTheme();
  }

  private loadSavedTheme(): void {
    const savedTheme = localStorage.getItem(THEME_STORAGE_KEY);
    if (savedTheme && AVAILABLE_THEMES.includes(savedTheme)) {
      document.body.classList.remove(...AVAILABLE_THEMES);
      document.body.classList.add(savedTheme);
    }
  }
}
