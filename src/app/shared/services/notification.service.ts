import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";

export interface AppNotification {
  id: string;
  message: string;
  type: "success" | "error" | "info" | "warning";
  duration?: number;
}

@Injectable({
  providedIn: "root",
})
export class NotificationService {
  private notifications$ = new BehaviorSubject<AppNotification[]>([]);
  readonly notifications = this.notifications$.asObservable();

  private generateId(): string {
    return crypto.randomUUID();
  }

  show(
    message: string,
    type: AppNotification["type"] = "info",
    duration = 5000
  ): void {
    const notification: AppNotification = {
      id: this.generateId(),
      message,
      type,
      duration,
    };

    this.notifications$.next([...this.notifications$.value, notification]);

    if (duration > 0) {
      setTimeout(() => {
        this.dismiss(notification.id);
      }, duration);
    }
  }

  success(message: string, duration = 5000): void {
    this.show(message, "success", duration);
  }

  error(message: string, duration = 7000): void {
    this.show(message, "error", duration);
  }

  info(message: string, duration = 5000): void {
    this.show(message, "info", duration);
  }

  warning(message: string, duration = 6000): void {
    this.show(message, "warning", duration);
  }

  dismiss(id: string): void {
    const current = this.notifications$.value;
    this.notifications$.next(current.filter((n) => n.id !== id));
  }

  dismissAll(): void {
    this.notifications$.next([]);
  }
}
