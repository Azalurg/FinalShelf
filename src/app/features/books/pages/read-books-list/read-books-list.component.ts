import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { BookListComponent } from "../../components/book-list/book-list.component";
import { Book } from "../../../../models/books";

@Component({
  selector: "app-read-books-list",
  standalone: true,
  imports: [CommonModule, BookListComponent],
  templateUrl: "../list/list.component.html",
  styleUrl: "../list/list.component.scss",
})
export class ReadBooksListPageComponent {
  books: Book[] = [];
  page = 0;
  limit = 21;
  sortBy = "title";
  sortOrder = "asc";

  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks(): Promise<void> {
    try {
      const books = await invoke<Book[]>("get_all_read_books_command", {
        page: this.page + 1,
        limit: this.limit,
        sortBy: this.sortBy,
        sortOrder: this.sortOrder,
      });
      this.books = books;
      console.log(this.books);
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

    this.limit = newPageSize;
    this.page = 0; // Reset to the first page whenever the page size changes
    await this.fetchBooks();
  }

  async changeSortOrder(event: Event): Promise<void> {
    const selectElement = event.target as HTMLSelectElement;
    const value = parseInt(selectElement.value, 0);
    if (value === 0) {
      this.sortBy = "author_name";
      this.sortOrder = "asc";
    }
    if (value === 1) {
      this.sortBy = "title";
      this.sortOrder = "asc";
    }
    if (value === 2) {
      this.sortBy = "title";
      this.sortOrder = "desc";
    }
    if (value === 3) {
      this.sortBy = "author_name";
      this.sortOrder = "asc";
    }
    if (value === 4) {
      this.sortBy = "author_name";
      this.sortOrder = "desc";
    }
    await this.fetchBooks();
  }
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx
