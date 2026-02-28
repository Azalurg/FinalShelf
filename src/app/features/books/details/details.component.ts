import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { convertImgPathBook } from "../../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../../shared/utils/getCurrentAbsolutePath";
import { Book } from "../../../models/books";

@Component({
  selector: "app-book-details",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./details.component.html",
})
export class BookDetailsPageComponent implements OnInit {
  bookDetails: Book | any;
  absolute_path = "";

  getSrc = (path: string) => convertImgPathBook(path, this.absolute_path);

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const bookTitle = params.get("title");
      if (bookTitle) {
        this.fetchBookDetails(bookTitle);
      }
    });

    getCurrentAbsolutePath().then((path) => {
      this.absolute_path = path;
    });
  }

  async fetchBookDetails(bookTitle: string) {
    try {
      const bookDetails = await invoke<Book>("get_book_command", {
        title: bookTitle,
      });
      this.bookDetails = bookDetails;
      console.log(this.bookDetails);
    } catch (error) {
      console.error(error);
    }
  }

  async markAsRead() {
    if (this.bookDetails) {
      this.bookDetails.read = !this.bookDetails.read;
      try {
        await invoke("update_book_command", { book: this.bookDetails });
        console.log("Book updated successfully");
      } catch (error) {
        console.error("Error updating book:", error);
      }
    }
  }

  formatDuration(totalSeconds: number): string {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }
}
