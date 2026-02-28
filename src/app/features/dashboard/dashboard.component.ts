import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import {
  convertImgPathAuthor,
  convertImgPathBook,
} from "../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../shared/utils/getCurrentAbsolutePath";
import { RouterModule } from "@angular/router";
import { GenericListComponent } from "../../shared/components/generic-list/generic-list.component";
import { Book } from "../../models/books";
import { AuthorListItem } from "../../models/authors";

interface DashboardData {
  books_count: number;
  read_books_count: number;
  authors_count: number;
  genres_count: number;
  lectors_count: number;
  new_books: Book[];
  top_authors: AuthorListItem[];
  top_books: Book[];
}

const EMPTY_DASHBOARD: DashboardData = {
  books_count: 0,
  read_books_count: 0,
  authors_count: 0,
  genres_count: 0,
  lectors_count: 0,
  new_books: [],
  top_authors: [],
  top_books: [],
};

@Component({
  selector: "app-dashboard",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./dashboard.component.html",
  styleUrl: "./dashboard.component.scss",
})
export class DashboardPageComponent implements OnInit {
  dashboardData: DashboardData = EMPTY_DASHBOARD;
  absolutePath = "";

  ngOnInit(): void {
    this.fetchDashboardData();

    getCurrentAbsolutePath().then((path) => {
      this.absolutePath = path;
    });
  }

  async fetchDashboardData(): Promise<void> {
    try {
      this.dashboardData = await invoke<DashboardData>("get_dashboard_data_command");
    } catch (error) {
      console.error("Failed to fetch dashboard data:", error);
    }
  }

  getAuthorSrc = (path: string) =>
    convertImgPathAuthor(path, this.absolutePath);

  getBookSrc = (path: string) => convertImgPathBook(path, this.absolutePath);
}
