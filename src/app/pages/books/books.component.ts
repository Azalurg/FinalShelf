import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { CommonModule } from '@angular/common';
import { Book } from '../../models/books';
import { convertImgPathBook } from '../../common/convertImgPath';

@Component({
  selector: 'app-books',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './books.component.html',
  styleUrl: './books.component.scss'
})

export class BooksComponent {
  books: Book[] = [];
  page = 0;
  pageSize = 25;

  getSrc = (path: string) => convertImgPathBook(path);
  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks(): Promise<void> {
    try {
      const books = await invoke<Book[]>('tauri_get_books', {page: this.page, pageSize: this.pageSize});
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
    this.page = 0;  // Reset to the first page whenever the page size changes
    await this.fetchBooks();
}
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx