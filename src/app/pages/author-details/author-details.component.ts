import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { AuthorDetails } from '../../models/authors';
import { convertImgPathAuthor, convertImgPathBook } from '../../common/convertImgPath';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";
import { getCurrentAbsolutePath } from '../../common/getCurrentAbsolutePath';

@Component({
  selector: 'app-author-details',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './author-details.component.html',
  styleUrl: './author-details.component.scss'
})
export class AuthorDetailsComponent {
  authorDetails: AuthorDetails | any;
  absolute_path: string = "";

  getSrcAuthor = (path: string) => convertImgPathAuthor(path, this.absolute_path);
  getSrcBook = (path: string) => convertImgPathBook(path, this.absolute_path);

  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const authorName = params.get('name');
      if (authorName) {
        this.fetchAuthorDetails(authorName);
      }

      getCurrentAbsolutePath().then((path) => {
        this.absolute_path = path;
      });
    });
  }

  async fetchAuthorDetails(authorName: string) {
    try {
      const authorDetailsData = await invoke<AuthorDetails>('get_author_command', { name: authorName });
      this.authorDetails = authorDetailsData;
      console.log(this.authorDetails);
    } catch (error) {
      console.error(error);
    }
  }

}
