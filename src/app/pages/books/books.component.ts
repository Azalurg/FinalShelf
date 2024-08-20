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
  getSrc = (path: string) => convertImgPathBook(path);
  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks() {
    try {
      const books = await invoke<Book[]>('tauri_get_books', {page: 0, pageSize: 10});
      this.books = books;
    } catch (error) {
      console.error(error);
    }
  } 
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx