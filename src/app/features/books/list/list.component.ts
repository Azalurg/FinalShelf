import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { Book, BookListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";
import { Author } from "../../../models/authors";
import { Genre } from "../../../models/genres";
import { Lector } from "../../../models/lectors";

interface BookFilters {
  author_name: string | null;
  genre: string | null;
  lector: string | null;
  read_status: boolean | null;
}

@Component({
  selector: "app-books",
  standalone: true,
  imports: [CommonModule, GenericListComponent, FormsModule],
  templateUrl: "./list.component.html",
  styleUrl: "./list.component.scss",
})
export class BooksListPageComponent implements OnInit {
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
    "Date ^": ["create_date", "asc"],
    "Date v": ["create_date", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Title ^";

  // Filter options
  authors: Author[] = [];
  genres: Genre[] = [];
  lectors: Lector[] = [];

  // Active filters
  filters: BookFilters = {
    author_name: null,
    genre: null,
    lector: null,
    read_status: null,
  };

  showFilters = false;

  ngOnInit(): void {
    this.fetchFilterOptions();
    this.fetchBooks();
  }

  async fetchFilterOptions(): Promise<void> {
    try {
      const [authors, genres, lectors] = await Promise.all([
        invoke<Author[]>("get_authors_list_command", { params: {} }),
        invoke<Genre[]>("get_genres_list_command"),
        invoke<Lector[]>("get_lectors_list_command"),
      ]);
      this.authors = authors;
      this.genres = genres;
      this.lectors = lectors;
    } catch (error) {
      console.error("Failed to fetch filter options:", error);
    }
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
            author_name: this.filters.author_name,
            genre: this.filters.genre,
            lector: this.filters.lector,
            read_status: this.filters.read_status,
          },
        }
      );
      this.books = response.items;
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

  onFilterChange(): void {
    this.currentPage = 1;
    this.fetchBooks();
  }

  clearFilters(): void {
    this.filters = {
      author_name: null,
      genre: null,
      lector: null,
      read_status: null,
    };
    this.currentPage = 1;
    this.fetchBooks();
  }

  toggleFilters(): void {
    this.showFilters = !this.showFilters;
  }

  get hasActiveFilters(): boolean {
    return !!(
      this.filters.author_name ||
      this.filters.genre ||
      this.filters.lector ||
      this.filters.read_status !== null
    );
  }

  private getSortField(): string {
    const sort = this.sortObject[this.sortIndex];
    return sort[0];
  }

  private getSortOrder(): string {
    const sort = this.sortObject[this.sortIndex];
    return sort[1];
  }
}
