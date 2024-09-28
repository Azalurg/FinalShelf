import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';
import { BookDetails } from '../../models/books';
import { CommonModule } from '@angular/common';
import { convertImgPathBook } from '../../common/convertImgPath';

@Component({
  selector: 'app-book-details',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './book-details.component.html',
  styleUrls: ['./book-details.component.scss']
})
export class BookDetailsComponent implements OnInit {
  bookDetails: BookDetails | any;

  getSrc = (path: string) => convertImgPathBook(path);


  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const bookId = Number(params.get('id'));
      if (bookId) {
        this.fetchBookDetails(bookId);
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
