import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";
import { SettingsComponent } from "./pages/settings/settings.component";
import { BookDetailsComponent } from "./pages/book-details/book-details.component";
import { AuthorsComponent } from "./pages/authors/authors.component";
import { AuthorDetailsComponent } from "./pages/author-details/author-details.component";
import { LectorsComponent } from "./pages/lectors/lectors.component";


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
    path: 'settings',
    component: SettingsComponent
  }
]