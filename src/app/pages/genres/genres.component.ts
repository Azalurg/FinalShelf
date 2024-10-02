import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-genres',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './genres.component.html',
  styleUrl: './genres.component.scss'
})
export class GenresComponent {
  genres: any[] = [];
  ngOnInit(): void {
    this.fetchGenres();
  }
  
  async fetchGenres() {
    try {
      const genres = await invoke<any[]>('tauri_get_genres');
      this.genres = genres;
    } catch (error) {
      console.error(error);
    }
  }
}
