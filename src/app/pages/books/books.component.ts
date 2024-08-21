import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { CommonModule } from "@angular/common";
import { Book } from "../../models/books";
import { BookListComponent } from "./book-list/book-list.component";

@Component({
  selector: "app-books",
  standalone: true,
  imports: [CommonModule, BookListComponent],
  templateUrl: "./books.component.html",
  styleUrl: "./books.component.scss",
})
export class BooksComponent {
  books: Book[] = [];
  page = 0;
  pageSize = 25;
  sortParams: any = "authors.name, title";
  sortOrder: any = "ASC";


  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks(): Promise<void> {
    try {
      const books = await invoke<Book[]>("tauri_get_books", {
        page: this.page,
        pageSize: this.pageSize,
        sortParams: this.sortParams,
        sortOrder: this.sortOrder,
      });
      this.books = books;
    } catch (error) {
      console.error(error);
    }
  }

  async nextPage(): Promise<void> {
    this.page++;
    await this.fetchBooks();
    if (this.books.length === 0) {
      this.page--;
      await this.fetchBooks();
    }
  }

  prevPage(): void {
    if (this.page > 0) {
      this.page--;
      this.fetchBooks();
    }
  }

  async changePageSize(event: Event): Promise<void> {
    const selectElement = event.target as HTMLSelectElement;
    const newPageSize = parseInt(selectElement.value, 10);

    this.pageSize = newPageSize;
    this.page = 0; // Reset to the first page whenever the page size changes
    await this.fetchBooks();
  }

  async changeSortOrder(event: Event): Promise<void> {
    const selectElement = event.target as HTMLSelectElement;
    const value = parseInt(selectElement.value, 0);
    if (value === 0) {
      this.sortParams = null;
      this.sortOrder = null;
    }
    if (value === 1) {
      this.sortParams = "title";
      this.sortOrder = "ASC";
    }
    if (value === 2) {
      this.sortParams = "title";
      this.sortOrder = "DESC";
    }
    if (value === 3) {
      this.sortParams = "authors.name, title";
      this.sortOrder = "ASC";
    }
    if (value === 4) {
      this.sortParams = "authors.name, title";
      this.sortOrder = "DESC";
    }
    await this.fetchBooks();
  }
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx
