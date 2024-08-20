import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { NavbarComponent } from "./components/navbar/navbar.component";
import { ToolbarComponent } from "./components/toolbar/toolbar.component";

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
