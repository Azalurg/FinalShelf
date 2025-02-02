import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";
import { Lector } from '../../models/lectors';

@Component({
  selector: 'app-lectors',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './lectors.component.html',
  styleUrl: './lectors.component.scss'
})
export class LectorsComponent {
  lectors: Lector[] = [];
  ngOnInit(): void {
    this.fetchLectors();
  }

  async fetchLectors() {
    try {
      const lectors = await invoke<Lector[]>('get_lectors_list_command');
      this.lectors = lectors;
    } catch (error) {
      console.error(error);
    }
  }

}
