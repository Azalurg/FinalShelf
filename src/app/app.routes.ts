import { Routes } from "@angular/router";
import { BookDetailsPageComponent } from "./features/books/pages/details/details.component";
import { BooksListPageComponent } from "./features/books/pages/list/list.component";
import { AuthorsListPageComponent } from "./features/authors/list/authors.component";
import { AuthorDetailsPageComponent } from "./features/authors/details/author-details.component";
import { LectorsListPageComponent } from "./features/lectors/list/lectors.component";
import { LectorsDetailsPageComponent } from "./features/lectors/details/lectors-details.component";
import { GenresListPageComponent } from "./features/genres/list/genres.component";
import { GenresDetailsPageComponent } from "./features/genres/details/genres-details.component";
import { SettingsPageComponent } from "./features/settings/settings.component";
import { SearchPageComponent } from "./features/search/search.component";
import { DashboardPageComponent } from "./features/dashboard/dashboard.component";
import { ReadBooksListPageComponent } from "./features/books/pages/read-books-list/read-books-list.component";

export const routes: Routes = [
  {
    path: "",
    component: DashboardPageComponent,
  },
  {
    path: "books/:title",
    component: BookDetailsPageComponent,
  },
  {
    path: "books",
    component: BooksListPageComponent,
  },
  {
    path: "read",
    component: ReadBooksListPageComponent,
  },
  {
    path: "authors",
    component: AuthorsListPageComponent,
  },
  {
    path: "authors/:name",
    component: AuthorDetailsPageComponent,
  },
  {
    path: "lectors",
    component: LectorsListPageComponent,
  },
  {
    path: "lectors/:name",
    component: LectorsDetailsPageComponent,
  },
  {
    path: "genres",
    component: GenresListPageComponent,
  },
  {
    path: "genres/:name",
    component: GenresDetailsPageComponent,
  },
  {
    path: "settings",
    component: SettingsPageComponent,
  },
  {
    path: "search",
    component: SearchPageComponent,
  },
];
