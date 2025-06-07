import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { Book, BookListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-books",
  standalone: true,
  imports: [CommonModule, GenericListComponent],
  templateUrl: "./list.component.html",
  styleUrl: "./list.component.scss",
})
export class BooksListPageComponent {
  books: Book[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1; // You'll need to get this from your backend
  sortOptions = ["Author ^", "Author v", "Title ^", "Title v"];
  totalCount = 0;

  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks(): Promise<void> {
    try {
      const response = await invoke<BookListResponse>(
        "get_books_list_command",
        {
          page: this.currentPage,
          limit: this.pageSize,
          sortBy: this.getSortField(),
          sortOrder: this.getSortOrder(),
          // Add filters as needed
          authorName: null,
          genre: null,
          title: null,
          readStatus: null,
        }
      );

      this.books = response.books;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch books:", error);
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
    this.fetchBooks();
  }

  private getSortField(): string {
    // Implement your sort field logic based on sortIndex
    return "title"; // Example
  }

  private getSortOrder(): string {
    // Implement your sort order logic based on sortIndex
    return "asc"; // Example
  }
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx
