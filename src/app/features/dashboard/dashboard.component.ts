import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import {
  convertImgPathAuthor,
  convertImgPathBook,
} from "../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../shared/utils/getCurrentAbsolutePath";
import { RouterModule } from "@angular/router";
import { GenericListComponent } from "../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-dashboard",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./dashboard.component.html",
  styleUrl: "./dashboard.component.scss",
})
export class DashboardPageComponent {
  dashboardData: any;
  absolute_path: any;

  ngOnInit(): void {
    const dashboardData = this.fetchDashboardData();

    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
    });

    if (dashboardData) {
      this.dashboardData = dashboardData;
    }
  }

  async fetchDashboardData() {
    try {
      const dashboardData = await invoke<any>("get_dashboard_data_command");
      this.dashboardData = dashboardData;
      console.log(this.dashboardData);
    } catch (error) {
      console.error(error);
    }
  }

  getAuthorSrc = (path: string) =>
    convertImgPathAuthor(path, this.absolute_path);

  getBookSrc = (path: string) => convertImgPathBook(path, this.absolute_path);
}
