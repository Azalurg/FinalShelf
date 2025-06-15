// generic-list.component.ts
import { Component, Input, Output, EventEmitter } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { getCurrentAbsolutePath } from "../../utils/getCurrentAbsolutePath";
import {
  convertImgPathAuthor,
  convertImgPathBook,
} from "../../utils/convertImgPath";

interface ListConfig {
  page: number;
  pageSize: number;
  totalPages: number;
  sortOptions: string[];
}

@Component({
  selector: "app-generic-list",
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule, ReactiveFormsModule],
  templateUrl: "./generic-list.component.html",
  styleUrls: ["./generic-list.component.scss"],
})
export class GenericListComponent {
  @Input() items: any[] = [];
  @Input() listType: "books" | "authors" = "books";
  @Input() config: ListConfig = {
    page: 1,
    pageSize: 21,
    totalPages: 1,
    sortOptions: ["Author ^", "Author v", "Title ^", "Title v"],
  };
  @Input() paginationEnabled = true;
  @Input() sortEnabled = true;

  @Output() pageChange = new EventEmitter<number>();
  @Output() pageSizeChange = new EventEmitter<number>();
  @Output() sortChange = new EventEmitter<string>();

  getSrc: (path: string) => string = (path) => path;
  absolute_path = "";
  isLoaded = true;

  constructor() {
    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
      this.isLoaded = true; // Mark as loaded once the path is available
    });
    if (this.listType === "books") {
      this.getSrc = (path: string) =>
        convertImgPathBook(path, this.absolute_path);
    }

    if (this.listType === "authors") {
      this.getSrc = (path: string) =>
        convertImgPathAuthor(path, this.absolute_path);
    }
  }

  prevPage(): void {
    if (this.config.page > 1) {
      this.pageChange.emit(this.config.page - 1);
    }
  }

  nextPage(): void {
    if (this.config.page < this.config.totalPages) {
      this.pageChange.emit(this.config.page + 1);
    }
  }

  changePageSize(event: Event): void {
    const size = +(event.target as HTMLSelectElement).value;
    this.pageSizeChange.emit(size);
  }

  changeSortOrder(event: Event): void {
    const order = (event.target as HTMLSelectElement).value;
    this.sortChange.emit(order);
  }
}
