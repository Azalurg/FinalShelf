import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { AuthorDetails } from "../../../models/authors";
import { convertImgPathAuthor } from "../../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../../shared/utils/getCurrentAbsolutePath";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-author-details",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./author-details.component.html",
  styleUrl: "./author-details.component.scss",
})
export class AuthorDetailsPageComponent implements OnInit {
  authorDetails: AuthorDetails | null = null;
  absolutePath = "";

  getSrcAuthor = (path: string) =>
    convertImgPathAuthor(path, this.absolutePath);

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const authorName = params.get("name");
      if (authorName) {
        this.fetchAuthorDetails(authorName);
      }
    });

    getCurrentAbsolutePath().then((path) => {
      this.absolutePath = path;
    });
  }

  async fetchAuthorDetails(authorName: string): Promise<void> {
    try {
      this.authorDetails = await invoke<AuthorDetails>(
        "get_author_command",
        { name: authorName },
      );
    } catch (error) {
      console.error("Failed to fetch author details:", error);
    }
  }
}
