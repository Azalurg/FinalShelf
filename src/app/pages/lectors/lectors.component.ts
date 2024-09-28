import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-lectors',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './lectors.component.html',
  styleUrl: './lectors.component.scss'
})
export class LectorsComponent {
  lectors: any[] = [];
  ngOnInit(): void {
    this.fetchLectors();
  }

  async fetchLectors() {
    try {
      const lectors = await invoke<any[]>('tauri_get_lectors');
      this.lectors = lectors;
    } catch (error) {
      console.error(error);
    }
  }

}
