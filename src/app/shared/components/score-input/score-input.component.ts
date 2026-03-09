import { CommonModule } from "@angular/common";
import { Component, EventEmitter, Input, Output } from "@angular/core";

@Component({
  selector: "app-score-input",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./score-input.component.html",
  styleUrls: ["./score-input.component.scss"],
})
export class ScoreInputComponent {
  @Input() value = 0;
  @Input() max = 10;
  @Input() editable = true;
  @Output() valueChange = new EventEmitter<number>();

  hover = 0;

  get stars(): number[] {
    return Array.from({ length: this.max }, (_, idx) => idx + 1);
  }

  setScore(score: number): void {
    if (!this.editable) return;
    this.valueChange.emit(score);
  }

  clearScore(): void {
    if (!this.editable) return;
    this.valueChange.emit(0);
  }

  setHover(star: number): void {
    if (!this.editable) return;
    this.hover = star;
  }

  resetHover(): void {
    this.hover = 0;
  }
}
