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
      const authorId = Number(params.get('id'));
      if (authorId) {
        this.fetchAuthorDetails(authorId);
      }

      getCurrentAbsolutePath().then((path) => {
        this.absolute_path = path;
      });
    });
  }

  async fetchAuthorDetails(authorId: number) {
    try {
      const authorDetailsData = await invoke<AuthorDetails>('tauri_get_author_details', { authorId });
      this.authorDetails = authorDetailsData;
      console.log(this.authorDetails);
    } catch (error) {
      console.error(error);
    }
  }

}
