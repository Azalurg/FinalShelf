import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { GenreDetails } from "../../../models/genres";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-genres-details",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./genres-details.component.html",
  styleUrl: "./genres-details.component.scss",
})
export class GenresDetailsPageComponent implements OnInit {
  genreDetails: GenreDetails | null = null;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const genreName = params.get("name");
      if (genreName) {
        this.fetchGenreDetails(genreName);
      }
    });
  }

  async fetchGenreDetails(genreName: string): Promise<void> {
    try {
      this.genreDetails = await invoke<GenreDetails>("get_genre_command", {
        genreName,
      });
    } catch (error) {
      console.error("Failed to fetch genre details:", error);
    }
  }
}
