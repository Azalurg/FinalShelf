import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { convertImgPathBook } from '../../common/convertImgPath';
import { ActivatedRoute, RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-lectors-details',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './lectors-details.component.html',
  styleUrl: './lectors-details.component.scss'
})
export class LectorsDetailsComponent {
  lectorDetails: any;

  getSrcBook = (path: string) => convertImgPathBook(path);

  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.route.paramMap.subscribe(params => {
      const lectorId = Number(params.get('id'));
      if (lectorId) {
        this.fetchLectorDetails(lectorId);
      }
    });
  }

  async fetchLectorDetails(lectorId: number) {
    try {
      const lectorDetailsData = await invoke<any>('tauri_get_lector_details', { lectorId });
      this.lectorDetails = lectorDetailsData;
      console.log(this.lectorDetails);
    } catch (error) {
      console.error(error);
    }
  }
}
