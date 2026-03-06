import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { RouterModule } from "@angular/router";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { SeriesListItem, SeriesListResponse } from "../../../models/series";

@Component({
  selector: "app-series",
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule],
  templateUrl: "./series.component.html",
  styleUrl: "./series.component.scss",
})
export class SeriesListPageComponent implements OnInit {
  seriesList: SeriesListItem[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;
  totalCount = 0;
  sortObject = {
    "Name ^": ["name", "asc"],
    "Name v": ["name", "desc"],
    "Author ^": ["author_name", "asc"],
    "Author v": ["author_name", "desc"],
    "Books ^": ["books_count", "asc"],
    "Books v": ["books_count", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Name ^";

  ngOnInit(): void {
    this.fetchSeries();
  }

  async fetchSeries(): Promise<void> {
    try {
      const response = await invoke<SeriesListResponse>(
        "get_series_list_command",
        {
          params: {
            page: this.currentPage,
            limit: this.pageSize,
            sort_by: this.sortObject[this.sortIndex][0],
            sort_order: this.sortObject[this.sortIndex][1],
          },
        }
      );
      this.seriesList = response.items;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch series:", error);
    }
  }

  onPageChange(newPage: number): void {
    this.currentPage = newPage;
    this.fetchSeries();
  }

  onSortChange(event: Event): void {
    this.sortIndex = (event.target as HTMLSelectElement).value as keyof typeof this.sortObject;
    this.fetchSeries();
  }

  changePageSize(event: Event): void {
    this.pageSize = parseInt((event.target as HTMLSelectElement).value, 10);
    this.currentPage = 1;
    this.fetchSeries();
  }

  prevPage(): void {
    if (this.currentPage > 1) {
      this.onPageChange(this.currentPage - 1);
    }
  }

  nextPage(): void {
    if (this.currentPage < this.totalPages) {
      this.onPageChange(this.currentPage + 1);
    }
  }
}
