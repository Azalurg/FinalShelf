import { Component, OnDestroy, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { ActivatedRoute } from "@angular/router";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { Book, BookListResponse } from "../../models/books";
import { Subscription } from "rxjs";
import { GenericListComponent } from "../../shared/components/generic-list/generic-list.component";

interface SearchField {
  key: string;
  label: string;
  enabled: boolean;
}

@Component({
  selector: "app-search",
  standalone: true,
  imports: [CommonModule, FormsModule, GenericListComponent],
  templateUrl: "./search.component.html",
  styleUrl: "./search.component.scss",
})
export class SearchPageComponent implements OnInit, OnDestroy {
  books: Book[] = [];
  searchQuery: string = "";
  totalCount = 0;
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;

  searchFields: SearchField[] = [
    { key: "title", label: "Title", enabled: true },
    { key: "author", label: "Author", enabled: true },
    { key: "genre", label: "Genre", enabled: true },
    { key: "lector", label: "Lector", enabled: true },
  ];

  sortObject = {
    "Title ^": ["title", "asc"],
    "Title v": ["title", "desc"],
    "Author ^": ["author_name", "asc"],
    "Author v": ["author_name", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Title ^";

  private routeSub!: Subscription;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.routeSub = this.route.queryParams.subscribe((params) => {
      this.searchQuery = params["query"] || "";
      this.currentPage = 1;
      this.searchBooks();
    });
  }

  ngOnDestroy(): void {
    if (this.routeSub) {
      this.routeSub.unsubscribe();
    }
  }

  get activeFields(): string[] {
    return this.searchFields.filter((f) => f.enabled).map((f) => f.key);
  }

  toggleField(field: SearchField): void {
    const enabledCount = this.searchFields.filter((f) => f.enabled).length;
    if (field.enabled && enabledCount <= 1) return;
    field.enabled = !field.enabled;
    this.currentPage = 1;
    this.searchBooks();
  }

  async searchBooks(): Promise<void> {
    if (!this.searchQuery.trim()) {
      this.books = [];
      this.totalCount = 0;
      this.totalPages = 1;
      return;
    }
    try {
      const [sortBy, sortOrder] = this.sortObject[this.sortIndex];
      const response = await invoke<BookListResponse>("search_command", {
        target: this.searchQuery,
        by: this.activeFields,
        page: this.currentPage,
        limit: this.pageSize,
        sort_by: sortBy,
        sort_order: sortOrder,
      });
      this.books = response.items;
      this.totalCount = response.total_count;
      this.totalPages = response.total_pages;
    } catch (error) {
      console.error(error);
      this.books = [];
      this.totalCount = 0;
    }
  }

  onPageChange(newPage: number): void {
    this.currentPage = newPage;
    this.searchBooks();
  }

  onPageSizeChange(newSize: number): void {
    this.pageSize = newSize;
    this.currentPage = 1;
    this.searchBooks();
  }

  onSortChange(sortKey: string): void {
    this.sortIndex = sortKey as keyof typeof this.sortObject;
    this.searchBooks();
  }
}
