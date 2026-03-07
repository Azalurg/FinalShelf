import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { SeriesWithBooks } from "../../../models/series";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-series-details",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./series-details.component.html",
  styleUrl: "./series-details.component.scss",
})
export class SeriesDetailsPageComponent implements OnInit {
  seriesDetails: SeriesWithBooks | null = null;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const seriesId = params.get("id");
      if (seriesId) {
        this.fetchSeriesDetails(parseInt(seriesId, 10));
      }
    });
  }

  async fetchSeriesDetails(seriesId: number): Promise<void> {
    try {
      this.seriesDetails = await invoke<SeriesWithBooks>("get_series_command", {
        id: seriesId,
      });
    } catch (error) {
      console.error("Failed to fetch series details:", error);
    }
  }

  getBooksInOrder() {
    if (!this.seriesDetails) return [];
    // Books are already ordered by series_order from the backend
    return this.seriesDetails.books;
  }
}
