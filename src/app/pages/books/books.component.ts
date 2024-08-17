import { Component } from '@angular/core';
import Book from '../../models/books';
import { invoke } from '@tauri-apps/api/tauri';
import { CommonModule } from '@angular/common';
import { appDataDir, join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';

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
    console.log(convertFileSrc("/home/azalurg/Obrazy/pobrane (6)_out.png"));
  }
}
