import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { RouterModule } from "@angular/router";
import { Author } from "../../../models/authors";
import { convertImgPathAuthor } from "../../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../../shared/utils/getCurrentAbsolutePath";

@Component({
  selector: "app-authors",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./authors.component.html",
  styleUrl: "./authors.component.scss",
})
export class AuthorsListPageComponent {
  authors: Author[] = [];
  absolute_path = "";

  getSrc = (path: string) => convertImgPathAuthor(path, this.absolute_path);

  ngOnInit(): void {
    this.fetchAuthors();

    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
    });
  }

  async fetchAuthors() {
    try {
      const response = await invoke<{ items: Author[] }>(
        "get_authors_list_command",
        { params: { limit: 100, sort_by: "name", sort_order: "asc" } }
      );
      this.authors = response.items;
    } catch (error) {
      console.error(error);
    }
  }
}
