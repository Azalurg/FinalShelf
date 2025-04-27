import { Component, Input, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { Book } from "../../../../models/books";
import { convertImgPathBook } from "../../../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../../../shared/utils/getCurrentAbsolutePath";

@Component({
  selector: "app-book-list-component",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./book-list.component.html",
  styleUrls: ["./book-list.component.scss"],
})
export class BookListComponent implements OnInit {
  @Input() books: Book[] | undefined = [];
  absolute_path = "";
  isLoaded = false; // Flag to control loading state

  // Function to get the image source path
  getSrc = (path: string) => convertImgPathBook(path, this.absolute_path);

  ngOnInit(): void {
    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
      this.isLoaded = true; // Mark as loaded once the path is available
    });
  }
}
