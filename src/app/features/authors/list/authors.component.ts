import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
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
export class AuthorsListPageComponent implements OnInit {
  authors: Author[] = [];
  absolutePath = "";

  getSrc = (path: string) => convertImgPathAuthor(path, this.absolutePath);

  ngOnInit(): void {
    this.fetchAuthors();

    getCurrentAbsolutePath().then((path) => {
      this.absolutePath = path;
    });
  }

  async fetchAuthors(): Promise<void> {
    try {
      const response = await invoke<{ items: Author[] }>(
        "get_authors_list_command",
        { params: { limit: 100, sort_by: "name", sort_order: "asc" } }
      );
      this.authors = response.items;
    } catch (error) {
      console.error("Failed to fetch authors:", error);
    }
  }
}
