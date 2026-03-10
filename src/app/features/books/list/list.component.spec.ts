import { ComponentFixture, TestBed } from "@angular/core/testing";
import * as tauriCore from "@tauri-apps/api/core";
import { BooksListPageComponent } from "./list.component";
import { BookListResponse } from "../../../models/books";
import { AuthorListItem } from "../../../models/authors";
import { Genre } from "../../../models/genres";
import { Lector } from "../../../models/lectors";

describe("BooksListPageComponent", () => {
  let fixture: ComponentFixture<BooksListPageComponent>;
  let component: BooksListPageComponent;
  let invokeSpy: jasmine.Spy;

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
    items: [] as AuthorListItem[] | Genre[] | Lector[],
    total_count: 0,
    page: 1,
    limit: 0,
    total_pages: 0,
  };

  beforeEach(async () => {
    invokeSpy = spyOn(tauriCore, "invoke").and.callFake((command: string) => {
      if (command === "get_books_list_command") {
        return Promise.resolve(mockBookResponse);
      }
      if (
        command === "get_authors_list_command" ||
        command === "get_genres_list_command" ||
        command === "get_lectors_list_command"
      ) {
        return Promise.resolve(emptyListResponse);
      }
      return Promise.resolve(null);
    });

    await TestBed.configureTestingModule({
      imports: [BooksListPageComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(BooksListPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
    await fixture.whenStable();
  });

  it("fetches books with filters and resets the page", async () => {
    component.currentPage = 3;
    component.filters.author_name = "Author A";

    component.onFilterChange();
    await fixture.whenStable();

    const bookCalls = invokeSpy.calls
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
