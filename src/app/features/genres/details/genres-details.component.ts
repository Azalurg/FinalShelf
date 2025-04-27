import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { GenreDetails } from "../../../models/genres";
import { BookListComponent } from "../../books/components/book-list/book-list.component";

@Component({
  selector: "app-genres-details",
  standalone: true,
  imports: [CommonModule, RouterModule, BookListComponent],
  templateUrl: "./genres-details.component.html",
  styleUrl: "./genres-details.component.scss",
})
export class GenresDetailsPageComponent {
  genreDetails: GenreDetails | any;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const genreName = params.get("name");
      if (genreName) {
        this.fetchGenreDetails(genreName);
      }
    });
  }

  async fetchGenreDetails(genreName: string) {
    try {
      const genreDetailsData = await invoke<GenreDetails>("get_genre_command", {
        genreName,
      });
      this.genreDetails = genreDetailsData;
      console.log(this.genreDetails);
    } catch (error) {
      console.error(error);
    }
  }
}
