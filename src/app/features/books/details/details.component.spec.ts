import { ComponentFixture, TestBed } from "@angular/core/testing";
import { ActivatedRoute, convertToParamMap } from "@angular/router";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { of } from "rxjs";
import { BookDetailsPageComponent } from "./details.component";
import { Book } from "../../../models/books";

describe("BookDetailsPageComponent", () => {
  let fixture: ComponentFixture<BookDetailsPageComponent>;
  let component: BookDetailsPageComponent;
  let ipcSpy: jasmine.Spy;

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
    // Tauri's `invoke`/`listen` are read-only ES-module exports and cannot be
    // spied on directly; mockIPC installs a fake IPC layer on the window that
    // the real `invoke` delegates to.
    ipcSpy = jasmine.createSpy("ipc").and.resolveTo(undefined);
    mockIPC((cmd, args) => ipcSpy(cmd, args));

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

  afterEach(() => {
    clearMocks();
  });

  it("updates score and persists changes", async () => {
    component.bookDetails = { ...baseBook };

    await component.onScoreChange(8);

    expect(ipcSpy).toHaveBeenCalledWith(
      "update_book_command",
      jasmine.objectContaining({
        book: jasmine.objectContaining({ score: 8 }),
      }),
    );
    expect(component.bookDetails?.score).toBe(8);
  });

  it("reverts score when update fails", async () => {
    ipcSpy.and.rejectWith(new Error("failure"));
    component.bookDetails = { ...baseBook, score: 7 };

    await component.onScoreChange(3);

    expect(component.bookDetails?.score).toBe(7);
  });
});
