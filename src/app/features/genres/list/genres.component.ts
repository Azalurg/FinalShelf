import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { Genre } from "../../../models/genres";

@Component({
  selector: "app-genres",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./genres.component.html",
  styleUrl: "./genres.component.scss",
})
export class GenresListPageComponent {
  genres: Genre[] = [];
  ngOnInit(): void {
    this.fetchGenres();
  }

  async fetchGenres() {
    try {
      const response = await invoke<{ items: Genre[] }>(
        "get_genres_list_command",
        { params: { limit: 100, sort_by: "name", sort_order: "asc" } }
      );
      this.genres = response.items;
    } catch (error) {
      console.error(error);
    }
  }
}
