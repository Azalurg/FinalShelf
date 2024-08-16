import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  books: { title: string, genre: string }[] = [];

  async scanDirectory(): Promise<void> {
    try {
      const directory = prompt('Enter directory to scan:');
      if (directory) {
        await invoke('tauri_scan', { directory });
        alert('Scan completed successfully!');
      }
    } catch (error) {
      console.error('Error:', error);
      alert('Failed to scan metadata: ' + (error as Error).message);
    }
  }
 
  async getBooks(): Promise<void> {
    try {
      const books = await invoke<{ title: string, genre: string }[]>('tauri_get_books');
      this.books = books;
    } catch (error) {
      console.error('Error:', error);
      alert('Failed to retrieve books: ' + (error as Error).message);
    }
  }
}
