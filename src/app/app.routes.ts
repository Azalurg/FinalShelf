import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";
import { SettingsComponent } from "./pages/settings/settings.component";
import { BookDetailsComponent } from "./pages/book-details/book-details.component";
import { AuthorsComponent } from "./pages/authors/authors.component";
import { AuthorDetailsComponent } from "./pages/author-details/author-details.component";


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
    path: 'settings',
    component: SettingsComponent
  }
]