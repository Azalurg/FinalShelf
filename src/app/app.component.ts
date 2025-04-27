import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { SidebarComponent } from "./shared/components/sidebar/sidebar.component";
import { TopbarComponent } from "./shared/components/topbar/topbar.component";

@Component({
  selector: "app-root",
  imports: [CommonModule, RouterOutlet, SidebarComponent, TopbarComponent],
  standalone: true,
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {}
