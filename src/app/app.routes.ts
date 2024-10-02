import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";
import { SettingsComponent } from "./pages/settings/settings.component";
import { BookDetailsComponent } from "./pages/book-details/book-details.component";
import { AuthorsComponent } from "./pages/authors/authors.component";
import { AuthorDetailsComponent } from "./pages/author-details/author-details.component";
import { LectorsComponent } from "./pages/lectors/lectors.component";
import { SearchComponent } from "./pages/search/search.component";
import { LectorsDetailsComponent } from "./pages/lectors-details/lectors-details.component";
import { GenresComponent } from "./pages/genres/genres.component";
import { GenresDetailsComponent } from "./pages/genres-details/genres-details.component";


export const routes: Routes = [
  {
    path: '',
    component: DashboardComponent,
  },
  {
    path: 'books/:id',
    component: BookDetailsComponent,
  },
  {
    path: 'books',
    component: BooksComponent
  },
  {
    path: 'authors',
    component: AuthorsComponent
  },
  {
    path: 'authors/:id',
    component: AuthorDetailsComponent
  },
  {
    path: 'lectors',
    component: LectorsComponent
  },
  {
    path: 'lectors/:id',
    component: LectorsDetailsComponent
  },
  {
    path: 'genres',
    component: GenresComponent
  },
  {
    path: 'genres/:id',
    component: GenresDetailsComponent
  },
  {
    path: 'settings',
    component: SettingsComponent
  },
  {
    path: 'search',
    component: SearchComponent
  }
]