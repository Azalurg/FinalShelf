import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AuthorListItem } from "../../../models/authors";
import { ListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-authors",
  standalone: true,
  imports: [CommonModule, GenericListComponent],
  templateUrl: "./authors.component.html",
  styleUrl: "./authors.component.scss",
})
export class AuthorsListPageComponent implements OnInit {
  authors: AuthorListItem[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;
  totalCount = 0;
  sortObject = {
    "Name ^": ["name", "asc"],
    "Name v": ["name", "desc"],
    "Books ^": ["books_count", "asc"],
    "Books v": ["books_count", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Name ^";

  ngOnInit(): void {
    this.fetchAuthors();
  }

  async fetchAuthors(): Promise<void> {
    try {
      const response = await invoke<ListResponse<AuthorListItem>>(
        "get_authors_list_command",
        {
          params: {
            page: this.currentPage,
            limit: this.pageSize,
            sort_by: this.sortObject[this.sortIndex][0],
            sort_order: this.sortObject[this.sortIndex][1],
          },
        }
      );
      this.authors = response.items;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch authors:", error);
    }
  }

  onPageChange(newPage: number): void {
    this.currentPage = newPage;
    this.fetchAuthors();
  }

  onPageSizeChange(newSize: number): void {
    this.pageSize = newSize;
    this.currentPage = 1;
    this.fetchAuthors();
  }

  onSortChange(sortIndex: string): void {
    this.sortIndex = sortIndex as keyof typeof this.sortObject;
    this.fetchAuthors();
  }
}
