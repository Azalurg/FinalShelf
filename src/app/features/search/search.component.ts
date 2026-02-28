import { Component, OnDestroy, OnInit } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { Book, BookListResponse } from "../../models/books";
import { Subscription } from "rxjs";
import { GenericListComponent } from "../../shared/components/generic-list/generic-list.component";

@Component({
  selector: "app-search",
  standalone: true,
  imports: [GenericListComponent],
  templateUrl: "./search.component.html",
  styleUrl: "./search.component.scss",
})
export class SearchPageComponent implements OnInit, OnDestroy {
  books: Book[] = [];
  searchQuery: string = "";
  private routeSub!: Subscription;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.routeSub = this.route.queryParams.subscribe((params) => {
      this.searchQuery = params["query"] || "";
      this.search_books();
    });
  }

  ngOnDestroy(): void {
    if (this.routeSub) {
      this.routeSub.unsubscribe();
    }
  }

  async search_books(): Promise<void> {
    if (!this.searchQuery.trim()) {
      this.books = [];
      return;
    }
    try {
      const response = await invoke<BookListResponse>("search_command", {
        target: this.searchQuery,
        by: ["title", "author", "genre", "lector"],
        limit: 100,
      });
      this.books = response.items;
    } catch (error) {
      console.error(error);
      this.books = [];
    }
  }
}
