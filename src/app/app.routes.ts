import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";


export const routes: Routes = [
  {
    path: '',
    component: DashboardComponent,
    data: {}
  },
  {
    path: 'books',
    component: BooksComponent
  },
//   {
//     path: 'profile',
//     component: ProfileComponent
//   },
//   {
//     path: 'transactions',
//     component: TransactionsComponent
]