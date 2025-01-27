import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { convertImgPathBook } from '../../common/convertImgPath';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";
import { BookListComponent } from "../books/book-list/book-list.component";
import { getCurrentAbsolutePath } from '../../common/getCurrentAbsolutePath';

@Component({
  selector: 'app-lectors-details',
  standalone: true,
  imports: [CommonModule, RouterModule, BookListComponent],
  templateUrl: './lectors-details.component.html',
  styleUrl: './lectors-details.component.scss'
})
export class LectorsDetailsComponent {
  lectorDetails: any;
  absolute_path: string = "";

   getSrcBook = (path: string, absolute_path: string) => convertImgPathBook(path, absolute_path);

  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const lectorName = params.get('name');
      if (lectorName) {
        this.fetchLectorDetails(lectorName);
      }
      
      getCurrentAbsolutePath().then((path) => {
        this.absolute_path = path;
      });
    });
  }

  async fetchLectorDetails(lectorName: string) {
    try {
      const lectorDetailsData = await invoke<any>('tauri_get_lector_details', { lectorId: lectorName });
      this.lectorDetails = lectorDetailsData;
      console.log(this.lectorDetails);
    } catch (error) {
      console.error(error);
    }
  }
}
