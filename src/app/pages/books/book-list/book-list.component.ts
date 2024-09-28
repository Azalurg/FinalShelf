import { Component, Input } from '@angular/core';
import { convertImgPathBook } from '../../../common/convertImgPath';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-book-list',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './book-list.component.html',
  styleUrl: './book-list.component.scss'
})
export class BookListComponent {
  @Input() books: any[] = [];
  getSrc = (path: string) => convertImgPathBook(path);
}
