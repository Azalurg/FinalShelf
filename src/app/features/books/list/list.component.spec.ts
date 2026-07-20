import { ComponentFixture, TestBed } from "@angular/core/testing";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { BooksListPageComponent } from "./list.component";
import { BookListResponse } from "../../../models/books";

describe("BooksListPageComponent", () => {
  let fixture: ComponentFixture<BooksListPageComponent>;
  let component: BooksListPageComponent;
  let ipcSpy: jasmine.Spy;

  const mockBookResponse: BookListResponse = {
    items: [
      {
        title: "Book A",
        relative_cover_path: "",
        author_name: "Author A",
        genre: "Mystery",
        lector: "Lector 1",
        create_date: "2025-01-01",
        read: true,
        score: 8,
        duration_seconds: 3600,
        series_id: null,
        series_order: null,
      },
    ],
    total_count: 1,
    page: 1,
    limit: 21,
    total_pages: 1,
  };

  const emptyListResponse = {
    items: [],
    total_count: 0,
    page: 1,
    limit: 0,
    total_pages: 0,
  };

  beforeEach(async () => {
    // Route each command through a jasmine spy via Tauri's mock IPC layer so we
    // can both stub responses and assert on the calls the component makes.
    ipcSpy = jasmine.createSpy("ipc").and.callFake((cmd: string) => {
      if (cmd === "get_books_list_command") {
        return Promise.resolve(mockBookResponse);
      }
      if (
        cmd === "get_authors_list_command" ||
        cmd === "get_genres_list_command" ||
        cmd === "get_lectors_list_command"
      ) {
        return Promise.resolve(emptyListResponse);
      }
      return Promise.resolve(null);
    });
    mockIPC((cmd, args) => ipcSpy(cmd, args));

    await TestBed.configureTestingModule({
      imports: [BooksListPageComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(BooksListPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
    await fixture.whenStable();
  });

  afterEach(() => {
    clearMocks();
  });

  it("fetches books with filters and resets the page", async () => {
    component.currentPage = 3;
    component.filters.author_name = "Author A";

    component.onFilterChange();
    await fixture.whenStable();

    const bookCalls = ipcSpy.calls
      .all()
      .filter((call) => call.args[0] === "get_books_list_command");
    const lastCall = bookCalls[bookCalls.length - 1];
    const params = lastCall.args[1].params;

    expect(params.author_name).toBe("Author A");
    expect(params.page).toBe(1);
    expect(component.books.length).toBe(1);
    expect(component.totalCount).toBe(mockBookResponse.total_count);
  });

  it("clears filters and removes the active flag", async () => {
    component.filters = {
      author_name: "Author B",
      genre: "Drama",
      lector: "Lector 2",
      read_status: true,
    };

    component.clearFilters();
    await fixture.whenStable();

    expect(component.filters).toEqual({
      author_name: null,
      genre: null,
      lector: null,
      read_status: null,
    });
    expect(component.hasActiveFilters).toBeFalse();
  });
});
