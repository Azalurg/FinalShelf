import { Component, Input, Output, EventEmitter, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { FormsModule } from "@angular/forms";
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
  imports: [CommonModule, RouterModule, FormsModule],
  templateUrl: "./generic-list.component.html",
  styleUrls: ["./generic-list.component.scss"],
})
export class GenericListComponent implements OnInit {
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

  getSrc: (path: string) => string = () => "";
  absolutePath = "";
  isLoaded = false;

  ngOnInit(): void {
    getCurrentAbsolutePath().then((path) => {
      this.absolutePath = path;
      this.isLoaded = true;

      this.getSrc =
        this.listType === "authors"
          ? (p: string) => convertImgPathAuthor(p, this.absolutePath)
          : (p: string) => convertImgPathBook(p, this.absolutePath);
    });
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
