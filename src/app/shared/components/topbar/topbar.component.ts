import { CommonModule } from "@angular/common";
import { Component, OnDestroy } from "@angular/core";
import { Router, RouterModule, NavigationEnd } from "@angular/router";
import { FormsModule } from "@angular/forms";
import { filter, Subscription } from "rxjs";

@Component({
  selector: "app-topbar",
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule], // Dodaj FormsModule
  templateUrl: "./topbar.component.html",
  styleUrl: "./topbar.component.scss",
})
export class TopbarComponent implements OnDestroy {
  navPaths: { name: string; url: string }[] = [];
  currentTime: string = "";
  searchTerm: string = "";

  private intervalId: any;
  private routerSubscription: Subscription;

  constructor(private router: Router) {
    this.routerSubscription = this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe(() => this.generateNavPaths());
  }

  ngOnInit(): void {
    this.generateNavPaths();
    this.updateTime();
    this.intervalId = setInterval(() => this.updateTime(), 5000);
  }

  ngOnDestroy(): void {
    if (this.intervalId) clearInterval(this.intervalId);
    this.routerSubscription.unsubscribe();
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
    if (this.searchTerm.trim()) {
      this.router.navigate(["/search"], {
        queryParams: { query: this.searchTerm.trim() },
      });
      this.searchTerm = "";
    }
  }
}
