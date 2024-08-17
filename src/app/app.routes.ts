import { Routes } from "@angular/router";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { BooksComponent } from "./pages/books/books.component";
import { SettingsComponent } from "./pages/settings/settings.component";


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