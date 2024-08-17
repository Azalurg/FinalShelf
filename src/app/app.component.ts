import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { NavbarComponent } from "./components/navbar/navbar.component";
import { BooksComponent } from "./pages/books/books.component";

@Component({
  selector: "app-root",
  imports: [CommonModule, RouterOutlet, NavbarComponent, BooksComponent],
  standalone: true,
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {}
