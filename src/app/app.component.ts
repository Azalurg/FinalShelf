import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { NavbarComponent } from "./shared/components/navbar/navbar.component";
import { ToolbarComponent } from "./shared/components/toolbar/toolbar.component";

@Component({
  selector: "app-root",
  imports: [
    CommonModule,
    RouterOutlet,
    NavbarComponent,
    ToolbarComponent
],
  standalone: true,
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {}
