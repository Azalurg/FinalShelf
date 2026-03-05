import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { NotificationService } from "../../services/notification.service";

@Component({
  selector: "app-notification-container",
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="notification-container">
      <div
        *ngFor="let notification of notifications$ | async"
        class="notification"
        [class.success]="notification.type === 'success'"
        [class.error]="notification.type === 'error'"
        [class.info]="notification.type === 'info'"
        [class.warning]="notification.type === 'warning'"
      >
        <span class="notification-message">{{ notification.message }}</span>
        <button
          class="notification-close"
          (click)="dismiss(notification.id)"
        >
          &times;
        </button>
      </div>
    </div>
  `,
  styles: [
    `
      .notification-container {
        position: fixed;
        top: 1rem;
        right: 1rem;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        max-width: 400px;
      }

      .notification {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1rem 1.25rem;
        border-radius: 0.5rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        animation: slideIn 0.3s ease;
        color: white;
      }

      @keyframes slideIn {
        from {
          transform: translateX(100%);
          opacity: 0;
        }
        to {
          transform: translateX(0);
          opacity: 1;
        }
      }

      .notification.success {
        background-color: #10b981;
      }

      .notification.error {
        background-color: #ef4444;
      }

      .notification.info {
        background-color: #3b82f6;
      }

      .notification.warning {
        background-color: #f59e0b;
      }

      .notification-message {
        flex: 1;
        margin-right: 1rem;
      }

      .notification-close {
        background: transparent;
        border: none;
        color: white;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0;
        line-height: 1;
        opacity: 0.7;
        transition: opacity 0.15s;
      }

      .notification-close:hover {
        opacity: 1;
      }
    `,
  ],
})
export class NotificationContainerComponent {
  notifications$ = this.notificationService.notifications;

  constructor(private notificationService: NotificationService) {}

  dismiss(id: string): void {
    this.notificationService.dismiss(id);
  }
}
