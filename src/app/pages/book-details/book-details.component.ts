import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { BookDetails } from '../../models/books';
import { CommonModule } from '@angular/common';
import { convertImgPath } from '../../common/convertImgPath';

@Component({
  selector: 'app-book-details',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './book-details.component.html',
  styleUrls: ['./book-details.component.scss']
})
export class BookDetailsComponent implements OnInit {
  bookId: number | null = null;
  bookDetails: BookDetails | any;

  getSrc = (path: string) => convertImgPath(path);


  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      this.bookId = Number(params.get('id'));
      if (this.bookId) {
        this.fetchBookDetails(this.bookId);
      }
    });
  }

  async fetchBookDetails(bookId: number) {
    try {
      const bookDetails = await invoke<BookDetails>('tauri_get_book_details', { bookId });
      this.bookDetails = bookDetails;
      console.log(this.bookDetails);
    } catch (error) {
      console.error(error);
    }
  }
    
}
