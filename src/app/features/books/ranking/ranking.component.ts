import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { Book, BookListResponse } from "../../../models/books";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-ranking",
  standalone: true,
  imports: [CommonModule, GenericListComponent],
  templateUrl: "./ranking.component.html",
  styleUrl: "./ranking.component.scss",
})
export class RankingPageComponent implements OnInit {
  books: Book[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;
  totalCount = 0;

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
            sort_by: "score",
            sort_order: "desc",
            min_score: 1,
          },
        }
      );
      this.books = response.items;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch ranking:", error);
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
}
