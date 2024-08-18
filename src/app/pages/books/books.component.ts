import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { CommonModule } from '@angular/common';
import { Book } from '../../models/books';
import { convertImgPath } from '../../common/convertImgPath';

@Component({
  selector: 'app-books',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './books.component.html',
  styleUrl: './books.component.scss'
})
export class BooksComponent {
  books: Book[] = [];
  getSrc = (path: string) => convertImgPath(path);
  ngOnInit(): void {
    this.fetchBooks();
  }

  async fetchBooks() {
    try {
      const books = await invoke<Book[]>('tauri_get_books');
      this.books = books;
    } catch (error) {
      console.error(error);
    }
  } 
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx