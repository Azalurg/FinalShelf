import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { convertImgPathBook } from "../../../shared/utils/convertImgPath";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentAbsolutePath } from "../../../shared/utils/getCurrentAbsolutePath";
import { LectorDetails } from "../../../models/lectors";
import { GenericListComponent } from "../../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-lectors-details",
  standalone: true,
  imports: [CommonModule, RouterModule, GenericListComponent],
  templateUrl: "./lectors-details.component.html",
  styleUrl: "./lectors-details.component.scss",
})
export class LectorsDetailsPageComponent {
  lectorDetails: LectorDetails | any;
  absolute_path = "";

  getSrcBook = (path: string, absolute_path: string) =>
    convertImgPathBook(path, absolute_path);

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      const lectorName = params.get("name");
      if (lectorName) {
        this.fetchLectorDetails(lectorName);
      } else {
        console.error("No lector name provided");
      }

      getCurrentAbsolutePath().then((path) => {
        this.absolute_path = path;
      });
    });
  }

  async fetchLectorDetails(lectorName: string) {
    console.log("fetching lector details");
    try {
      const lectorDetailsData = await invoke<LectorDetails>(
        "get_lector_command",
        { lectorName: lectorName }
      );
      this.lectorDetails = lectorDetailsData || [];
      console.log(this.lectorDetails);
    } catch (error) {
      console.error(error);
    }
  }
}
