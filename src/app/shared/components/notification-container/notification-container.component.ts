import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { NotificationService } from "../../services/notification.service";

@Component({
  selector: "app-notification-container",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./notification-container.component.html",
  styleUrl: "./notification-container.component.scss",
})
export class NotificationContainerComponent {
  notifications$ = this.notificationService.notifications;

  constructor(private notificationService: NotificationService) {}

  dismiss(id: string): void {
    this.notificationService.dismiss(id);
  }
}

