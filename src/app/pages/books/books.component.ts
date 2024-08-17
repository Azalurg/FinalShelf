import { Component } from '@angular/core';
import Book from '../../models/books';
import { invoke } from '@tauri-apps/api/tauri';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-books',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './books.component.html',
  styleUrl: './books.component.css'
})
export class BooksComponent {
  books: Book[] = [];

  async fetchBooks() {
    try {
      const books = await invoke<Book[]>('tauri_get_books');
      this.books = books;
      console.log(this.books);
    } catch (error) {
      console.error(error);
    }
  }
  
  ngOnInit(): void {
    this.fetchBooks();
  }
}
