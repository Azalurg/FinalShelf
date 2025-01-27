import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { convertImgPathAuthor } from '../../common/convertImgPath';
import { invoke } from "@tauri-apps/api/core";
import { Author } from '../../models/authors';
import { RouterModule } from '@angular/router';
import { getCurrentAbsolutePath } from '../../common/getCurrentAbsolutePath';

@Component({
  selector: 'app-authors',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './authors.component.html',
  styleUrl: './authors.component.scss'
})
export class AuthorsComponent {
  authors: Author[] = [];
  absolute_path: string = "";
  
  getSrc = (path: string) => convertImgPathAuthor(path, this.absolute_path);

  ngOnInit(): void {
    this.fetchAuthors();
    
    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
    });
  }

  async fetchAuthors() {
    try {
      const authors = await invoke<Author[]>('get_authors_list_command', {});
      this.authors = authors;
    } catch (error) {
      console.error(error);
    }
  }
}
