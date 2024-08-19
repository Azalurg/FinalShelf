import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { convertImgPathAuthor } from '../../common/convertImgPath';
import { invoke } from '@tauri-apps/api';
import { Author } from '../../models/authors';

@Component({
  selector: 'app-authors',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './authors.component.html',
  styleUrl: './authors.component.scss'
})
export class AuthorsComponent {
  authors: Author[] = [];
  getSrc = (path: string) => convertImgPathAuthor(path);
  ngOnInit(): void {
    this.fetchAuthors();
  }

  async fetchAuthors() {
    try {
      const authors = await invoke<Author[]>('tauri_get_authors');
      this.authors = authors;
    } catch (error) {
      console.error(error);
    }
  }
}
