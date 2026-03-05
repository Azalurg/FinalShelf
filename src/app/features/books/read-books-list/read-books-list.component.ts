import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { Book, BookListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-read-books-list",
  standalone: true,
  imports: [CommonModule, GenericListComponent],
  templateUrl: "./read-books-list.component.html",
  styleUrl: "./read-books-list.component.scss",
})
export class ReadBooksListPageComponent implements OnInit {
  books: Book[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;
  totalCount = 0;
  sortObject = {
    "Author ^": ["author", "asc"],
    "Author v": ["author", "desc"],
    "Title ^": ["title", "asc"],
    "Title v": ["title", "desc"],
    "Score ^": ["score", "asc"],
    "Score v": ["score", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Title ^";

  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks(): Promise<void> {
    try {
      const response = await invoke<BookListResponse>(
        "get_books_list_command",
        {
          params: {
            page: this.currentPage,
            limit: this.pageSize,
            sort_by: this.getSortField(),
            sort_order: this.getSortOrder(),
            read_status: true,
          },
        }
      );
      this.books = response.items;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch read books:", error);
    }
  }

  onPageChange(newPage: number): void {
    this.currentPage = newPage;
    this.fetchBooks();
  }

  onPageSizeChange(newSize: number): void {
    this.pageSize = newSize;
    this.currentPage = 1;
    this.fetchBooks();
  }

  onSortChange(sortIndex: string): void {
    this.sortIndex = sortIndex as keyof typeof this.sortObject;
    this.fetchBooks();
  }

  private getSortField(): string {
    return this.sortObject[this.sortIndex][0];
  }

  private getSortOrder(): string {
    return this.sortObject[this.sortIndex][1];
  }
}
