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
  bookDetails: Book | null = null;
  absolutePath = "";

  getSrc = (path: string) => convertImgPathBook(path, this.absolutePath);

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const bookTitle = params.get("title");
      if (bookTitle) {
        this.fetchBookDetails(bookTitle);
      }
    });

    getCurrentAbsolutePath().then((path) => {
      this.absolutePath = path;
    });
  }

  async fetchBookDetails(bookTitle: string): Promise<void> {
    try {
      this.bookDetails = await invoke<Book>("get_book_command", {
        title: bookTitle,
      });
    } catch (error) {
      console.error("Failed to fetch book details:", error);
    }
  }

  async markAsRead(): Promise<void> {
    if (this.bookDetails) {
      this.bookDetails.read = !this.bookDetails.read;
      try {
        await invoke("update_book_command", { book: this.bookDetails });
      } catch (error) {
        console.error("Failed to update book:", error);
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
