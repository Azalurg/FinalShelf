import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { RouterModule } from "@angular/router";
import { FormsModule } from "@angular/forms";
import { invoke } from "@tauri-apps/api/core";
import { Lector } from "../../../models/lectors";
import { ListResponse } from "../../../models/books";

@Component({
  selector: "app-lectors",
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule],
  templateUrl: "./lectors.component.html",
  styleUrl: "./lectors.component.scss",
})
export class LectorsListPageComponent implements OnInit {
  lectors: Lector[] = [];
  currentPage = 1;
  pageSize = 21;
  totalPages = 1;
  totalCount = 0;
  sortObject = {
    "Name ^": ["name", "asc"],
    "Name v": ["name", "desc"],
    "Books ^": ["books_count", "asc"],
    "Books v": ["books_count", "desc"],
  } as const;
  sortOptions = Object.keys(this.sortObject);
  sortIndex: keyof typeof this.sortObject = "Name ^";

  ngOnInit(): void {
    this.fetchLectors();
  }

  async fetchLectors(): Promise<void> {
    try {
      const response = await invoke<ListResponse<Lector>>(
        "get_lectors_list_command",
        {
          params: {
            page: this.currentPage,
            limit: this.pageSize,
            sort_by: this.sortObject[this.sortIndex][0],
            sort_order: this.sortObject[this.sortIndex][1],
          },
        }
      );
      this.lectors = response.items;
      this.totalPages = response.total_pages;
      this.totalCount = response.total_count;
    } catch (error) {
      console.error("Failed to fetch lectors:", error);
    }
  }

  onPageChange(newPage: number): void {
    this.currentPage = newPage;
    this.fetchLectors();
  }

  onPageSizeChange(newSize: number): void {
    this.pageSize = newSize;
    this.currentPage = 1;
    this.fetchLectors();
  }

  onSortChange(event: Event): void {
    this.sortIndex = (event.target as HTMLSelectElement).value as keyof typeof this.sortObject;
    this.fetchLectors();
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

  changePageSize(event: Event): void {
    const size = +(event.target as HTMLSelectElement).value;
    this.onPageSizeChange(size);
  }
}
