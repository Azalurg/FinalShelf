import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { Lector } from "../../../models/lectors";

@Component({
  selector: "app-lectors",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./lectors.component.html",
  styleUrl: "./lectors.component.scss",
})
export class LectorsListPageComponent implements OnInit {
  lectors: Lector[] = [];

  ngOnInit(): void {
    this.fetchLectors();
  }

  async fetchLectors(): Promise<void> {
    try {
      const response = await invoke<{ items: Lector[] }>(
        "get_lectors_list_command",
        { params: { limit: 100, sort_by: "name", sort_order: "asc" } }
      );
      this.lectors = response.items;
    } catch (error) {
      console.error("Failed to fetch lectors:", error);
    }
  }
}
