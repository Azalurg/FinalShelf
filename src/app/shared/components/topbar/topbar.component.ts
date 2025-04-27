import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { Router, RouterModule } from "@angular/router";

@Component({
  selector: "app-topbar",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./topbar.component.html",
  styleUrl: "./topbar.component.scss",
})
export class TopbarComponent {
  navPaths: { name: string; url: string }[] = [];
  currentTime: string = "";

  private intervalId: any;

  constructor(private router: Router) {}

  ngOnInit(): void {
    this.generateNavPaths();
    this.updateTime();
    this.intervalId = setInterval(() => this.updateTime(), 5000);
  }

  ngOnDestroy(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId);
    }
  }

  generateNavPaths(): void {
    const urlSegments = this.router.url.split("/").filter((segment) => segment);
    let fullUrl = "";

    this.navPaths = urlSegments.map((segment, index) => {
      fullUrl += `/${segment}`;
      return {
        name: segment,
        url: fullUrl,
      };
    });
  }

  private updateTime(): void {
    const now = new Date();
    this.currentTime = now.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  searchIt() {
    this.router.navigate(["/search"]);
  }
}
