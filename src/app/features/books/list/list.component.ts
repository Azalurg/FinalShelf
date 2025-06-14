import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { Book, BookListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

// Helper type for clarity, though you can also do this inline.
type SortOptionKey = keyof typeof BooksListPageComponent.prototype.sortObject;

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
  totalPages = 1;
  totalCount = 0;
  sortObject = {
    "Author ^": ["author", "asc"],
    "Author v": ["author", "desc"],
    "Title ^": ["title", "asc"],
    "Title v": ["title", "desc"],
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
      console.log(
        "page:",
        this.currentPage,
        "limit:",
        this.pageSize,
        "sortBy:",
        this.getSortField(),
        "sortOrder:",
        this.getSortOrder()
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
    this.sortIndex = sortIndex as keyof typeof this.sortObject;
    this.fetchBooks();
  }

  private getSortField(): string {
    const sort = this.sortObject[this.sortIndex];
    console.log(this.sortIndex);
    return sort[0];
  }

  private getSortOrder(): string {
    const sort = this.sortObject[this.sortIndex];
    return sort[1];
  }
}
