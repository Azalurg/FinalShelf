import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { convertImgPathBook } from '../../common/convertImgPath';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';
import { BookListComponent } from "../books/book-list/book-list.component";

@Component({
  selector: 'app-genres-details',
  standalone: true,
  imports: [CommonModule, RouterModule, BookListComponent],
  templateUrl: './genres-details.component.html',
  styleUrl: './genres-details.component.scss'
})
export class GenresDetailsComponent {
  genreDetails: any;

  getSrcBook = (path: string) => convertImgPathBook(path);

  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const genreId = Number(params.get('id'));
      if (genreId) {
        this.fetchGenreDetails(genreId);
      }
    });
  }

  async fetchGenreDetails(genreId: number) {
    try {
      const genreDetailsData = await invoke<any>('tauri_get_genre_details', { genreId });
      this.genreDetails = genreDetailsData;
      console.log(this.genreDetails);
    } catch (error) {
      console.error(error);
    }
  }


}
