import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";
import { SettingsComponent } from "./pages/settings/settings.component";
import { BookDetailsComponent } from "./pages/book-details/book-details.component";


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
    path: 'settings',
    component: SettingsComponent
  }
//   {
//     path: 'profile',
//     component: ProfileComponent
//   },
//   {
//     path: 'transactions',
//     component: TransactionsComponent
]