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
  styleUrl: './books.component.scss'
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

  convertPath(path: string): string {
    if (!path){
      return 'assets/logo.svg';
    }
    return convertFileSrc(path);
    
  }

  ngOnInit(): void {
    this.fetchBooks();
  }
}

// https://github.com/sprout2000/tauview/blob/main/src/Grid.tsx