import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { ActivatedRoute, Router, UrlSegment } from '@angular/router';

@Component({
  selector: 'app-toolbar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './toolbar.component.html',
  styleUrl: './toolbar.component.scss'
})
export class ToolbarComponent {
  breadcrumbs: { name: string, url: string }[] = [];
  currentTime: string = '';

  private intervalId: any;

  ngOnInit(): void {
    this.generateBreadcrumbs();
    this.updateTime();
    this.intervalId = setInterval(() => this.updateTime(), 5000);
  }

  ngOnDestroy(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId); // Clear interval when the component is destroyed
    }
  }

  generateBreadcrumbs(): void {
    const urlSegments = window.location.pathname.split('/').filter(segment => segment);
    let fullUrl = '';

    this.breadcrumbs = urlSegments.map((segment, index) => {
      fullUrl += `/${segment}`;
      return {
        name: segment,
        url: fullUrl
      };
    });
  }

  private updateTime(): void {
    const now = new Date();
    this.currentTime = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
}
