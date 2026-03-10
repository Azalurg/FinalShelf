import { ComponentFixture, TestBed } from "@angular/core/testing";
import { ActivatedRoute, convertToParamMap } from "@angular/router";
import * as tauriCore from "@tauri-apps/api/core";
import { of } from "rxjs";
import { BookDetailsPageComponent } from "./details.component";
import { Book } from "../../../models/books";

describe("BookDetailsPageComponent", () => {
  let fixture: ComponentFixture<BookDetailsPageComponent>;
  let component: BookDetailsPageComponent;
  let invokeSpy: jasmine.Spy;

  const baseBook: Book = {
    title: "Sample Book",
    relative_cover_path: "",
    author_name: "Author",
    genre: "Genre",
    lector: "Lector",
    create_date: "2025-01-01",
    read: false,
    score: 5,
    duration_seconds: 120,
    series_id: null,
    series_order: null,
  };

  beforeEach(async () => {
    invokeSpy = spyOn(tauriCore, "invoke").and.returnValue(Promise.resolve());

    await TestBed.configureTestingModule({
      imports: [BookDetailsPageComponent],
      providers: [
        {
          provide: ActivatedRoute,
          useValue: { paramMap: of(convertToParamMap({})) },
        },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(BookDetailsPageComponent);
    component = fixture.componentInstance;
  });

  it("updates score and persists changes", async () => {
    component.bookDetails = { ...baseBook };

    await component.onScoreChange(8);

    expect(invokeSpy).toHaveBeenCalledWith("update_book_command", {
      book: jasmine.objectContaining({ score: 8 }),
    });
    expect(component.bookDetails?.score).toBe(8);
  });

  it("reverts score when update fails", async () => {
    invokeSpy.and.returnValue(Promise.reject(new Error("failure")));
    component.bookDetails = { ...baseBook, score: 7 };

    await component.onScoreChange(3);

    expect(component.bookDetails?.score).toBe(7);
  });
});
