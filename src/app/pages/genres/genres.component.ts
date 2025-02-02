import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";
import { Genre } from '../../models/genres';

@Component({
  selector: 'app-genres',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './genres.component.html',
  styleUrl: './genres.component.scss'
})
export class GenresComponent {
  genres: Genre[] = [];
  ngOnInit(): void {
    this.fetchGenres();
  }
  
  async fetchGenres() {
    try {
      const genres = await invoke<Genre[]>('get_genres_list_command');
      this.genres = genres;
    } catch (error) {
      console.error(error);
    }
  }
}
