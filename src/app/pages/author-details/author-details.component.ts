import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { AuthorDetails } from '../../models/authors';
import { convertImgPathAuthor } from '../../common/convertImgPath';
import { ActivatedRoute } from '@angular/router';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-author-details',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './author-details.component.html',
  styleUrl: './author-details.component.css'
})
export class AuthorDetailsComponent {
  authorDetails: AuthorDetails | any;

  getSrc = (path: string) => convertImgPathAuthor(path);
  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const authorId = Number(params.get('id'));
      if (authorId) {
        this.fetchAuthorDetails(authorId);
      }
    });
  }

  async fetchAuthorDetails(authorId: number) {
    try {
      const authorDetails = await invoke<AuthorDetails>('tauri_get_author_details', { authorId });
      this.authorDetails = authorDetails;
      console.log(this.authorDetails);
    } catch (error) {
      console.error(error);
    }
  }

}
