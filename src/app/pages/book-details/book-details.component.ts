import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api/core';
import { Book } from '../../models/books';
import { CommonModule } from '@angular/common';
import { convertImgPathBook } from '../../common/convertImgPath';
import { getCurrentAbsolutePath } from '../../common/getCurrentAbsolutePath';

@Component({
  selector: 'app-book-details',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './book-details.component.html',
  styleUrls: ['./book-details.component.scss']
})
export class BookDetailsComponent implements OnInit {
  bookDetails: Book | any;
  absolute_path: string = "";

  getSrc = (path: string) => convertImgPathBook(path, this.absolute_path);


  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
        const bookTitle =params.get('title');
      if (bookTitle) {
        this.fetchBookDetails(bookTitle);
      }
    });

    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
    });
  }

  async fetchBookDetails(bookTitle: string) {
    try {
      const bookDetails = await invoke<Book>('get_book_command', { title: bookTitle });
      this.bookDetails = bookDetails;
      console.log(this.bookDetails);
    } catch (error) {
      console.error(error);
    }
  }
    
}
