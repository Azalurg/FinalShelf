import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css'
})
export class DashboardComponent {
  dashboardData: any

  ngOnInit(): void {
    const dashboardData = this.fetchDashboardData();
    if (dashboardData) {
      this.dashboardData = dashboardData;
    }
  }

  async fetchDashboardData() {
    try {
      const dashboardData = await invoke<any>('tauri_get_dashboard_data');
      this.dashboardData = dashboardData;
      console.log(this.dashboardData);
    } catch (error) {
      console.error(error);
    }
  }
}
