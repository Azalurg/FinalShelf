import { Component } from "@angular/core";
import { BookListComponent } from "../books/book-list/book-list.component";
import { invoke } from "@tauri-apps/api";
import { Book } from "../../models/books";

@Component({
  selector: "app-search",
  standalone: true,
  imports: [BookListComponent],
  templateUrl: "./search.component.html",
  styleUrl: "./search.component.scss",
})
export class SearchComponent {
  searchResults: any[] = [];
  searchQuery: string = "";
  books: any[] = [];

  ngOnInit(): void {
    this.search_books();
  }

  async search_books(): Promise<void> {
    console.log("Voici les livres");
    const searchInput = document.getElementById("search_input");
    if (searchInput) {
      const search_query = (searchInput as HTMLInputElement).value;
      try {
        const books = await invoke<Book[]>("tauri_search_books", {
          searchQuery: search_query,
        });
        this.books = books;
        console.log(this.books);
      } catch (error) {
        console.error(error);
      }
    }
  }
}
