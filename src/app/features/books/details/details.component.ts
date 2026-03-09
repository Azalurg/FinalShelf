import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { ScoreInputComponent } from "../../../shared/components/score-input/score-input.component";
import { convertImgPathBook } from "../../../shared/utils/convertImgPath";
import { getCurrentAbsolutePath } from "../../../shared/utils/getCurrentAbsolutePath";
import { Book } from "../../../models/books";
import { Series } from "../../../models/series";

@Component({
  selector: "app-book-details",
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule, ScoreInputComponent],
  templateUrl: "./details.component.html",
})
export class BookDetailsPageComponent implements OnInit {
  bookDetails: Book | null = null;
  absolutePath = "";
  
  // Series-related properties
  currentSeries: Series | null = null;
  availableSeries: Series[] = [];
  selectedSeriesId: number | null = null;
  newSeriesOrder: number | null = null;
  showSeriesEditor = false;

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
    // Reset state to avoid stale data when navigating between books
    this.currentSeries = null;
    this.showSeriesEditor = false;
    
    try {
      this.bookDetails = await invoke<Book>("get_book_command", {
        title: bookTitle,
      });
      
      // Fetch available series for this author first
      if (this.bookDetails.author_name) {
        await this.fetchAvailableSeries(this.bookDetails.author_name);
      }
      
      // Derive current series from available series (avoids extra get_series_command call)
      if (this.bookDetails.series_id) {
        this.currentSeries = this.availableSeries.find(
          (s) => s.id === this.bookDetails!.series_id
        ) ?? null;
      }
    } catch (error) {
      console.error("Failed to fetch book details:", error);
    }
  }

  async fetchAvailableSeries(authorName: string): Promise<void> {
    try {
      this.availableSeries = await invoke<Series[]>("get_series_by_author_command", {
        authorName,
      });
    } catch (error) {
      console.error("Failed to fetch available series:", error);
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

  async onScoreChange(score: number): Promise<void> {
    if (!this.bookDetails) return;

    const previousScore = this.bookDetails.score;
    this.bookDetails.score = score;
    try {
      await invoke("update_book_command", { book: this.bookDetails });
    } catch (error) {
      console.error("Failed to update book score:", error);
      this.bookDetails.score = previousScore;
    }
  }

  toggleSeriesEditor(): void {
    this.showSeriesEditor = !this.showSeriesEditor;
    if (this.showSeriesEditor && this.bookDetails) {
      this.selectedSeriesId = this.bookDetails.series_id;
      this.newSeriesOrder = this.bookDetails.series_order;
    }
  }

  async assignToSeries(): Promise<void> {
    if (!this.bookDetails) return;
    
    // Ensure series_order is null when series_id is null
    const seriesId = this.selectedSeriesId ?? null;
    const seriesOrder = seriesId ? this.newSeriesOrder : null;
    
    try {
      await invoke("assign_book_to_series_command", {
        bookTitle: this.bookDetails.title,
        seriesId: seriesId,
        seriesOrder: seriesOrder,
      });
      
      // Update local state
      this.bookDetails.series_id = seriesId;
      this.bookDetails.series_order = seriesOrder;
      
      // Derive current series from already-loaded availableSeries
      this.currentSeries = seriesId
        ? (this.availableSeries.find((s) => s.id === seriesId) ?? null)
        : null;
      
      this.showSeriesEditor = false;
    } catch (error) {
      console.error("Failed to assign book to series:", error);
    }
  }

  async removeFromSeries(): Promise<void> {
    if (!this.bookDetails) return;
    
    try {
      await invoke("assign_book_to_series_command", {
        bookTitle: this.bookDetails.title,
        seriesId: null,
        seriesOrder: null,
      });
      
      this.bookDetails.series_id = null;
      this.bookDetails.series_order = null;
      this.currentSeries = null;
      this.showSeriesEditor = false;
    } catch (error) {
      console.error("Failed to remove book from series:", error);
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
