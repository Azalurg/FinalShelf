import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { LectorDetails } from "../../../models/lectors";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-lectors-details",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./lectors-details.component.html",
  styleUrl: "./lectors-details.component.scss",
})
export class LectorsDetailsPageComponent implements OnInit {
  lectorDetails: LectorDetails | null = null;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const lectorName = params.get("name");
      if (lectorName) {
        this.fetchLectorDetails(lectorName);
      }
    });
  }

  async fetchLectorDetails(lectorName: string): Promise<void> {
    try {
      this.lectorDetails = await invoke<LectorDetails>(
        "get_lector_command",
        { lectorName },
      );
    } catch (error) {
      console.error("Failed to fetch lector details:", error);
    }
  }
}
